use aptos_sdk::{
    crypto::_once_cell::sync::Lazy,
    move_types::{account_address, identifier::Identifier, language_storage::ModuleId},
    rest_client::{Client as ApiClient, PendingTransaction, Response},
    transaction_builder::TransactionBuilder,
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, TransactionPayload},
        LocalAccount,
    },
};
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

use super::error::{OracleError, OracleTypedResult};

pub const MODULE_ID: &str = "0xe96492a1b4527f7ea41dafe83584103e05f3d557ad751a7b8c35b87815001b2b"; //::tokens
pub const MODULE_NAME: &str = "tokens";
pub const APTOS_NODE_URL: &str = "https://fullnode.devnet.aptoslabs.com";
pub const APTOS_FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com";

pub static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(APTOS_NODE_URL),
    )
    .unwrap()
});

pub static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_FAUCET_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(APTOS_FAUCET_URL),
    )
    .unwrap()
});

pub struct TransactionOptions {
    pub max_gas_amount: u64,
    pub gas_unit_price: u64,
    /// This is the number of seconds from now you're willing to wait for the
    /// transaction to be committed.
    pub timeout_secs: u64,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            max_gas_amount: 5_000,
            gas_unit_price: 1,
            timeout_secs: 10,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OracleClient {
    pub api_client: ApiClient,
}

impl OracleClient {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }

    pub fn get_module_id() -> ModuleId {
        let module_account_address =
            account_address::AccountAddress::from_hex_literal(MODULE_ID).unwrap();

        let module_name = Identifier::new(MODULE_NAME).unwrap();
        ModuleId::new(module_account_address, module_name)
    }

    pub async fn send_transaction(
        &self,
        account: &mut LocalAccount,
        entry_function: EntryFunction,
        options: Option<TransactionOptions>,
    ) -> OracleTypedResult<Response<PendingTransaction>> {
        let options = options.unwrap_or_default();

        let chain_id = self
            .api_client
            .get_ledger_information()
            .await
            .unwrap()
            .state()
            .chain_id;

        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(entry_function),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + options.timeout_secs,
            ChainId::new(chain_id),
        )
        .sender(account.address())
        .sequence_number(account.sequence_number())
        .max_gas_amount(options.max_gas_amount)
        .gas_unit_price(options.gas_unit_price);

        let signed_txn = account.sign_with_transaction_builder(transaction_builder);

        let pending_transaction = self.api_client.submit(&signed_txn).await;

        pending_transaction.map_err(|err| {
            OracleError::TransactionError(format!("Sending transaction failed {}", err.to_string()))
        })
    }

    pub async fn add_feed(
        &self,
        account: &mut LocalAccount,
        price: u128,
        decimals: u8,
        last_update: String,
        options: Option<TransactionOptions>,
    ) -> OracleTypedResult<Response<PendingTransaction>> {
        let entry_function = EntryFunction::new(
            OracleClient::get_module_id(),
            Identifier::new("add_feed").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&price).unwrap(),
                bcs::to_bytes(&decimals).unwrap(),
                bcs::to_bytes(&last_update).unwrap(),
            ],
        );

        self.send_transaction(account, entry_function, options)
            .await
            .map_err(|err| {
                OracleError::InstructionExecutionError(format!(
                    "The execution of add_feed failed : {}",
                    err.to_string()
                ))
            })
    }

    pub async fn initialize_oracle(
        &self,
        account: &mut LocalAccount,
        version: u8,
        oracle_name: &str,
        oracle_symbol: &str,
        options: Option<TransactionOptions>,
    ) -> OracleTypedResult<Response<PendingTransaction>> {
        let entry_function = EntryFunction::new(
            OracleClient::get_module_id(),
            Identifier::new("initialize").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&version).unwrap(),
                bcs::to_bytes(&oracle_name).unwrap(),
                bcs::to_bytes(&oracle_symbol).unwrap(),
            ],
        );

        self.send_transaction(account, entry_function, options)
            .await
            .map_err(|err| {
                OracleError::InstructionExecutionError(format!(
                    "The execution of add_feed failed : {}",
                    err.to_string()
                ))
            })
    }

    /// Retrieves sequence number from the rest client
    pub async fn get_sequence_number(&self, address: AccountAddress) -> OracleTypedResult<u64> {
        let account_response = self.api_client.get_account(address).await.map_err(|err| {
            OracleError::UnexpectedError(format!("Fetch error from aptos :  {}", err.to_string()))
        })?;
        let account = account_response.inner();
        Ok(account.sequence_number)
    }
}
