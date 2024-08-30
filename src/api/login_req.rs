#[derive(Debug, Default, serde::Serialize)]
pub struct Validate {
    pub tmp_code: String,
    pub sms_type: String,
    pub recaptcha_token: String,
    pub gee_challenge: String,
}

#[derive(Debug,Default, serde::Serialize)]
pub struct LoginReq {
    pub actionKey: String,
    pub appKey: String,
    pub build: i32,
    pub captcha: String,
    pub challenge: String,
    pub channel:String,
    pub device:String,
    pub mobi_app:String,
    pub password:String,
    pub permission:String,
    pub platform:String,
    pub seccode:String,
    pub subid: i8,
    pub ts: u64,
    pub username:String,
    pub validate:String
}
