use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct EmailAddress(String);

static VALIDATOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$").unwrap());

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self, String> {
        if VALIDATOR.is_match(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(format!("Validation of email {} failed", email))
        }
    }
}

impl ToString for EmailAddress {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
