use sqlx::{Pool, Postgres, PgPool};
use sqlx::postgres::{PgPoolOptions};
use crate::Memi32;


pub async fn db_connect() -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/test").await {
        Ok(pg) => pg,
        Err(err) => panic!(err)
    }
}

pub async fn mem(pg: &PgPool, id: &String) -> Result<i32, sqlx::Error> {
    let kek: Memi32 = sqlx::query_as!(
        Memi32,
        "INSERT INTO test.public.test (test_id, test_text)
         VALUES (DEFAULT, $1)
         RETURNING  test_id",
        id
    )
        .fetch_one(pg)
        .await?;
    Ok(kek.test_id)
}
