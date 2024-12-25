use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;
use validator::Validate;

static EMAIL_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

#[derive(Deserialize, Validate)]
pub struct AuthFormModel {
    #[validate(regex(path=*EMAIL_RX, message="Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password min 8 characters"))]
    pub password: String,
}
