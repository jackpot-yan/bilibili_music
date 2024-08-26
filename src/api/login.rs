use crate::config;

use super::dto::Passport;

pub fn login() -> bool {
    match ureq::get(config::PASSURL).call() {
        Ok(res) => {
            let pass_port:Passport = res.into_json().unwrap();
            println!("{:?}",  pass_port);
            true
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}
