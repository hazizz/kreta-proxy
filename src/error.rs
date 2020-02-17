use crate::resources::KretaErrorResponse;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use chrono::Utc;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    error_code: u32,
    title: String,
    message: String,
    time: String,
}

impl ErrorResponse {
    pub fn from_message(error_code: u32, title: String, message: String) -> ErrorResponse {
        ErrorResponse {
            error_code,
            title,
            message,
            time: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Display)]
pub enum KretaError {
    #[display(fmt = "Kreta invalid response!")]
    KretaBadResponse(reqwest::Error),
    #[display(fmt = "Invalid access token!")]
    KretaRequestSendFailed(reqwest::Error),
    #[display(fmt = "Response couldn't be parsed!")]
    ParseError(reqwest::Error),
    #[display(fmt = "Kreta responses with error!")]
    ErrorResponse(KretaErrorResponse),
}

impl actix_web::error::ResponseError for KretaError {
    fn error_response(&self) -> HttpResponse {
        match self {
            KretaError::KretaBadResponse(err) => HttpResponse::build(
                StatusCode::from_u16(500).unwrap(),
            )
            .json(ErrorResponse::from_message(
                21,
                String::from("Unrecognisable response from kreta server"),
                format!("{}", err),
            )),

            KretaError::KretaRequestSendFailed(err) => HttpResponse::build(
                StatusCode::from_u16(500).unwrap(),
            )
            .json(ErrorResponse::from_message(
                20,
                String::from("Request failed"),
                format!("{}", err),
            )),
            KretaError::ParseError(err) => HttpResponse::build(StatusCode::from_u16(500).unwrap())
                .json(ErrorResponse::from_message(
                    22,
                    String::from("Parsing response failed"),
                    format!("{}", err),
                )),
            KretaError::ErrorResponse(response) => HttpResponse::build(
                StatusCode::from_u16(400).unwrap(),
            )
            .json(ErrorResponse::from_message(
                22,
                String::from("Kreta error response"),
                format!(
                    "error_title={};error_message={};error_code={}",
                    response.error, response.error_description, response.error_code
                ),
            )),
        }
    }
}
