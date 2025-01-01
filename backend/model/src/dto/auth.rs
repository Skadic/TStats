use poem_openapi::{ApiResponse, Object};
use serde::Serialize;

#[derive(Object, Debug, Serialize)]
#[oai(rename = "RequestAuthCodeResponse")]
pub struct RequestAuthCodeResponseDto {
    pub auth_url: String,
}

#[derive(Object, PartialEq, Eq, Debug, Clone)]
#[oai(rename = "DeliverAuthCodeRequest")]
pub struct DeliverAuthCodeRequestDto {
    pub auth_code: String,
    pub state: String,
}

#[derive(ApiResponse)]
pub enum DeliverAuthCodeResponseDto {
    #[oai(status = "301")]
    Redirect(#[oai(header = "Location")] String),
}
