use crate::client::ApiClient;
use crate::error::ApiResult;

pub struct SettingsServiceImpl {
    client: ApiClient,
}

impl SettingsServiceImpl {
    const BASE_PATH: &'static str = "/me";

    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
}

pub trait SettingsService {
    fn settings(&self) -> ApiResult<serde_json::Value>;
}

impl SettingsService for SettingsServiceImpl {
    fn settings(&self) -> ApiResult<serde_json::Value> {
        self.client.get(Self::BASE_PATH)
    }
}
