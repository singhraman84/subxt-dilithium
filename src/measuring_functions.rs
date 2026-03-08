// This file contains functions to measure metrics for benchmarking
use std::{str::FromStr, time::Instant};

use subxt::{OnlineClient, PolkadotConfig};
use subxt::ext::scale_value::Composite;
use subxt::utils::AccountId32;
use subxt::ext::futures::{stream, StreamExt};
use std::sync::Arc;
use subxt::tx::Signer as SubxtSigner;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use subxt::tx::TxStatus;
use subxt::backend::rpc::RpcClient;

use crate::helpers::*;


// This function measures submit → in-block / finalized 
// = How long it takes for a real signed transfer to go from submission 
// to being executed in a block and then finalized on the ledger.
pub async fn measure_ledger_latency<S>(
    api: &OnlineClient<PolkadotConfig>,
    signer: &S,
    sender: &AccountId32,
    dest: &AccountId32,
    amount: u128,
    n: usize,
    experiment_file: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: SubxtSigner<PolkadotConfig>,
{
    println!("START experiment={}", experiment_file);

    let mut csv = csv_open_append(experiment_file)?;

    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![dest_multiaddress_id(dest), subxt::dynamic::Value::u128(amount)],
    );

    let mut included_samples = Vec::with_capacity(n);
    let mut finalized_samples = Vec::with_capacity(n);
    let mut all_included_samples = Vec::with_capacity(n);
    let mut all_finalized_samples = Vec::with_capacity(n);

    for i in 0..n {
        println!("iter {}/{}", i + 1, n);

        // First timer => true end-to-end (includes signing)
        let t_all = Instant::now();

        // Signing (contributes to all)
        let signed = api.tx().create_signed(&tx, signer, Default::default()).await?;

        // Second timer => submit -> included/finalized
        let t_submit = Instant::now();
        let mut progress = signed.submit_and_watch().await?;

        let mut inc_us = None;
        let mut fin_us = None;
        let mut all_inc_us = None;
        let mut all_fin_us = None;

        while let Some(status) = progress.next().await {
            match status? {
                TxStatus::InBestBlock(in_block) => {
                    if inc_us.is_none() {
                        let submit_us = t_submit.elapsed().as_micros() as u128;
                        let all_us = t_all.elapsed().as_micros() as u128;

                        inc_us = Some(submit_us);
                        all_inc_us = Some(all_us);

                        included_samples.push(submit_us);
                        all_included_samples.push(all_us);

                        println!(
                            "  included_us={} all_included_us={} block_hash={:?}",
                            submit_us,
                            all_us,
                            in_block.block_hash()
                        );
                    }
                }

                TxStatus::InFinalizedBlock(in_block) => {
                    let submit_us = t_submit.elapsed().as_micros() as u128;
                    let all_us = t_all.elapsed().as_micros() as u128;

                    // if we never observed InBestBlock, treat finalized as "included" too
                    if inc_us.is_none() {
                        inc_us = Some(submit_us);
                        all_inc_us = Some(all_us);

                        included_samples.push(submit_us);
                        all_included_samples.push(all_us);

                        println!(
                            "  (no InBestBlock seen) setting included_us={} all_included_us={} from finalized",
                            submit_us, all_us
                        );
                    }

                    fin_us = Some(submit_us);
                    all_fin_us = Some(all_us);

                    finalized_samples.push(submit_us);
                    all_finalized_samples.push(all_us);

                    println!(
                        "  finalized_us={} all_finalized_us={} block_hash={:?}",
                        submit_us,
                        all_us,
                        in_block.block_hash()
                    );
                    break;
                }

                TxStatus::Error { message }
                | TxStatus::Invalid { message }
                | TxStatus::Dropped { message } => return Err(message.into()),
                _ => {}
            }
        }

        writeln!(
            csv,
            "{},{},{},{},{}",
            i,
            inc_us.unwrap_or(0),
            fin_us.unwrap_or(0),
            all_inc_us.unwrap_or(0),
            all_fin_us.unwrap_or(0),
        )?;
    }
    Ok(())
}

// This function measures how long it takes to sign a transaction
pub async fn measure_signing_time<S>(
    api: &OnlineClient<PolkadotConfig>,
    signer: &S,
    sender: &AccountId32,
    dest: &AccountId32,
    amount: u128,
    iters: usize,
    experiment_file: &str,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: SubxtSigner<PolkadotConfig>,
{
    println!("START signing experiment={}", label);
    let mut csv = csv_open_append_sign(experiment_file)?;

    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![
            dest_multiaddress_id(dest),
            subxt::dynamic::Value::u128(amount),
        ],
    );

    // Fetch nonce once (reduces non-signing noise, and allows measuring signing in isolation)
    let base_nonce: u64 = account_nonce(api, sender).await? as u64;

    // Helper to build params with explicit nonce 
    let params_with_nonce = |nonce: u64| {
        subxt::config::DefaultExtrinsicParamsBuilder::<PolkadotConfig>::default()
            .nonce(nonce)
            .build()
    };

    // cold (usually first signing is slower due to caches, JIT, etc.)
    let t0 = Instant::now();
    let signed = api.tx().create_signed(&tx, signer, params_with_nonce(base_nonce)).await?;
    println!("{}_sign_cold_us={}", label, t0.elapsed().as_micros());

    // warm
    let mut samples: Vec<u128> = Vec::with_capacity(iters);
    let t0 = Instant::now();

    for i in 0..iters {
        let t_iter = Instant::now();

        let signed = api
            .tx()
            .create_signed(&tx, signer, params_with_nonce(base_nonce + i as u64 + 1))
            .await?;
        std::hint::black_box(signed);

        let us = t_iter.elapsed().as_micros() as u128;
        samples.push(us);
        writeln!(csv, "{},{}", i, us)?;

        if (i + 1) % 1000 == 0 {
            println!("{} iter {}/{}", label, i + 1, iters);
        }
    }
    Ok(())
}


