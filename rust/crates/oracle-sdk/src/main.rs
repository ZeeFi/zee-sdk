use std::error::Error;

use aptos_sdk::{
    crypto::{ed25519::Ed25519PrivateKey, PrivateKey},
    rest_client::{self, FaucetClient, Transaction},
    types::{
        account_address::AccountAddress, transaction::authenticator::AuthenticationKey,
        LocalAccount,
    },
};
use lib::{
    config::OracleConfig,
    oracle::{self, OracleClient},
};

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let oracle_client = OracleClient::new(rest_client::Client::new(oracle::NODE_URL.clone()));
    let faucet_client = FaucetClient::new(oracle::FAUCET_URL.clone(), oracle::NODE_URL.clone());

    let config: OracleConfig = OracleConfig::read_config(".aptos/config.yaml").unwrap();

    let profiles = config.profiles.unwrap();

    let default_profile = profiles.get("default").unwrap().clone();

    //println!("{}", default_profile.rest_url.clone().unwrap());

    let key = clone(default_profile.private_key.as_ref().unwrap());

    let sender_address = AuthenticationKey::ed25519(&key.public_key()).derived_address();
    let sender_address = AccountAddress::new(*sender_address);

    let sequence_number = oracle_client.get_sequence_number(sender_address).await?;

    let oracle_account = &mut LocalAccount::new(sender_address, key, sequence_number);

    //faucet_client.fund(oracle_account.address(), 5_000).await?;

    let result = oracle_client
        .api_client
        .get_account_balance(oracle_account.address())
        .await?
        .inner()
        .get();

    println!("The balance is {}", result);

    // let pending_transaction = oracle_client
    //     .initialize_oracle(oracle_account, 1, "Eth Oracle", "ETH", None)
    //     .await
    //     .unwrap()
    //     .into_inner();

    let pending_transaction = oracle_client
        .add_feed(oracle_account, 1900000, 3, "20220826".to_string(), None)
        .await
        .unwrap()
        .into_inner();

    let result: Transaction = oracle_client
        .api_client
        .wait_for_transaction(&pending_transaction)
        .await?
        .into_inner();

    println!("The transaction is {}", result.success());

    Ok(())
}

fn clone(key: &Ed25519PrivateKey) -> Ed25519PrivateKey {
    let serialized: &[u8] = &(key.to_bytes());
    Ed25519PrivateKey::try_from(serialized).unwrap()
}
