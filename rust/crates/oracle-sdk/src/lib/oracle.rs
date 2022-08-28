use aptos_sdk::{
    crypto::_once_cell::sync::Lazy,
    move_types::{identifier::Identifier, language_storage::ModuleId},
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

use crate::lib::model::{Aggregator, TokenDetails};

use super::error::{OracleError, OracleTypedResult};

pub const MODULE_NAME: &str = "tokens";
pub const APTOS_NODE_URL: &str = "https://fullnode.devnet.aptoslabs.com";
// pub const APTOS_FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com";
// pub static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
//     Url::from_str(
//         std::env::var("APTOS_FAUCET_URL")
//             .as_ref()
//             .map(|s| s.as_str())
//             .unwrap_or(APTOS_FAUCET_URL),
//     )
//     .unwrap()
// });

pub static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(APTOS_NODE_URL),
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

#[allow(dead_code)]
impl OracleClient {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }

    pub fn get_module(&self, module_account_address: AccountAddress) -> ModuleId {
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

    pub async fn initialize_aggregator(
        &self,
        module_account_address: AccountAddress,
        account: &mut LocalAccount,
        aggregator_id: u8,
        source_name: &str,
        options: Option<TransactionOptions>,
    ) -> OracleTypedResult<Response<PendingTransaction>> {
        let entry_function = EntryFunction::new(
            self.get_module(module_account_address),
            Identifier::new("initialize_aggregator").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&aggregator_id).unwrap(),
                bcs::to_bytes(&source_name).unwrap(),
            ],
        );

        self.send_transaction(account, entry_function, options)
            .await
            .map_err(|err| {
                OracleError::InstructionExecutionError(format!(
                    "The execution of initialize_aggregator failed : {}",
                    err.to_string()
                ))
            })
    }

    pub async fn initialize_token(
        &self,
        module_account_address: AccountAddress,
        account: &mut LocalAccount,
        token_name: &str,
        token_symbol: &str,
        options: Option<TransactionOptions>,
    ) -> OracleTypedResult<Response<PendingTransaction>> {
        let entry_function = EntryFunction::new(
            self.get_module(module_account_address),
            Identifier::new("initialize_token").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&token_name).unwrap(),
                bcs::to_bytes(&token_symbol).unwrap(),
            ],
        );

        self.send_transaction(account, entry_function, options)
            .await
            .map_err(|err| {
                OracleError::InstructionExecutionError(format!(
                    "The execution of initialize_token failed : {}",
                    err.to_string()
                ))
            })
    }

    pub async fn add_feed(
        &self,
        module_account_address: AccountAddress,
        account: &mut LocalAccount,
        token_symbol: &str,
        price: u128,
        decimals: u8,
        last_update: &str,
        options: Option<TransactionOptions>,
    ) -> OracleTypedResult<Response<PendingTransaction>> {
        let entry_function = EntryFunction::new(
            self.get_module(module_account_address),
            Identifier::new("add_feed").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&token_symbol).unwrap(),
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

    pub async fn get_aggregator_data(
        &self,
        //module_account_address: AccountAddress,
        account: &LocalAccount,
    ) -> OracleTypedResult<()> {
        let resource_resp = self
            .api_client
            .get_resource::<Aggregator>(
                account.address(),
                format!("{}::tokens::Aggregator", account.address().to_hex_literal()).as_str(),
            )
            .await
            .map_err(|err| {
                OracleError::FetchError(format!(
                    "Failed to get_aggregator data {} ",
                    err.to_string()
                ))
            })
            .unwrap();

        let resource = resource_resp.inner();

        //resource.data

        //let result = serde_json::from_value::<Aggregator>(resource.data.clone()).unwrap();

        info!("{:?}", resource.tokens.data);

        Ok(())
    }

    pub async fn get_latest_token_data(
        &self,
        account: &LocalAccount,
        token_symbol: &str,
    ) -> OracleTypedResult<TokenDetails> {
        let resource_resp = self
            .api_client
            .get_resource::<Aggregator>(
                account.address(),
                format!("{}::tokens::Aggregator", account.address().to_hex_literal()).as_str(),
            )
            .await
            .map_err(|err| {
                OracleError::FetchError(format!(
                    "Failed to get_token data {} :{}",
                    token_symbol,
                    err.to_string()
                ))
            })
            .unwrap();

        let resource = resource_resp.inner();

        info!("{:?}", resource);

        let token_symbol_data = resource
            .tokens
            .data
            .iter()
            .filter(|value| value.key == token_symbol)
            .next()
            .unwrap();

        let latest_token_details_data = token_symbol_data.value.token_details_list.last();

        Ok(latest_token_details_data
            .unwrap_or(&Default::default())
            .clone())
    }
}
