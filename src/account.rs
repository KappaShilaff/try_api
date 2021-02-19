use crate::models::{AccountId, ExchangeName};
use std::fmt::Error;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Account {
    pub uid: AccountId,
    pub exchange: ExchangeName,
    pub data_to_sign: u8,
    pub api_key: Option<String>,
    pub sign_key: Option<String>,
}

impl Account {
    pub fn new() -> Account {
        Account {
            uid: AccountId("".to_string()),
            exchange: ExchangeName::Binance,
            data_to_sign: 0,
            api_key: None,
            sign_key: None,
        }
    }
    pub async fn sign_and_get_key(
        &self,
        uid: &AccountId,
        _exchange: &ExchangeName,
        data_to_sign: &[u8],
    ) -> Result<(String, String), Error> {
        if !uid.0.is_empty() && !data_to_sign.is_empty() {
            Ok(((uid.0.to_string()), data_to_sign.get(0).unwrap().to_string()))
        } else {
            Err(Error::default())
        }
    }

    pub async fn create_account(
        &self,
        uid: &AccountId,
        _exchange: &ExchangeName,
        api_key: &str,
        sign_key: Option<String>,
    ) -> Result<(), Error> {
        if !uid.0.is_empty() && !api_key.is_empty() && sign_key.is_some() {
            Ok(())
        } else {
            Err(Error::default())
        }
    }

    pub async fn remove_key(&self, uid: &AccountId) -> Result<(), Error> {
        if !uid.0.is_empty() {
            Ok(())
        } else {
            Err(Error::default())
        }
    }

    pub async fn update_account(
        &self,
        uid: &AccountId,
        _exchange: &ExchangeName,
        api_key: Option<String>,
        sign_key: Option<String>,
    ) -> Result<String, Error> {
        if !uid.0.is_empty() && api_key.is_some() && sign_key.is_some() {
            Ok(uid.0.clone())
        } else {
            Err(Error::default())
        }
    }

    pub async fn get_api_key(
        &self,
        uid: &AccountId,
        _exchange: &ExchangeName,
    ) -> Result<String, Error> {
        if !uid.0.is_empty() {
            Ok(uid.0.to_string())
        } else {
            Err(Error::default())
        }
    }
    pub async fn remove_account(&self, id: &AccountId) -> Result<(), Error> {
        if !id.0.is_empty() {
            Ok(())
        } else {
            Err(Error::default())
        }
    }
}