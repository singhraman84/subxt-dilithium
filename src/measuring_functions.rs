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

    free_balance(api, "sender BEFORE", sender).await?;
    free_balance(api, "dest   BEFORE", dest).await?;

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

        // True end-to-end starts BEFORE signing
        let t_all = Instant::now();

        // Signing (not written separately, only contributes to all_*)
        let signed = api.tx().create_signed(&tx, signer, Default::default()).await?;

        // submit -> included/finalized
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

        // CSV: exactly the requested columns
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

    println!("submit→included summary:");
    summarize_us(included_samples);

    println!("submit→finalized summary:");
    summarize_us(finalized_samples);

    println!("all→included (sign→included) summary:");
    summarize_us(all_included_samples);

    println!("all→finalized (sign→finalized) summary:");
    summarize_us(all_finalized_samples);

    free_balance(api, "sender AFTER ", sender).await?;
    free_balance(api, "dest   AFTER ", dest).await?;

    Ok(())
}

// This function measures how long it takes to sign a transaction
pub async fn measure_signing_time<S>(
    api: &OnlineClient<PolkadotConfig>,
    signer: &S,
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

    /* ---------- cold ---------- */
    let t0 = Instant::now();
    let signed = api.tx().create_signed(&tx, signer, Default::default()).await?;
    std::hint::black_box(signed);
    println!("{}_sign_cold_us={}", label, t0.elapsed().as_micros());

    /* ---------- warm ---------- */
    let mut samples: Vec<u128> = Vec::with_capacity(iters);
    let t0 = Instant::now();

    for i in 0..iters {
        let t_iter = Instant::now();
        let signed = api.tx().create_signed(&tx, signer, Default::default()).await?;
        std::hint::black_box(signed);

        let us = t_iter.elapsed().as_micros() as u128;
        samples.push(us);
        writeln!(csv, "{},{}", i, us)?;

        if (i + 1) % 1000 == 0 {
            println!("{} iter {}/{}", label, i + 1, iters);
        }
    }

    println!(
        "{}_sign_warm_total_us={}",
        label,
        t0.elapsed().as_micros()
    );

    println!("{}_sign summary:", label);
    summarize_us(samples);

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
    F: FnMut() -> Result<(), Box<dyn std::error::Error>>,
{
    println!("START keygen {} iters={} file={}", label, iters, file);

    let mut csv = csv_open_append_keygen(file)?;
    let mut samples: Vec<u128> = Vec::with_capacity(iters);

    for i in 0..iters {
        let t0 = Instant::now();
        gen()?; // does black_box internally
        let us = t0.elapsed().as_micros() as u128;

        samples.push(us);
        writeln!(csv, "{},{}", i, us)?;

        if (i + 1) % 1000 == 0 {
            println!("{} iter {}/{}", label, i + 1, iters);
        }
    }

    println!("[{}] keygen summary:", label);
    summarize_us(samples);
    Ok(())
}