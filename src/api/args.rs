pub(crate) enum AppKeyStore {
    BiliTV,
    Android
}

impl AppKeyStore {
    pub fn app_key(&self) -> &'static str {
        match self {
            AppKeyStore::BiliTV => "4409e2ce8ffd12b8",
            AppKeyStore::Android => "783bbb7264451d82"
        }
    }

    pub fn app_sec(&self) -> &'static str {
        match self {
            AppKeyStore::BiliTV => "59b43e04ad6965f34319062b478f83dd",
            AppKeyStore::Android => "2653583c8873dea268ab9386918b1d65"
        }
    }

    // pub fn sign(param:&str, app_sec:&str) -> String {
    // }
}