use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]

pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}
