use sqlx::{Pool, Postgres, PgPool, Error};
use sqlx::postgres::{PgPoolOptions};
use crate::models::{AccountId, ExchangeName};
use serde::{Deserialize, Serialize};
use opg::ModelTypeDescription::Integer;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct AccountEntity {
    uid: String,
    exchange: ExchangeName,
    api_key: Option<String>,
    sign_key: Option<String>,
    data_to_sign: Option<Vec<u8>>,
}

pub async fn db_connect() -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/test").await {
        Ok(pg) => pg,
        Err(err) => panic!(err)
    }
}

#[derive(Copy, Clone)]
pub struct AccountOrm <'a> {
    pg_pool: &'a Pool<Postgres>,
}

impl AccountOrm<'_> {
    pub async fn new(pg_pool: &Pool<Postgres>) -> AccountOrm<'_> {

        AccountOrm{ pg_pool }
    }
    pub async fn create_account (
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        api_key: &str,
        sign_key: Option<String>,
    ) -> Result<String, Error> {
        let result = sqlx::query!(
        r#"INSERT INTO test.public.accounts (uid, exchange, api_key, sign_key)
         VALUES ($1, $2, $3, $4)
         RETURNING (uid)"#,
        uid.0,
        exchange.to_string(),
        api_key,
        sign_key,
    )
            .fetch_one(&*self.pg_pool)
            .await?;
        Ok(result.uid)
    }

    pub async fn sign_and_get_key (
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        data_to_sign: &[u8],
    ) -> Result<(String, String), Error> {
        let lol: Vec<i32> = data_to_sign.to_vec().into_iter().map(move |x| i32::from_u8(x).unwrap()).collect();
        let result = sqlx::query!(
        r#"UPDATE test.public.accounts SET data_to_sign = $1
         WHERE uid = $2 AND exchange = $3
         RETURNING uid, api_key;"#,
        lol.as_slice(),
        uid.0,
        exchange.to_string(),
    )
            .fetch_one(&*self.pg_pool)
            .await?;
        Ok((result.uid, result.api_key.unwrap()))
    }
}
