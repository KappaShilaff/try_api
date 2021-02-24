use std::convert::TryFrom;
use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use opg::*;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, OpgModel)]
#[opg("accountid string", format = "id", example = "abcd0001")]
pub struct AccountId(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, OpgModel)]
#[serde(rename_all = "camelCase")]
#[opg("ExchangeName")]
pub enum ExchangeName {
    Binance,
    HitBtc,
    Kraken,
    Okex,
    Kucoin,
    Bitfinex,
    Huobi,
    Quoine,
}

#[derive(Debug, Error)]
pub struct ExchangeConvertError;

impl fmt::Display for ExchangeConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl TryFrom<String> for ExchangeName {
    type Error = ExchangeConvertError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Binance" => Ok(Self::Binance),
            "HitBtc" => Ok(Self::HitBtc),
            "Kraken" => Ok(Self::Kraken),
            "Okex" => Ok(Self::Okex),
            "Kucoin" => Ok(Self::Kucoin),
            "Bitfinex" => Ok(Self::Bitfinex),
            "Huobi" => Ok(Self::Huobi),
            "Quoine" => Ok(Self::Quoine),
            _ => Err(ExchangeConvertError)
        }
    }
}

impl fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum OrderState {
    New,
    Placed,
    PartiallyFilled,
    Filled,
    Cancelled,
    Error,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum OrderSide {
    Sell,
    Buy,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CreateOrder {
    pub order_type: OrderType,
    pub price: Decimal,
    pub volume: Decimal,
    pub side: OrderSide,
    pub base: Currency,
    pub counter: Currency,
    pub user_id: UserOrderId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Order {
    pub order_type: OrderType,
    pub price: Decimal,
    pub volume: Decimal,
    pub side: OrderSide,
    pub base: Currency,
    pub counter: Currency,
    pub user_id: UserOrderId,
    pub market_id: MarketId,
    pub state: OrderState,
    pub filled_volume: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct MarketId(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Balance(pub Vec<BalancePair>);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Currency(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Trade {
    pub id: TradeId,
    pub volume: Decimal,
    pub price: Decimal,
    pub order_id: UserOrderId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct UserOrderId(
    pub Uuid
);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct TradeId(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct BalancePair {
    pub currency: Currency,
    pub volume: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum ExecutorResponse {
    GetBalanceResponse { res: Result<Balance, ExecutorError> },
    PlaceOrderResponse { res: Result<MarketId, ExecutorError>, id: UserOrderId },
    CancelOrderResponse { res: Result<(), ExecutorError>, id: UserOrderId },
    GetOrderResponse { res: Result<Order, ExecutorError>, id: UserOrderId },
    GetTradesResponse { res: Result<Vec<Trade>, ExecutorError>, id: UserOrderId },
}

#[derive(Error, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum ExecutorError {
    PlaceOrderError,
    CancelOrderError,
    GetOrderError,
}

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}