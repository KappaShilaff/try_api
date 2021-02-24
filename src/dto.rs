use crate::models::{ExchangeName};
use serde::{Deserialize, Serialize};
use opg::*;

#[derive(Serialize, Deserialize, Debug, OpgModel)]
pub struct CreateAccountDto {
    pub uid: String,
    pub _exchange: ExchangeName,
    pub api_key: String,
    pub sign_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, OpgModel)]
pub struct SignAndGetDto {
    pub uid: String,
    pub _exchange: ExchangeName,
    pub data_to_sign: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, OpgModel)]
pub struct UpdateAccountDto {
    pub uid: String,
    pub _exchange: ExchangeName,
    pub api_key: Option<String>,
    pub sign_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, OpgModel)]
pub struct GetApiKeyDto {
    pub uid: String,
    pub _exchange: ExchangeName,
}
