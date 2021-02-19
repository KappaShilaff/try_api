use crate::models::{ExchangeName};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct CreateAccountDto {
    pub uid: String,
    pub _exchange: ExchangeName,
    pub api_key: String,
    pub sign_key: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SignAndGetDto {
    pub uid: String,
    pub _exchange: ExchangeName,
    pub data_to_sign: Vec<u8>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateAccountDto {
    pub uid: String,
    pub _exchange: ExchangeName,
    pub api_key: Option<String>,
    pub sign_key: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetApiKeyDto {
    pub uid: String,
    pub _exchange: ExchangeName,
}
