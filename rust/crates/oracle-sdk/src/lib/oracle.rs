use std::error::Error;

use super::common::{Account, RestClient};

pub const MODULE_NAME: &str =
    "0xe96492a1b4527f7ea41dafe83584103e05f3d557ad751a7b8c35b87815001b2b::tokens";
pub struct OracleClient {
    pub rest_client: RestClient,
}

impl OracleClient {
    pub fn new(url: String) -> Self {
        Self {
            rest_client: RestClient::new(url),
        }
    }

    pub fn add_feed(
        &self,
        account_from: &mut Account,
        price: u128,
        decimals: u8,
        last_update: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!(" The account address is {}", account_from.address());

        let last_update_hex = hex::encode(last_update);

        let payload = serde_json::json!({
            "type" : "entry_function_payload",
            "function" : format!("{}::add_feed_general", MODULE_NAME),
            "type_arguments" : [],
            "arguments" :[price.to_string(), decimals, last_update_hex]
        });

        self.rest_client
            .execution_transaction_with_payload(account_from, payload);

        Ok(())
    }
}
