extern crate dotenv;
use aptos_sdk::types::LocalAccount;
use dotenv::dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::{
    crypto::PrivateKey,
    rest_client::{
        self,
        //FaucetClient,
        Transaction,
    },
    types::transaction::authenticator::AuthenticationKey,
};
use lib::{
    config::OracleConfig,
    oracle::{self, OracleClient},
};
use std::collections::HashMap;
use std::error::Error;

use crate::lib::config::ProfileConfig;

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let oracle_client = OracleClient::new(rest_client::Client::new(oracle::NODE_URL.clone()));
    //let oracle_config = OracleConfig::new(None);

    let str = "---\nprofiles:\n  default:\n    private_key: \"0xa25bb9a6d43201304f73c70414323eddf7f9ed3eadd4fa6e97bc42f7a904a2c0\"\n    public_key: \"0xfb64b4a8a01b816e7f9dbad8d548112af3407b0058c87f6bce3b2fa3414c89d7\"\n    account: 262715fea1109c185b8f654818478caf057fe22445e9c986bd7ee295702f0e4f\n    rest_url: \"https://fullnode.devnet.aptoslabs.com/v1\"\n    faucet_url: \"https://faucet.devnet.aptoslabs.com/\"\n";

    // let result = OracleConfig::from_yaml::<Option<HashMap<String, ProfileConfig>>>(str)?
    //     .unwrap()
    //     .get("default")
    //     .clone()
    //     .unwrap();

    println!(
        "{:?}",
        OracleConfig::from_yaml::<Option<HashMap<String, ProfileConfig>>>(str)?.unwrap()
    );

    /*  let config: OracleConfig = OracleConfig::read_config(".aptos/config.yaml").unwrap();

    let profiles = config.profiles.unwrap();

    let default_profile = profiles.get("default").unwrap().clone();

    //println!("{}", default_profile.rest_url.clone().unwrap());

    let key = OracleConfig::clone(default_profile.private_key.as_ref().unwrap());

    let sender_address = AuthenticationKey::ed25519(&key.public_key()).derived_address();
    let sender_address = AccountAddress::new(*sender_address);

    let sequence_number = OracleConfig::get_sequence_number(&oracle_client, sender_address).await?;

    let default_account = &mut LocalAccount::new(sender_address, key, sequence_number);

    // let default_account = &mut OracleConfig::load_default_account(&oracle_client)
    //     .await
    //     .unwrap();

    //let faucet_client = FaucetClient::new(oracle::FAUCET_URL.clone(), oracle::NODE_URL.clone());
    //faucet_client.fund(oracle_account.address(), 5_000).await?;

    let result = oracle_client
        .api_client
        .get_account_balance(default_account.address())
        .await?
        .inner()
        .get();

    println!("The balance is {}", result);

    let pending_transaction = oracle_client
        .initialize_aggregator(
            default_account.address(),
            default_account,
            "Coinbase Agggregator",
            None,
        )
        .await
        .unwrap()
        .into_inner();

    // let pending_transaction = oracle_client
    //     .add_feed(
    //         default_account.address(),
    //         default_account,
    //         "ETH",
    //         1900000,
    //         3,
    //         "20220826",
    //         None,
    //     )
    //     .await
    //     .unwrap()
    //     .into_inner();

    let result: Transaction = oracle_client
        .api_client
        .wait_for_transaction(&pending_transaction)
        .await?
        .into_inner();

    println!("The transaction is {}", result.success());*/

    Ok(())
}
