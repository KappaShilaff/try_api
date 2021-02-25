use crate::models::{AccountId, ExchangeName};
use std::fmt::Error;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use crate::db::AccountOrm;
use crate::main;


#[derive(Copy, Clone)]
pub struct AccountRepo {
    pub account_orm: AccountOrm<'static>,
}

impl AccountRepo {
    pub async fn new(pg_pool: &'static Pool<Postgres>) -> AccountRepo {
        let account_orm = AccountOrm::new(pg_pool).await;
        AccountRepo{ account_orm }
    }

    pub async fn sign_and_get_key(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        data_to_sign: &[u8],
    ) -> Result<(String, String), Error> {
        match self.account_orm.sign_and_get_key(uid, exchange, data_to_sign).await {
            Ok(result) => Ok(result),
            Err(err) => {
                eprintln!("{}", err);
                Err(Error::default())
            }
        }
    }

    pub async fn create_account(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        api_key: &str,
        sign_key: Option<String>,
    ) -> Result<(), Error> {
        match self.account_orm.create_account(uid, exchange, api_key, sign_key).await {
            Ok(account_id) => {
                println!("{}", account_id);
                Ok(())
            }
            Err(err) => {
                eprintln!("{}", err);
                Err(Error::default())
            }
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