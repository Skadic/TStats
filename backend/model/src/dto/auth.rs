use poem_openapi::Object;
use rosu_v2::prelude::User;
use serde::{Deserialize, Serialize};

#[derive(Object, Debug, Serialize)]
#[oai(rename = "RequestAuthCodeResponse", rename_all = "camelCase")]
pub struct RequestAuthCodeResponseDto {
    pub auth_url: String,
}

#[derive(Object, PartialEq, Eq, Debug, Clone)]
#[oai(rename = "DeliverAuthCodeRequest", rename_all = "camelCase")]
pub struct DeliverAuthCodeRequestDto {
    pub auth_code: String,
    pub state: String,
}

#[derive(Object, PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
#[oai(rename = "AuthenticatedUser", rename_all = "camelCase")]
pub struct AuthenticatedUserDto {
    pub user_id: u32,
    pub username: String,
    pub country: String
}

impl From<User> for AuthenticatedUserDto {
    fn from(value: User) -> Self {
        AuthenticatedUserDto {
            user_id: value.user_id,
            username: value.username.to_string(),
            country: value.country.unwrap()
        }
    }
}

#[derive(Object, PartialEq, Eq, Debug, Clone)]
#[oai(rename = "DeliverAuthCodeResponse", rename_all = "camelCase")]
pub struct DeliverAuthCodeResponseDto {
    pub return_url: String,
    pub user: AuthenticatedUserDto
}
