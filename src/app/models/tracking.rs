#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TrackingData {
    #[serde(rename = "advertiserID")]
    pub advertiser_id: String,
    pub android_id: Option<String>,
    pub build_id: String,
    pub carrier: String,
    pub device_id: String,
    pub events: Vec<TrackingEvent>,
    pub firmvare_ver: String,
    pub hw_id: String,
    pub jflag: String,
    pub limit_ad_tracking: bool,
    pub network_access: String,
    #[serde(rename = "now_timestamp")]
    pub now_timestamp: String,
    pub origin_user: String,
    pub persona: String,
    pub pflag: String,
    pub platform: String,
    pub schema_ver: String,
    pub sdk_cfg: String,
    pub sdk_ver: String,
    pub sell_id: String,
    pub timezone: String,
    pub uid: String,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TrackingEvent {
    pub repost_count: String,
    pub session: String,
    pub step: String,
    pub timestamp: String,
    pub event_type: String,
    pub event_key_type01: Option<String>,
    pub event_key_type02: Option<String>,
    pub event_key_type03: Option<String>,
    pub event_key_type04: Option<String>,
    pub event_key_type05: Option<String>,
    pub event_key_type06: Option<String>,
    pub event_key_type07: Option<String>,
    pub event_key_type08: Option<String>,
    pub event_key_type09: Option<String>,
    pub event_value01: Option<String>,
    pub event_value02: Option<String>,
    pub event_value03: Option<String>,
    pub event_value04: Option<String>,
    pub event_value05: Option<String>,
    pub event_value06: Option<String>,
    pub event_value07: Option<String>,
    pub event_value08: Option<String>,
    pub event_value09: Option<String>,
}
