use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Passport {
    pub data: PassData,
}

#[derive(Deserialize, Debug, Default)]
pub struct PassData {
    pub hash: String,
    pub key: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Captcha {
    pub code: i8,
    pub message: String,
    pub ttl: i8,
    pub data: CaptchaData,
}

#[derive(Deserialize, Debug, Default)]
pub struct CaptchaData {
    pub recaptcha_type: String,
    pub recaptcha_token: String,
    pub gee_challenge: String,
    pub gee_gt: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ValidateRes {
    pub code:i8,
    pub message:String,
    pub ttl:i8,
    pub data: VaildateKey
}

#[derive(Deserialize, Debug, Default)]
pub struct VaildateKey {
    pub key: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct LoginRes {
    pub code: i16,
    pub message: String,
    pub ttl:i8
}