use crate::rest_model::{string_or_float, string_or_float_opt};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Long,
    Short,
    Both,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginType {
    #[serde(alias = "isolated")]
    Isolated,
    #[serde(alias = "cross")]
    Crossed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopProfit,
    TakeProfit,
    StopMarket,
    TakeProfitMarket,
    TrailingStopMarket,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    Gtc,
    GteGtc,
    Ioc,
    Fok,
    Gtx,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    New,
    Canceled,
    Calculated,
    Expired,
    Trade,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Expired,
    NewInsurance,
    NewAdl,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListenKeyExpired {
    #[serde(rename = "E")]
    pub event_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum WebsocketEvent {
    #[serde(alias = "listenKeyExpired")]
    ListenKeyExpired(Box<ListenKeyExpired>),
    #[serde(alias = "ORDER_TRADE_UPDATE")]
    OrderTradeUpdate(Box<OrderTradeUpdate>),
    #[serde(alias = "MARGIN_CALL")]
    MarginCall(Box<MarginCall>),
    #[serde(alias = "ACCOUNT_UPDATE")]
    AccountUpdate(Box<AccountUpdate>),
    #[serde(alias = "ACCOUNT_CONFIG_UPDATE")]
    AccountConfigUpdate(Box<AccountConfigUpdate>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTradeUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "o")]
    pub order: Order,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub client_order_id: String,

    #[serde(rename = "S")]
    pub order_side: OrderSide,

    #[serde(rename = "o")]
    pub order_type: OrderType,

    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,

    #[serde(with = "string_or_float")]
    #[serde(rename = "q")]
    pub original_quantity: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "p")]
    pub original_price: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "ap")]
    pub average_price: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "sp")]
    pub stop_price: f64,

    #[serde(rename = "x")]
    pub execution_type: ExecutionType,

    #[serde(rename = "X")]
    pub status: OrderStatus,

    #[serde(rename = "i")]
    pub id: u64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "l")]
    pub last_filled_quantity: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "z")]
    pub filled_accumulated_quantity: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "L")]
    pub last_filled_price: f64,

    #[serde(rename = "N")]
    pub commission_asset: Option<String>,

    #[serde(default, with = "string_or_float_opt")]
    #[serde(rename = "n")]
    pub commission: Option<f64>,

    #[serde(rename = "T")]
    pub trade_time: u64,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "b")]
    pub bids_notional: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "a")]
    pub ask_notional: f64,

    #[serde(rename = "m")]
    pub maker_side: bool,

    #[serde(rename = "R")]
    pub reduce_only: bool,

    #[serde(rename = "wt")]
    pub stop_price_working_type: WorkingType,

    #[serde(rename = "ot")]
    pub original_order_type: OrderType,

    #[serde(rename = "ps")]
    pub position_side: PositionSide,

    #[serde(rename = "cp")]
    pub close_position: bool,

    #[serde(default, with = "string_or_float_opt")]
    #[serde(rename = "AP")]
    pub activation_price: Option<f64>,

    #[serde(default, with = "string_or_float_opt")]
    #[serde(rename = "cr")]
    pub callback_rate: Option<f64>,

    #[serde(with = "string_or_float")]
    #[serde(rename = "rp")]
    pub realized_profit: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarginCall {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "cw")]
    pub cross_wallet_balance: f64,

    #[serde(rename = "p")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "ps")]
    pub side: PositionSide,

    #[serde(with = "string_or_float")]
    #[serde(rename = "pa")]
    pub amount: f64,

    #[serde(rename = "mt")]
    pub margin_type: MarginType,

    #[serde(with = "string_or_float")]
    #[serde(rename = "iw")]
    pub isolated_wallet: f64,

    #[serde(default, with = "string_or_float_opt")]
    #[serde(rename = "mp")]
    pub mark_price: Option<f64>,

    #[serde(with = "string_or_float")]
    #[serde(rename = "up")]
    pub unrealized_pnl: f64,

    #[serde(default, with = "string_or_float_opt")]
    #[serde(rename = "mm")]
    pub maintenance_margin_required: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "a")]
    pub data: UpdateData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,

    #[serde(with = "string_or_float")]
    #[serde(rename = "wb")]
    pub balance: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "cw")]
    pub cross_wallet: f64,

    #[serde(with = "string_or_float")]
    #[serde(rename = "bc")]
    pub balance_change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateData {
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,

    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountConfigUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "ac", alias = "ai")]
    pub update: ConfigUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ConfigUpdate {
    Leverage {
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "l")]
        value: u64,
    },
    MultiAssets {
        #[serde(rename = "j")]
        multi_assets: bool,
    },
}

#[cfg(test)]
mod test {
    use super::WebsocketEvent;

    /// Test the conversion of futures websocket event as received from binance into WebsocketEvent struct.
    #[test]
    fn deserialize_messages() {
        let fc = std::fs::read_to_string("test_data/futuresWebsocketsUserData.json").unwrap();
        match serde_json::from_str::<Vec<WebsocketEvent>>(&fc) {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };
    }
}
