use crate::models::{AccountId, ExchangeName};
use std::fmt::Error;
use sqlx::{Pool, Postgres};
use crate::db::AccountOrm;


#[derive(Clone)]
pub struct AccountRepo{
    pub account_orm: AccountOrm,
}

impl AccountRepo {
    pub async fn new(pg_pool: Pool<Postgres>) -> AccountRepo {
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
        match self.account_orm.remove_key(uid).await {
            Ok(()) => Ok(()),
            Err(err) => {
                eprintln!("{}", err);
                Err(Error::default())
            }
        }
    }

    pub async fn update_account(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        api_key: Option<String>,
        sign_key: Option<String>,
    ) -> Result<String, Error> {
       match self.account_orm.update_account(uid, exchange, api_key, sign_key).await {
           Ok(res) => Ok(res),
           Err(err) => {
               eprintln!("{}", err);
               Err(Error::default())
           }
       }
    }

    pub async fn get_api_key(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
    ) -> Result<String, Error> {
        match self.account_orm.get_api_key(uid, exchange).await {
            Ok(res) => Ok(res),
            Err(err) => {
                eprintln!("{}", err);
                Err(Error::default())
            }
        }
    }

    pub async fn remove_account(&self, uid: &AccountId) -> Result<(), Error> {
        match self.account_orm.remove_account(uid).await {
            Ok(()) => Ok(()),
            Err(err) => {
                eprintln!("{}", err);
                Err(Error::default())
            }
        }
    }
}