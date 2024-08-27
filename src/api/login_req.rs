
#[derive(Debug, Default, serde::Serialize)]
pub struct Validate {
    pub tmp_code: String,
    pub sms_type: String,
    pub recaptcha_token:String,
    pub gee_challenge:String,
}