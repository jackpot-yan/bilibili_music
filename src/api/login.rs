use crate::config;

use super::dto::Passport;

pub fn login() -> bool {
    match ureq::get(config::PASSURL).call() {
        Ok(res) => {
            let DataJson:Passport = res.into_json().unwrap();
            println!("{:?}", DataJson);
            true
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}
