use lib::{
    common::{Account, FaucetClient, DEVNET_URL, FAUCET_URL},
    oracle::OracleClient,
};

mod lib;

fn main() {
    let oracle_client = OracleClient::new(DEVNET_URL.to_string());
    let faucet_client =
        FaucetClient::new(FAUCET_URL.to_string(), oracle_client.rest_client.clone());

    let oracler = &mut Account::new(None);

    faucet_client.fund_account(&oracler.auth_key(), 5_000);

    oracle_client
        .add_feed(oracler, 190000, 2, "20220202")
        .unwrap();
}
