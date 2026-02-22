// This file contains helper functions 
use subxt::{OnlineClient, PolkadotConfig};
use subxt::ext::scale_value::Composite;
use subxt::utils::AccountId32;
use subxt::ext::futures::StreamExt;

use subxt_signer::dilithium::Keypair as DilithiumKeypair;
use subxt_signer::sr25519::Keypair as Sr25519Keypair;
use subxt_signer::ecdsa::Keypair as EcdsaKeypair;
use subxt_signer::dilithium::SignatureBundle;
use sp_core::hashing::blake2_256;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use subxt::tx::TxStatus;
#[subxt::subxt(runtime_metadata_path = "solochain_metadata.scale")]
pub mod polkadot_testnet {}


// Wrap AccountId32 as MultiAddress::Id(...) for dynamic tx
pub fn dest_multiaddress_id(dest: &AccountId32) -> subxt::dynamic::Value {
    let dest_bytes: &[u8] = dest.as_ref();
    subxt::dynamic::Value::variant(
        "Id",
        Composite::unnamed(vec![subxt::dynamic::Value::from_bytes(dest_bytes.to_vec())]),
    )
}

pub fn sr25519_account_id(sr: &Sr25519Keypair) -> AccountId32 {
    sr.public_key().into()
}

pub fn ecdsa_account_id(ec: &EcdsaKeypair) -> AccountId32 {
    let pk = ec.public_key();
    let pk33: &[u8] = pk.as_ref();
    blake2_256(pk33).into()
}

pub fn dilithium_account_id(dil: &DilithiumKeypair) -> AccountId32 {
    let pk = dil.public_key();
    let pk_bytes: &[u8] = pk.as_ref();
    blake2_256(pk_bytes).into()
}

pub fn summarize_us(mut xs: Vec<u128>) {
    xs.sort_unstable();
    let n = xs.len();
    let p = |q: f64| -> u128 {
        if n == 0 { return 0; }
        let idx = ((q * (n as f64 - 1.0)).round() as usize).min(n - 1);
        xs[idx]
    };
    let sum: u128 = xs.iter().copied().sum();
    let mean = if n == 0 { 0 } else { sum / n as u128 };

    println!("n={} mean_us={} p50_us={} p95_us={} p99_us={}",
        n, mean, p(0.50), p(0.95), p(0.99)
    );
}


pub async fn fund_account(
    api: &OnlineClient<PolkadotConfig>,
    from: &Sr25519Keypair,      //Alice
    to: &AccountId32,           // any scheme’s derived AccountId32
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![
            dest_multiaddress_id(to),
            subxt::dynamic::Value::u128(10_000_000_000_000),
        ],
    );

    let signed = api.tx().create_signed(&tx, from, Default::default()).await?;
    let mut progress = signed.submit_and_watch().await?;

    while let Some(status) = progress.next().await {
       match status? {
            TxStatus::InFinalizedBlock(_) => break,
            TxStatus::Error { message }
            | TxStatus::Invalid { message }
            | TxStatus::Dropped { message } => return Err(message.into()),
            _ => {}
        }
    }
    Ok(())
}

pub async fn free_balance(
    api: &OnlineClient<PolkadotConfig>,
    label: &str,
    account: &AccountId32,
) -> Result<(), Box<dyn std::error::Error>> {
    const PAS_UNITS: u128 = 10_000_000_000;

    let storage = api.storage().at_latest().await?;

    let addr = polkadot_testnet::storage()
        .system()
        .account();

    let account_info = storage
        .fetch(&addr, (account.clone(),))
        .await?;

    let info = account_info.decode()?;

    println!(
        "{} Free Balance: {} ({} PAS)",
        label,
        info.data.free,
        info.data.free as f64 / PAS_UNITS as f64
    );

    Ok(())
}

pub fn csv_open_append_keygen(path: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let exists = Path::new(path).exists();
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    if !exists {
        writeln!(f, "iter,keygen_us")?;
    }
    Ok(f)
}

pub fn csv_open_append_sign(path: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let exists = Path::new(path).exists();
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    if !exists {
        writeln!(f, "iter,sign_us")?;
    }

    Ok(f)
}

pub fn csv_open_append(path: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let exists = Path::new(path).exists();
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    if !exists {
        writeln!(
            f,
            "iter,included_us,finalized_us,all_included_us,all_finalized_us"
        )?;
    }

    Ok(f)
}

pub fn print_crypto_sizes(
    dil: &DilithiumKeypair,
    sr: &Sr25519Keypair,
    ec: &EcdsaKeypair,
    dil_sender: &AccountId32,
    sr_sender: &AccountId32,
    ec_sender: &AccountId32,
) {
    let msg: &[u8] = b"keysize-check";

    let sr_pub = sr.public_key();
    let ec_pub = ec.public_key();
    let dil_pub = dil.public_key();

    let sr_sig = sr.sign(msg);
    let ec_sig = ec.sign(msg);
    let dil_sig: SignatureBundle = dil.sign(msg);

    println!("KEY / SIG SIZE (bytes)");

    println!(
        "Sr25519:  pub={}  sig={}  account_id={}",
        AsRef::<[u8]>::as_ref(&sr_pub).len(),
        AsRef::<[u8]>::as_ref(&sr_sig).len(),
        <AccountId32 as AsRef<[u8]>>::as_ref(sr_sender).len(),
    );

    println!(
        "ECDSA:    pub={}  sig={}  account_id={}",
        AsRef::<[u8]>::as_ref(&ec_pub).len(),
        AsRef::<[u8]>::as_ref(&ec_sig).len(),
        <AccountId32 as AsRef<[u8]>>::as_ref(ec_sender).len(),
    );

    // SignatureBundle is a struct with fixed-size arrays; no AsRef needed.
    println!(
        "Dilithium: pub={}  sig={}  pk_in_sig_bundle={}  account_id={}",
        AsRef::<[u8]>::as_ref(&dil_pub).len(),
        dil_sig.signature.len(),
        dil_sig.public.len(),
        <AccountId32 as AsRef<[u8]>>::as_ref(dil_sender).len(),
    );
}
