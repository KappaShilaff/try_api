use warp::{http, Filter};
use crate::models::{AccountId};
use crate::account::AccountRepo;
use crate::dto::{CreateAccountDto, SignAndGetDto, UpdateAccountDto, GetApiKeyDto};
use crate::db::{db_connect};
use std::sync::Arc;

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
    let account_repo = Arc::new(AccountRepo::new(db.clone()).await);
    let state = warp::any().map(move || account_repo.clone());
    let swagger = warp::path!("swagger.yaml")
        .and(warp::get())
        .map(docs::swagger);

    let create_rout = warp::path!("account")
        .and(warp::post())
        .and(state.clone())
        .and(json_body::<CreateAccountDto>())
        .and_then(create_account_rest);

    let sign_rout = warp::path!("account")
        .and(warp::put())
        .and(state.clone())
        .and(json_body::<SignAndGetDto>())
        .and_then(sign_and_key_rest);

    let account_update_rout = warp::path!("account")
        .and(warp::patch())
        .and(state.clone())
        .and(json_body::<UpdateAccountDto>())
        .and_then(update_account_rest);

    let remove_account_rout = warp::path!("account" / String)
        .and(warp::delete())
        .and(state.clone())
        .and_then(remove_account_rest);

    let remove_key_rout = warp::path!("key"/ "account" / String)
        .and(warp::delete())
        .and(state.clone())
        .and_then(remove_key_rest);

    let get_api_key_rout = warp::path!("key" / "account")
        .and(warp::put())
        .and(state.clone())
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

async fn create_account_rest(
    account_repo: Arc<AccountRepo>,
    create_account_dto: CreateAccountDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    match account_repo.create_account(
        &AccountId(create_account_dto.uid),
        &create_account_dto.exchange,
        &create_account_dto.api_key,
        create_account_dto.sign_key,
    ).await {
        Ok(()) => {
            Ok(warp::reply::with_status(
                "Create account".to_string(),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                err.to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn sign_and_key_rest(
    account_repo: Arc<AccountRepo>,
    sign_and_get_dto: SignAndGetDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    match account_repo.sign_and_get_key(
        &AccountId(sign_and_get_dto.uid),
        &sign_and_get_dto.exchange,
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
                err.to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn remove_account_rest(
    account_id: String,
    account_repo: Arc<AccountRepo>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match account_repo.remove_account(
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
                err.to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn remove_key_rest(
    account_id: String,
    account_repo: Arc<AccountRepo>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match account_repo.remove_key(
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
                err.to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn update_account_rest(
    account_repo: Arc<AccountRepo>,
    update_account_dto: UpdateAccountDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    match account_repo.update_account(
        &AccountId(update_account_dto.uid),
        &update_account_dto.exchange,
        update_account_dto.api_key,
        update_account_dto.sign_key,
    ).await {
        Ok(response) => {
            Ok(warp::reply::with_status(
                format!("Account with uid \"{}\" updated", response),
                http::StatusCode::OK,
            ))
        }
        Err(err) => {
            println!("{}", err.to_string());
            Ok(warp::reply::with_status(
                err.to_string(),
                http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

async fn get_api_key_rest(
    account_repo: Arc<AccountRepo>,
    get_api_key_dto: GetApiKeyDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    match account_repo.get_api_key(
        &AccountId(get_api_key_dto.uid),
        &get_api_key_dto.exchange,
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
                err.to_string(),
                http::StatusCode::NOT_FOUND,
            ))
        }
    }
}
