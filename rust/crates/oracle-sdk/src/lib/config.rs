use aptos_sdk::{
    crypto::{
        ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
        PrivateKey,
    },
    types::{
        account_address::AccountAddress, transaction::authenticator::AuthenticationKey,
        LocalAccount,
    },
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml;
use std::{collections::HashMap, path::Path};

use super::{
    error::{OracleError, OracleTypedResult},
    oracle::OracleClient,
};

/// An individual profile
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProfileConfig {
    /// Private key .
    pub private_key: Option<Ed25519PrivateKey>,
    /// Public key
    pub public_key: Option<Ed25519PublicKey>,
    /// Account
    pub account: Option<AccountAddress>,
    /// URL for the Aptos rest endpoint
    pub rest_url: Option<String>,
    /// URL for the Faucet endpoint (if applicable)
    pub faucet_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OracleConfig {
    /// Map of profile configs
    pub profiles: Option<HashMap<String, ProfileConfig>>,
    //pub path: String,
}

impl OracleConfig {
    pub fn read_config<T: DeserializeOwned>(path: &str) -> OracleTypedResult<T> {
        let read_file = Self::read_from_file(Path::new(path))?;

        Self::from_yaml(&String::from_utf8(read_file).map_err(|err| {
            OracleError::UnableToParse(
                format!("Error at Oracle_config::read_config function {}", path),
                err.to_string(),
            )
        })?)
    }

    fn read_from_file(path: &Path) -> OracleTypedResult<Vec<u8>> {
        std::fs::read(path).map_err(|e| {
            OracleError::UnableToReadFile(
                format!(
                    "Error at Oracle_config::read_from_file function {}",
                    path.display()
                ),
                e.to_string(),
            )
        })
    }

    pub fn from_yaml<T: DeserializeOwned>(input: &str) -> OracleTypedResult<T> {
        //println!("The input is {} ", input);
        serde_yaml::from_str(input).map_err(|err| {
            OracleError::UnableToParse(
                format!("Error at Oracle_config::from_yaml function {}", input),
                err.to_string(),
            )
        })
    }

    pub fn clone(key: &Ed25519PrivateKey) -> Ed25519PrivateKey {
        let serialized: &[u8] = &(key.to_bytes());
        Ed25519PrivateKey::try_from(serialized).unwrap()
    }

    pub async fn load_default_account(
        oracle_client: &OracleClient,
        config_path: &str,
    ) -> OracleTypedResult<LocalAccount> {
        // println!("The path is {}", &self.path);

        let config: OracleConfig = match OracleConfig::read_config(config_path) {
            Ok(result) => result,
            Err(err) => {
                return Err(OracleError::UnableToReadFile(
                    "Unable to read file at load_default_account".to_string(),
                    err.to_string(),
                ))
            }
        };

        let profiles = match config.profiles {
            Some(result) => result,
            None => {
                return Err(OracleError::UnableToParse(
                    "Not found ".to_string(),
                    "profiles".to_string(),
                ));
            }
        };

        let default_profile = match profiles.get("default") {
            Some(result) => result,
            None => {
                return Err(OracleError::UnableToParse(
                    "Something went wrong while reading ".to_string(),
                    "default_profile".to_string(),
                ));
            }
        };

        let key = match default_profile.private_key.as_ref() {
            Some(result) => result,
            None => {
                return Err(OracleError::UnableToParse(
                    "Something went wrong while reading ".to_string(),
                    "secret key".to_string(),
                ));
            }
        };

        let key = OracleConfig::clone(key);

        let sender_address = AuthenticationKey::ed25519(&key.public_key()).derived_address();
        let sender_address = AccountAddress::new(*sender_address);
        let sequence_number =
            OracleConfig::get_sequence_number(oracle_client, sender_address).await?;

        Ok(LocalAccount::new(sender_address, key, sequence_number))
    }

    /// Retrieves sequence number from the rest client
    pub async fn get_sequence_number(
        oracle_client: &OracleClient,
        address: AccountAddress,
    ) -> OracleTypedResult<u64> {
        let account_response = oracle_client
            .api_client
            .get_account(address)
            .await
            .map_err(|err| {
                OracleError::UnexpectedError(format!(
                    "Fetch error from aptos :  {}",
                    err.to_string()
                ))
            })?;
        let account = account_response.inner();
        Ok(account.sequence_number)
    }
}
