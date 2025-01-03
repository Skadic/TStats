use poem_openapi::Object;
use serde::Serialize;

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

#[derive(Object, PartialEq, Eq, Debug, Clone)]
#[oai(rename = "AuthenticatedUser", rename_all = "camelCase")]
pub struct AuthenticatedUserDto {
    pub user_id: u32,
}

#[derive(Object, PartialEq, Eq, Debug, Clone)]
#[oai(rename = "DeliverAuthCodeResponse", rename_all = "camelCase")]
pub struct DeliverAuthCodeResponseDto {
    pub user_id: u32,
    pub return_url: String,
}
