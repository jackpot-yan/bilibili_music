use crate::config;
use base64::engine::general_purpose;
use base64::Engine;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::env;
use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{json, Error as ureqError};

use super::args::AppKeyStore;
use super::login_req::{self, Validate};
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
            let data_json: ValidateRes = res.into_json()?;
            return Ok(data_json);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

fn get_user() -> (String, String) {
    let user = env::var("b_user").unwrap_or_default();
    let password = env::var("b_password").unwrap_or_default();
    (user, password)
}

pub fn login() -> Result<(), ureqError> {
    let mut rand_png = rand::thread_rng();
    let salt = get_salt()?;
    let (user, password) = get_user();
    let pub_key = RsaPublicKey::from_public_key_pem(&salt.data.key).unwrap();
    let enc_data = pub_key
        .encrypt(
            &mut rand_png,
            Pkcs1v15Encrypt,
            (salt.data.hash + &password).as_bytes(),
        )
        .unwrap_or_default();
    let enc_password = general_purpose::STANDARD_NO_PAD.encode(enc_data);
    let mut payload = json!({
        "actionKey": "appkey".to_string(),
        "appKey": AppKeyStore::Android.app_key().to_string(),
        "build": 6270200,
        "captcha": "".to_string(),
        "challenge": "".to_string(),
        "channel": "bili".to_string(),
        "device": "phone".to_string(),
        "mobi_app": "android".to_string(),
        "password": enc_password,
        "permission": "ALL".to_string(),
        "platform": "android".to_string(),
        "seccode": "".to_string(),
        "subid": 1,
        "ts": SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        "username": user,
        "validate": "".to_string(),
    });
    let payload_encode = serde_json::to_string(&payload).unwrap();
    let sign = AppKeyStore::sign(&payload_encode, AppKeyStore::Android.app_sec());
    payload["sign"] = serde_json::Value::from(sign);
    match ureq::post(config::PASSWORDLOGIN).send_json(payload) {
        Ok(res) => {
            if res.status() != 200 {
                println!("{:?}", res)
            }
            println!("success");
            println!("{:?}", res.into_string());
        },
        Err(err) => {
            println!("{:?}", err)
        },
    };
    Ok(())
}
