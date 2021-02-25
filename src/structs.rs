#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub channel: String,
    pub message: Option<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "product_code")]
    pub product_code: String,
    pub state: String,
    pub timestamp: String,
    #[serde(rename = "tick_id")]
    pub tick_id: i64,
    #[serde(rename = "best_bid")]
    pub best_bid: f64,
    #[serde(rename = "best_ask")]
    pub best_ask: f64,
    #[serde(rename = "best_bid_size")]
    pub best_bid_size: f64,
    #[serde(rename = "best_ask_size")]
    pub best_ask_size: f64,
    #[serde(rename = "total_bid_depth")]
    pub total_bid_depth: f64,
    #[serde(rename = "total_ask_depth")]
    pub total_ask_depth: f64,
    #[serde(rename = "market_bid_size")]
    pub market_bid_size: f64,
    #[serde(rename = "market_ask_size")]
    pub market_ask_size: f64,
    pub ltp: f64,
    pub volume: f64,
    #[serde(rename = "volume_by_product")]
    pub volume_by_product: f64,
}
