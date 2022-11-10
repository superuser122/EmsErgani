use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Credentials{
    #[serde(rename = "Username")]
    pub user_name: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Usertype")]
    pub user_type: String,
}