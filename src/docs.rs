use crate::dto::{CreateAccountDto, SignAndGetDto, UpdateAccountDto, GetApiKeyDto};
use opg::*;

pub fn swagger() -> String {
    let api = describe_api! {
    info: {
            title: "My super API",
            version: "0.0.0",
    },
    servers: {
        "http://127.0.0.1:3030",
    },
    paths: {
            ("account"): {
                POST: {
                    summary: "Create account",
                    body: CreateAccountDto,
                    200: String,
                    400: String,
                },
                PUT: {
                    summary: "Sign and get key",
                    body: SignAndGetDto,
                    200: String,
                    400: String,
                },
                PATCH: {
                    summary: "Update account",
                    body: UpdateAccountDto,
                    200: String,
                    400: String,
                }
            },
            ("account" / {account_id: String}): {
                DELETE: {
                    summary: "Delete account",
                    200: String,
                    400: String,
                }
            },
            ("key"/ "account" / {account_id: String}): {
                DELETE: {
                    summary: "Delete key",
                    200: String,
                    400: String,
                }
            },
            ("key" / "account"): {
                PUT: {
                    summary: "Get api key",
                    body: GetApiKeyDto,
                    200: String,
                    400: String,
                }
            }
        }
    };
    serde_yaml::to_string(&api).unwrap()
}