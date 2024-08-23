use crate::config;

use super::dto::Passport;

pub fn login() -> bool {
    match ureq::get(config::PASSURL).call() {
        Ok(res) => {
            let data_json:Passport = res.into_json().unwrap();
            println!("{:?}", data_json);
            true
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}
