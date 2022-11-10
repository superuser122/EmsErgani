use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthResponse{

    #[serde(rename = "accessToken")]
    pub token: String,

    #[serde(rename = "accessTokenExpired")]
    pub token_expired: i32,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,

    #[serde(rename = "refreshTokenExpired")]
    pub refresh_token_expired: String,
}