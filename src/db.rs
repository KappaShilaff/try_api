use sqlx::{Pool, Postgres, Error, Row};
use sqlx::postgres::{PgPoolOptions};
use crate::models::{AccountId, ExchangeName};
use serde::{Deserialize, Serialize};
use rust_decimal::prelude::{FromPrimitive};

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

#[derive(Clone)]
pub struct AccountOrm {
    pg_pool: Pool<Postgres>,
}

impl AccountOrm {
    pub async fn new(pg_pool: Pool<Postgres>) -> AccountOrm {
        AccountOrm { pg_pool }
    }
    pub async fn create_account(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        api_key: &str,
        sign_key: Option<String>,
    ) -> Result<String, Error> {
        let stringa = "INSERT INTO test.public.accounts (uid, exchange, api_key, sign_key)
         VALUES ($1, $2, $3, $4)
         RETURNING (uid)";
        let result = sqlx::query(stringa)
            .bind(&uid.0)
            .bind(exchange.to_string())
            .bind(api_key)
            .bind(sign_key)
            .fetch_one(&self.pg_pool)
            .await?;
        Ok(result.get(0))
    }

    pub async fn sign_and_get_key(
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
            .fetch_one(&self.pg_pool)
            .await?;
        Ok((result.uid, result.api_key.unwrap()))
    }

    pub async fn remove_key(
        &self,
        uid: &AccountId,
    ) -> Result<(), Error> {
        sqlx::query!(
        r#"UPDATE test.public.accounts SET api_key = NULL
         WHERE uid = $1;"#,
        uid.0,
    )
            .execute(&self.pg_pool)
            .await?;
        Ok(())
    }

    pub async fn remove_account(
        &self,
        uid: &AccountId,
    ) -> Result<(), Error> {
        sqlx::query!(
        r#"DELETE FROM test.public.accounts
         WHERE uid = $1;"#,
        uid.0,
    )
            .execute(&self.pg_pool)
            .await?;
        Ok(())
    }

    pub async fn get_api_key(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
    ) -> Result<String, Error> {
        let result = sqlx::query!(
        r#"SELECT api_key FROM test.public.accounts
         WHERE uid = $1 AND exchange = $2;"#,
        uid.0,
        exchange.to_string(),
    )
            .fetch_one(&self.pg_pool)
            .await?;
        if result.api_key.is_some() {
            Ok(result.api_key.unwrap())
        } else {
            Err(Error::RowNotFound)
        }
    }

    pub async fn update_account(
        &self,
        uid: &AccountId,
        exchange: &ExchangeName,
        api_key: Option<String>,
        sign_key: Option<String>,
    ) -> Result<String, Error> {
        let mut query = "UPDATE test.public.accounts SET exchange = $1,".to_string();
        if api_key.is_some() {
            query += " api_key = $2,";
       }
        if sign_key.is_some() {
            query += " sign_key = $3,";
        }
        query.remove(query.len() - 1);
        query += "\nWHERE uid = $4\n RETURNING uid";

        let result = sqlx::query(query.as_str())
            .bind(exchange.to_string())
            .bind(api_key)
            .bind(sign_key)
            .bind(&uid.0)
            .fetch_one(&self.pg_pool)
            .await?;
        Ok(result.get(0))
    }
}
