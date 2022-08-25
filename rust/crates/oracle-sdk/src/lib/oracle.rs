use crate::common::RestClient;

pub struct OracleClient {
    pub rest_client: RestClient,
}

impl OracleClient {
    pub fn new(url: String) -> Self {
        Self {
            rest_client: RestClient::new(url),
        }
    }
}
