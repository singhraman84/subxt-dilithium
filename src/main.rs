mod helpers;
mod measuring_functions;

use helpers::*;
use measuring_functions::*;

use std::str::FromStr;

use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;

use subxt_signer::dilithium::Keypair as DilithiumKeypair;
use subxt_signer::sr25519::Keypair as Sr25519Keypair;
use subxt_signer::ecdsa::Keypair as EcdsaKeypair;
use subxt_signer::SecretUri;
use sp_core::crypto::{AccountId32 as SpAccountId32, Ss58Codec};

use crate::helpers::polkadot_testnet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to local dev node
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9944").await?;

    let seed32: [u8; 32] = *b"12345678901234567890123456789012";

    // Dilithium (your custom signer)
    let dil = DilithiumKeypair::from_seed(seed32);
    let dil_sender = dilithium_account_id(&dil);

    // Sr25519: seed-based via from_secret_key (32 bytes)
    let sr = Sr25519Keypair::from_secret_key(seed32)?;
    let sr_sender  = sr25519_account_id(&sr);

    // ECDSA: seed-based via from_secret_key (32 bytes)
    let ec = EcdsaKeypair::from_secret_key(seed32)?;
    let ec_sender  = ecdsa_account_id(&ec);

    print_crypto_sizes(&dil, &sr, &ec, &dil_sender, &sr_sender, &ec_sender);

    println!("Dilithium: {}", dil_sender);
    println!("Sr25519:   {}", sr_sender);
    println!("ECDSA:     {}", ec_sender);

    // Recipient (Alice)
    let alice = Sr25519Keypair::from_uri(&SecretUri::from_str("//Alice")?)?;
    let alice_account: AccountId32 = alice.public_key().into();

    let amount: u128 = 10_000_000;

    // Fund ecdsa & sr25519 accounts
    println!("funding sr...");
    fund_account(&api, &alice, &sr_sender).await?;
    println!("funding sr OK");

    println!("funding ecdsa...");
    fund_account(&api, &alice, &ec_sender).await?;
    println!("funding ecdsa OK");

    println!("funding dilithium...");
    fund_account(&api, &alice, &dil_sender).await?;
    println!("funding dilithium OK");

    // Build transaction
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![dest_multiaddress_id(&alice_account), subxt::dynamic::Value::u128(amount)],
    );
    
    /* ------- METRIC 1: LATENCY LEDGER REGISTERING TRANSACTION ------- */ 
    measure_ledger_latency(&api, &dil, &dil_sender, &alice_account, amount, 100, "ml_dsa_87_measure_end_to_end.csv",).await?; 

    measure_ledger_latency(&api, &sr, &sr_sender, &alice_account, amount, 100, "sr25519_measure_end_to_end.csv",).await?;

    measure_ledger_latency(&api, &ec, &ec_sender, &alice_account, amount, 100, "ecdsa_measure_end_to_end.csv",).await?;

    /* ------- METRIC 2: LATENCY KEY GENERATION ------- */
    measure_keygen_seed_to_keypair("Sr25519", 10_000, "sr25519_keygen.csv", || {
        let kp = Sr25519Keypair::from_secret_key(seed32)?;
        std::hint::black_box(kp);
        Ok(())
    })?;

    measure_keygen_seed_to_keypair("ECDSA", 10_000, "ecdsa_keygen.csv", || {
        let kp = EcdsaKeypair::from_secret_key(seed32)?;
        std::hint::black_box(kp);
        Ok(())
    })?;

    measure_keygen_seed_to_keypair("Dilithium", 10_000, "ml_dsa_87_keygen.csv", || {
        let kp = DilithiumKeypair::from_seed(seed32);
        std::hint::black_box(kp);
        Ok(())
    })?;

    /* ------- METRIC 3: LATENCY TRANSACTION SIGNING ------- */
    measure_signing_time(&api, &sr, &alice_account, amount, 10_000, "sr25519_sign_transfer.csv", "sr25519",).await?;

    measure_signing_time(&api, &ec, &alice_account, amount, 10_000, "ecdsa_sign_transfer.csv", "ecdsa",).await?;

    measure_signing_time(&api, &dil, &alice_account, amount, 10_000, "ml_dsa_87_sign_transfer.csv", "dilithium",).await?;
    
    /* ------- METRIC 4: LEDGER THROUGHPUT ------- */   
    Ok(())
}