// This function measures how long it takes to create a keypair
pub fn measure_keygen_seed_to_keypair<F>(
    label: &str,
    iters: usize,
    file: &str,
    mut gen: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(usize) -> Result<(), Box<dyn std::error::Error>>,
{
    println!("START keygen {} iters={} file={}", label, iters, file);

    let mut csv = csv_open_append_keygen(file)?;
    let mut samples: Vec<u128> = Vec::with_capacity(iters);

    for i in 0..iters {
        let t0 = Instant::now();
        gen(i)?; 
        let us = t0.elapsed().as_micros() as u128;

        samples.push(us);
        writeln!(csv, "{},{}", i, us)?;

        if (i + 1) % 1000 == 0 {
            println!("{} iter {}/{}", label, i + 1, iters);
        }
    }
    Ok(())
}


pub async fn measure_extrinsic_size_and_weight<S>(
    api: &OnlineClient<PolkadotConfig>,
    rpc_client: &RpcClient,
    signer: &S,
    sender: &AccountId32,
    dest: &AccountId32,
    amount: u128,
    label: &str,
    iters: usize,
    out_csv: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: SubxtSigner<PolkadotConfig>,
{
    println!(
        "START extrinsic size/weight experiment={} iters={} file={}",
        label, iters, out_csv
    );

    let exists = Path::new(out_csv).exists();
    let mut f = OpenOptions::new().create(true).append(true).open(out_csv)?;
    if !exists {
        writeln!(f, "iter,tx_type,xt_bytes,xt_ref_time")?;
    }

    let base_nonce = account_nonce(api, sender).await? as u64;

    /* Balances::transfer_allow_death */
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![
            dest_multiaddress_id(dest),
            subxt::dynamic::Value::u128(amount),
        ],
    );

    for i in 0..iters {

        let params = subxt::config::DefaultExtrinsicParamsBuilder::<PolkadotConfig>::default()
            .nonce(base_nonce + i as u64)
            .build();

        let signed = api.tx().create_signed(&tx, signer, params).await?;
        let encoded = signed.encoded();

        let xt_bytes = encoded.len();
        let xt_ref_time = payment_query_info_ref_time(rpc_client, &encoded).await?;

        writeln!(f, "{},transfer_allow_death,{},{}", i, xt_bytes, xt_ref_time)?;

        println!(
            "[{}][transfer_allow_death] iter={} xt_bytes={} xt_ref_time={}",
            label, i, xt_bytes, xt_ref_time
        );
    }

    /* Balances::transfer_keep_alive */
    /*
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_keep_alive",
        vec![
            dest_multiaddress_id(dest),
            subxt::dynamic::Value::u128(amount),
        ],
    );

    for i in 0..iters {

        let params = subxt::config::DefaultExtrinsicParamsBuilder::<PolkadotConfig>::default()
            .nonce(base_nonce + i as u64)
            .build();

        let signed = api.tx().create_signed(&tx, signer, params).await?;
        let encoded = signed.encoded();

        let xt_bytes = encoded.len();
        let xt_ref_time = payment_query_info_ref_time(rpc_client, &encoded).await?;

        writeln!(f, "{},transfer_keep_alive,{},{}", i, xt_bytes, xt_ref_time)?;

        println!(
            "[{}][transfer_keep_alive] iter={} xt_bytes={} xt_ref_time={}",
            label, i, xt_bytes, xt_ref_time
        );
    }
    */

    /* TRANSACTION 3: System::remark */
    /*
    let tx = subxt::dynamic::tx(
        "System",
        "remark",
        vec![
            subxt::dynamic::Value::from_bytes(b"benchmark")
        ],
    );

    for i in 0..iters {

        let params = subxt::config::DefaultExtrinsicParamsBuilder::<PolkadotConfig>::default()
            .nonce(base_nonce + i as u64)
            .build();

        let signed = api.tx().create_signed(&tx, signer, params).await?;
        let encoded = signed.encoded();

        let xt_bytes = encoded.len();
        let xt_ref_time = payment_query_info_ref_time(rpc_client, &encoded).await?;

        writeln!(f, "{},remark,{},{}", i, xt_bytes, xt_ref_time)?;

        println!(
            "[{}][remark] iter={} xt_bytes={} xt_ref_time={}",
            label, i, xt_bytes, xt_ref_time
        );
    }
    */

    Ok(())
}