use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Aggregator {
    pub id: u8,
    pub source: String,
    pub tokens: Data,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Data {
    pub data: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Token {
    pub key: String,
    pub value: TokenEntry,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TokenEntry {
    pub name: String,
    pub symbol: String,
    pub token_details_list: Vec<TokenDetails>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenDetails {
    pub decimals: u8,
    //write deserialize with
    pub last_update: String,
    //write deserialize with
    pub price: String,
}

impl Default for TokenDetails {
    fn default() -> Self {
        Self {
            decimals: 0,
            last_update: "".to_string(),
            price: "0".to_string(),
        }
    }
}
