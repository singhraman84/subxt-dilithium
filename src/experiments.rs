use std::str::FromStr;

use subxt::{OnlineClient, PolkadotConfig};
use subxt::backend::rpc::RpcClient;
use subxt::utils::AccountId32;

use subxt_signer::dilithium::Keypair as DilithiumKeypair;
use subxt_signer::sr25519::Keypair as Sr25519Keypair;
use subxt_signer::ecdsa::Keypair as EcdsaKeypair;
use subxt_signer::SecretUri;

use crate::helpers::*;
use crate::measuring_functions::*;

pub async fn run_experiment() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:9944";

    let rpc_client = RpcClient::from_url(url).await?;
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;

    let seed32: [u8; 32] = *b"12345678901234567890123456789012";

    /* This code remains unchanged regardless of which branch of Dilithium you want to test.
        You can choose the Dilithium version by changing the branch of https://github.com/bsaviozz/subxt and
        https://github.com/bsaviozz/polkadot-sdk.git  */
    let dil = DilithiumKeypair::from_seed(seed32);
    let dil_sender = dilithium_account_id(&dil);

    let sr = Sr25519Keypair::from_secret_key(seed32)?;
    let sr_sender = sr25519_account_id(&sr);

    let ec = EcdsaKeypair::from_secret_key(seed32)?;
    let ec_sender = ecdsa_account_id(&ec);

    print_crypto_sizes(&dil, &sr, &ec, &dil_sender, &sr_sender, &ec_sender);

    println!("Dilithium: {}", dil_sender);
    println!("Sr25519:   {}", sr_sender);
    println!("ECDSA:     {}", ec_sender);

    let alice = Sr25519Keypair::from_uri(&SecretUri::from_str("//Alice")?)?;
    let alice_account: AccountId32 = alice.public_key().into();

    let amount: u128 = 10_000_000;

    println!("funding sr...");
    fund_account(&api, &alice, &sr_sender).await?;
    println!("funding sr OK");

    println!("funding ecdsa...");
    fund_account(&api, &alice, &ec_sender).await?;
    println!("funding ecdsa OK");

    println!("funding dilithium...");
    fund_account(&api, &alice, &dil_sender).await?;
    println!("funding dilithium OK");

    /* Changing Dilithium versions:
    - Dilithium 2 (ml-dsa-44): change "https://github.com/bsaviozz/polkadot-sdk.git" branch to "dilithium-ml-dsa-44" 
    // + change "https://github.com/bsaviozz/subxt" branch to "dilithium-ml-dsa-44" 
    // + change labels in the code below to "ml_dsa_44" 

    - Dilithium 3 (ml-dsa-65): change "https://github.com/bsaviozz/polkadot-sdk.git" branch to "dilithium-ml-dsa-65" 
    // + change "https://github.com/bsaviozz/subxt" branch to "dilithium-ml-dsa-65" 
    // + change labels in the code below to "ml_dsa_65" 

    - Dilithium 5 (ml-dsa-87): change "https://github.com/bsaviozz/polkadot-sdk.git" branch to "dilithium" 
    // + change "https://github.com/bsaviozz/subxt" branch to "master" 
    // + change labels in the code below to "ml_dsa_87" */


    /* TRANSACTION LIFECYCLE LATENCY */ 
    // If you are using solochain with instant finality, use measure_transaction_latency_instant instead of measure_ledger_latency, the rest of the code remains unchanged
    measure_ledger_latency(&api, &dil, &dil_sender, &alice_account, amount, 100, "ml_dsa_87_measure_transaction_latency.csv",).await?;

    measure_ledger_latency(&api, &sr, &sr_sender, &alice_account, amount, 100, "sr25519_measure_transaction_latency.csv",).await?;

    measure_ledger_latency(&api, &ec, &ec_sender, &alice_account, amount, 100, "ecdsa_measure_transaction_latency.csv",).await?;

    /* KEY GENERATION LATENCY */
    measure_keygen_seed_to_keypair("Dilithium", 10_000, "ml_dsa_87_keygen.csv", |i| {
        let mut seed = seed32;
        seed[0] ^= (i as u8);
        let kp = DilithiumKeypair::from_seed(seed);
        std::hint::black_box(kp);
        Ok(())
    })?; 
    
    measure_keygen_seed_to_keypair("Sr25519", 10_000, "sr25519_keygen.csv", |i| {
        let mut seed = seed32;
        seed[0] ^= (i as u8);
        let kp = Sr25519Keypair::from_secret_key(seed)?;
        std::hint::black_box(kp);
        Ok(())
    })?;

    measure_keygen_seed_to_keypair("ECDSA", 10_000, "ecdsa_keygen.csv", |i| {
        let mut seed = seed32;
        seed[0] ^= (i as u8);
        let kp = EcdsaKeypair::from_secret_key(seed)?;
        std::hint::black_box(kp);
        Ok(())
    })?; 

    /* TRANSACTION SIGNING LATENCY */ 
    measure_signing_time(&api, &sr, &sr_sender, &alice_account, amount, 10_000, "sr25519_sign_latency.csv", "sr25519").await?;

    measure_signing_time(&api, &ec, &ec_sender, &alice_account, amount, 10_000, "ecdsa_sign_latency.csv", "ecdsa").await?;

    measure_signing_time(&api, &dil, &dil_sender, &alice_account, amount, 10_000, "ml_dsa_87_sign_latency.csv", "dilithium").await?;

    /* TRANSACTION SIZE AND WEIGHT */ 
    measure_extrinsic_size_and_weight(
        &api, &rpc_client, &sr, &sr_sender, &alice_account, amount,
        "sr25519", 1000, "sr25519_xt_size_weight.csv"
    ).await?;

    measure_extrinsic_size_and_weight(
        &api, &rpc_client, &ec, &ec_sender, &alice_account, amount,
        "ecdsa", 1000, "ecdsa_xt_size_weight.csv"
    ).await?;

    measure_extrinsic_size_and_weight(
        &api, &rpc_client, &dil, &dil_sender, &alice_account, amount,
        "ml_dsa_87", 1000, "ml_dsa_87_xt_size_weight.csv"
    ).await?;

    Ok(())
}