use crate::dto::CreateAccountDto;
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
                    body: CreateAccountDto,
                    201: String,
                }
            },
        }
    };
    serde_yaml::to_string(&api).unwrap()
}