use bip32::{DerivationPath, Language};
use cosmrs::proto::tendermint::Error;
use serde::Deserialize;

// derivation path for cosmos wallets is defined here:
// https://github.com/satoshilabs/slips/blob/master/slip-0044.md
const HD_PATH: &str = "m/44'/118'/0'/0/0";
// put chain you are interested in here
// osmo is default
const BECH_PREFIX: &str = "osmo";

fn main() -> Result<(), Error> {
    // put mnemonic phrase here
    let mnemonic_str = "";
    let mnemonic = bip32::Mnemonic::new(mnemonic_str, Language::English).unwrap();
    let seed = mnemonic.to_seed("");
    let private_key = cosmrs::crypto::secp256k1::SigningKey::derive_from_path(
        &seed,
        &HD_PATH.parse::<DerivationPath>().unwrap(),
    )
    .unwrap();

    let public_key = private_key.public_key();

    let public_key_str = public_key.to_string();
    let parsed = serde_json::from_str(&public_key_str);
    if let Ok(parsed_data) = parsed {
        let data: Data = serde_json::from_value(parsed_data).unwrap();
        println!("public key: {}", data.key);
    } else {
        println!("Error parsing JSON string.");
    }

    let account_id: cosmrs::AccountId = public_key.account_id(BECH_PREFIX).unwrap();
    println!("address: {account_id}");

    Ok(())
}

#[derive(Deserialize)]
struct Data {
    key: String,
}
