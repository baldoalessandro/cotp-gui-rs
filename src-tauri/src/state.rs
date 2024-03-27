use std::sync::Mutex;

use cotp::otp::otp_element::OTPDatabase;

pub struct AppState {

    pub database: Mutex<Option<OTPDatabase>>,
    pub key: Mutex<Option<Vec<u8>>>,
    pub salt: Mutex<Option<Vec<u8>>>,
}
