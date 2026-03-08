use std::str::FromStr;

use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;

use subxt_signer::dilithium::Keypair as DilithiumKeypair;
use subxt_signer::ecdsa::Keypair as EcdsaKeypair;
use subxt_signer::sr25519::Keypair as Sr25519Keypair;
use subxt_signer::SecretUri;

use crate::helpers::*;

async fn send_transfer<Signer>(
    api: &OnlineClient<PolkadotConfig>,
    signer: &Signer,
    dest: &AccountId32,
    amount: u128,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    Signer: subxt::tx::Signer<PolkadotConfig>,
{
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer_allow_death",
        vec![
            dest_multiaddress_id(dest),
            subxt::dynamic::Value::u128(amount),
        ],
    );

    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&tx, signer)
        .await?
        .wait_for_finalized_success()
        .await?;

    println!(
        "[{}] transfer finalized, extrinsic hash: {:?}",
        label,
        events.extrinsic_hash()
    );

    Ok(())
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:9944";
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

    let seed32: [u8; 32] = *b"12345678901234567890123456789012";

    let sr = Sr25519Keypair::from_secret_key(seed32)?;
    let ec = EcdsaKeypair::from_secret_key(seed32)?;
    /* This code remains unchanged regardless of which branch of Dilithium you want to test.
        You can choose the Dilithium version by changing the branch of https://github.com/bsaviozz/subxt and
        https://github.com/bsaviozz/polkadot-sdk.git  */
    let dil = DilithiumKeypair::from_seed(seed32);

    let sr_sender = sr25519_account_id(&sr);
    let ec_sender = ecdsa_account_id(&ec);
    let dil_sender = dilithium_account_id(&dil);

    println!("Sr25519 sender:   {}", sr_sender);
    println!("ECDSA sender:     {}", ec_sender);
    println!("Dilithium sender: {}", dil_sender);

    let alice = Sr25519Keypair::from_uri(&SecretUri::from_str("//Alice")?)?;
    fund_account(&api, &alice, &sr_sender).await?;
    fund_account(&api, &alice, &ec_sender).await?;
    fund_account(&api, &alice, &dil_sender).await?;

    let bob = Sr25519Keypair::from_uri(&SecretUri::from_str("//Bob")?)?;
    let bob_account: AccountId32 = bob.public_key().into();

    let amount: u128 = 10_000_000;

    send_transfer(&api, &sr, &bob_account, amount, "sr25519").await?;
    send_transfer(&api, &ec, &bob_account, amount, "ecdsa").await?;
    // Change the label based on the Dilithium version, the rest of the code remains unchanged
    send_transfer(&api, &dil, &bob_account, amount, "ml_dsa_87").await?;

    Ok(())
}