use warp::{http, Filter, any};
use crate::models::{AccountId};
use crate::account::Account;
use crate::dto::{CreateAccountDto, SignAndGetDto, UpdateAccountDto, GetApiKeyDto};
use crate::db::{db_connect, mem};
use sqlx::{Pool, Postgres};

mod account;
mod models;
mod dto;
mod db;
mod docs;

fn json_body<T>() -> impl Filter<Extract=(T, ), Error=warp::Rejection> + Clone
    where
            for<'a> T: serde::Deserialize<'a> + Send,
{
    warp::body::content_length_limit(1024 * 1024).and(warp::filters::body::json::<T>())
}

#[tokio::main]
async fn main() {
    let db = db_connect().await;

    let swagger = warp::path!("swagger.yaml")
        .and(warp::get())
        .map(docs::swagger);

    let create_rout = warp::path!("account")
        .and(warp::post())
        .and(any().map(move || db.clone()))
        .and(json_body::<CreateAccountDto>())
        .and_then(create_account_rest);

    let sign_rout = warp::path!("account")
        .and(warp::put())
        .and(json_body::<SignAndGetDto>())
        .and_then(sign_and_key_rest);

    let account_update_rout = warp::path!("account")
        .and(warp::patch())
        .and(json_body::<UpdateAccountDto>())
        .and_then(update_account_rest);

    let remove_account_rout = warp::path!("account" / String)
        .and(warp::delete())
        .and_then(remove_account_rest);

    let remove_key_rout = warp::path!("key"/ "account" / String)
        .and(warp::delete())
        .and_then(remove_key_rest);

    let get_api_key_rout = warp::path!("key" / "account")
        .and(warp::put())
        .and(json_body::<GetApiKeyDto>())
        .and_then(get_api_key_rest);

    let routes = swagger
        .or(create_rout)
        .or(sign_rout)
        .or(remove_account_rout)
        .or(remove_key_rout)
        .or(account_update_rout)
        .or(get_api_key_rout);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(sqlx::FromRow)]
struct Memi32 {
    test_id: i32,
}

async fn create_account_rest(
    pg: Pool<Postgres>,
    create_account_dto: CreateAccountDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    let acc = Account::new();
    match mem(&pg, &create_account_dto.uid).await {
        Ok(lol) => println!("{}", lol),
        Err(err) => eprintln!("{}", err)
    }
    match acc.create_account(
        &AccountId(create_account_dto.uid),
        &create_account_dto._exchange,
        &create_account_dto.api_key,
        create_account_dto.sign_key,
    ).await {
        Ok(()) => {
            Ok(warp::reply::with_status(
                "Create account",
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                "Error",
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn sign_and_key_rest(
    sign_and_get_dto: SignAndGetDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    let acc = Account::new();
    match acc.sign_and_get_key(
        &AccountId(sign_and_get_dto.uid),
        &sign_and_get_dto._exchange,
        &sign_and_get_dto.data_to_sign,
    ).await {
        Ok((mda, kek)) => {
            Ok(warp::reply::with_status(
                format!("Sign get {} {}", mda, kek),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                "error".to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn remove_account_rest(
    account_id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let acc = Account::new();
    match acc.remove_account(
        &AccountId(account_id),
    ).await {
        Ok(()) => {
            Ok(warp::reply::with_status(
                "Account removed".to_string(),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                "error".to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn remove_key_rest(
    account_id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let acc = Account::new();
    match acc.remove_key(
        &AccountId(account_id),
    ).await {
        Ok(()) => {
            Ok(warp::reply::with_status(
                "Key removed".to_string(),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                "error".to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn update_account_rest(
    update_account_dto: UpdateAccountDto
) -> Result<impl warp::Reply, warp::Rejection> {
    let acc = Account::new();
    match acc.update_account(
        &AccountId(update_account_dto.uid),
        &update_account_dto._exchange,
        update_account_dto.api_key,
        update_account_dto.sign_key,
    ).await {
        Ok(response) => {
            Ok(warp::reply::with_status(
                format!("Account updated {}", response),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                "Update error".to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn get_api_key_rest(
    get_api_key_dto: GetApiKeyDto
) -> Result<impl warp::Reply, warp::Rejection> {
    let acc = Account::new();
    match acc.get_api_key(
        &AccountId(get_api_key_dto.uid),
        &get_api_key_dto._exchange,
    ).await {
        Ok(api_key) => {
            Ok(warp::reply::with_status(
                format!("Api key {}", api_key),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                "Key not found".to_string(),
                http::StatusCode::NOT_FOUND,
            ))
        }
    }
}
