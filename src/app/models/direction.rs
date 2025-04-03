use std::collections::HashMap;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Direction {
    #[serde(rename = "DMGId", default)]
    pub dmg_id: usize,
    pub app_upgrade: usize,
    pub client_id: String,
    pub client_secret: String,
    pub disabled_features: Vec<String>,
    #[serde(rename = "facebookAPIKey")]
    pub facebook_api_key: String,
    pub facebook_app_id: String,
    pub hw_id: usize,
    pub mayhem_game_code: String,
    pub mdm_app_key: String,
    pub package_id: String,
    pub poll_intervals: Vec<KVPair>,
    pub product_id: usize,
    pub result_code: usize,
    pub sell_id: usize,
    pub server_api_version: String,
    pub server_data: Vec<KVPair>,
    pub telemetry_freq: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DirectionToml {
    #[serde(rename = "DMGId", default)]
    pub dmg_id: Option<usize>,
    pub app_upgrade: Option<usize>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub disabled_features: Option<Vec<String>>,
    #[serde(rename = "facebookAPIKey")]
    pub facebook_api_key: Option<String>,
    pub facebook_app_id: Option<String>,
    pub hw_id: Option<usize>,
    pub mayhem_game_code: Option<String>,
    pub mdm_app_key: Option<String>,
    pub package_id: Option<String>,
    pub poll_intervals: Option<HashMap<String, String>>,
    pub product_id: Option<usize>,
    pub result_code: Option<usize>,
    pub sell_id: Option<usize>,
    pub server_api_version: Option<String>,
    pub server_data: Option<HashMap<String, String>>,
    pub telemetry_freq: Option<usize>,
}

impl Default for DirectionToml {
    fn default() -> Self {
        Self {
            client_secret: Some(
                "D0fpQvaBKmAgBRCwGPvROmBf96zHnAuZmNepQht44SgyhbCdCfFgtUTdCezpWpbRI8N6oPtb38aOVg2y"
                    .to_owned(),
            ),
            mayhem_game_code: Some("bg_gameserver_plugin".to_owned()),
            telemetry_freq: Some(300),
            poll_intervals: Some(
                vec![("badgePollInterval".to_owned(), "300".to_owned())]
                    .into_iter()
                    .collect(),
            ),
            dmg_id: None,
            app_upgrade: None,
            client_id: None,
            disabled_features: None,
            facebook_api_key: None,
            facebook_app_id: None,
            hw_id: None,
            mdm_app_key: None,
            package_id: None,
            product_id: None,
            result_code: None,
            sell_id: None,
            server_api_version: None,
            server_data: None,
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

impl From<(String, String)> for KVPair {
    fn from((key, value): (String, String)) -> Self {
        Self { key, value }
    }
}

impl From<KVPair> for (String, String) {
    fn from(value: KVPair) -> Self {
        (value.key, value.value)
    }
}

pub trait HashMapToVec<K, V, T>
where
    T: From<(K, V)>,
{
    fn map(value: HashMap<K, V>) -> Vec<T> {
        value.into_iter().map(T::from).collect()
    }
}

impl HashMapToVec<String, String, KVPair> for Vec<KVPair> {}
