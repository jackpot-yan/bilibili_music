use crate::config;
use ureq;
use serde_json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Passport {
    data: PassData
}

#[derive(Deserialize, Debug)]
struct PassData {
    hash: String,
    key: String,
}

pub fn login() {
    match ureq::get(config::LOGINURL).call() {
        Ok(res) => {
            let data = res.into_string().unwrap_or_default();
            let data_json:Passport = serde_json::from_str(&data).unwrap();
            println!("{}", data_json.data.hash);
            println!("{}", data_json.data.key);
        },
        Err(err) => {
            println!("{}", err)
        },
    };
}
