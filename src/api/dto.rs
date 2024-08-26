use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Passport {
    pub data: PassData
}

#[derive(Deserialize, Debug, Default)]
pub struct PassData {
    pub hash: String,
    pub key: String
}
