use crate::config;
use std::io::Error;
use ureq::Error as ureqError;

use super::login_req::Validate;
use super::login_res::{Captcha, Passport, ValidateRes};

fn get_salt() -> Result<Passport, ureqError> {
    let ok = ureq::get(config::PASSURL).call().and_then(|op| {
        let salt: Result<Passport, Error> = op.into_json();
        return Ok(salt?);
    });
    ok
}

fn get_captcha() -> Result<Captcha, ureqError> {
    let captcha = ureq::post(config::CAPTCHA).call().and_then(|op| {
        let res: Result<Captcha, Error> = op.into_json();
        return Ok(res?);
    });
    captcha
}

fn verify(req: Validate) -> Result<ValidateRes, ureqError> {
    match ureq::post(config::VAILDATE).send_json(ureq::json!(req)) {
        Ok(res) => {
            let data_json:ValidateRes = res.into_json()?;
            return Ok(data_json);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub fn login() -> Result<(), ureqError> {
    let salt = get_salt()?;
    let captcha = get_captcha()?;
    println!("{:?}", captcha);
    let val = Validate {
        tmp_code: config::TMPCODE.to_string(),
        sms_type: config::SMSTYPE.to_string(),
        recaptcha_token: captcha.data.recaptcha_token,
        gee_challenge: captcha.data.gee_challenge,
    };
    let verify = verify(val);
    print!("{:?}", verify);
    Ok(())
}
