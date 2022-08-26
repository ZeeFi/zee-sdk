use aptos_sdk::{
    crypto::ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    types::account_address::AccountAddress,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml;
use std::{collections::HashMap, path::Path};

use super::error::{OracleError, OracleTypedResult};

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
}

impl OracleConfig {
    pub fn read_config<T: DeserializeOwned>(path: &str) -> OracleTypedResult<T> {
        let read_file = Self::read_from_file(Path::new(path))?;

        Self::from_yaml(
            &String::from_utf8(read_file)
                .map_err(|err| OracleError::UnableToParse(format!("{}", path), err.to_string()))?,
        )
    }

    fn read_from_file(path: &Path) -> OracleTypedResult<Vec<u8>> {
        std::fs::read(path).map_err(|e| {
            OracleError::UnableToReadFile(format!("{}", path.display()), e.to_string())
        })
    }

    fn from_yaml<T: DeserializeOwned>(input: &str) -> OracleTypedResult<T> {
        serde_yaml::from_str(input)
            .map_err(|err| OracleError::UnableToParse(format!("{}", input), err.to_string()))
    }
}
