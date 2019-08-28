use chrono::Utc;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use std::error::Error;
use serde::{Serialize, Deserialize};
use failure::Fail;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    error_code: u32,
    title: String,
    message: String,
    time: String,
}

impl ErrorResponse{
    pub fn from_title(error_code: u32, title: String) -> ErrorResponse{
        ErrorResponse{
            error_code,
            title,
            message: String::new(),
            time: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn from_message(error_code: u32, title: String, message: String) -> ErrorResponse{
        ErrorResponse{
            error_code,
            title,
            message,
            time: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Fail, Debug)]
pub enum HazizzError {
    #[fail(display="token missing")]
    KretaTokenMissing,
    #[fail(display="url missing")]
    KretaUrlMissing,
    #[fail(display="username missing")]
    KretaUsernameMissing,
    #[fail(display="password missing")]
    KretaPasswordMissing,
    #[fail(display="from date missing")]
    KretaToDateMissing,
    #[fail(display="to date missing missing")]
    KretaFromDateMissing,
    #[fail(display="bad response from kreta")]
    KretaBadResponse(reqwest::Error),
    #[fail(display="could not send request")]
    KretaRequestSendFailed(reqwest::Error),
}

impl actix_web::error::ResponseError for HazizzError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HazizzError::KretaTokenMissing
            => HttpResponse::build(StatusCode::from_u16(400).unwrap())
                .json(ErrorResponse::from_title(26,
                                                String::from("The Token is missing"))),

            HazizzError::KretaUrlMissing
            => HttpResponse::build(StatusCode::from_u16(400).unwrap())
                .json(ErrorResponse::from_title(25,
                                                String::from("The Url is missing"))),

            HazizzError::KretaUsernameMissing
            => HttpResponse::build(StatusCode::from_u16(400).unwrap())
                    .json(ErrorResponse::from_title(27,
                                                    String::from("The Username is missing"))),

            HazizzError::KretaPasswordMissing
            => HttpResponse::build(StatusCode::from_u16(400).unwrap())
                .json(ErrorResponse::from_title(28,
                                                String::from("The Password is missing"))),

            HazizzError::KretaToDateMissing
            => HttpResponse::build(StatusCode::from_u16(400).unwrap())
                .json(ErrorResponse::from_title(24,
                                                String::from("The ToDate is missing"))),

            HazizzError::KretaFromDateMissing
            => HttpResponse::build(StatusCode::from_u16(400).unwrap())
                .json(ErrorResponse::from_title(23,
                                                String::from("The FromDate is missing"))),

            HazizzError::KretaBadResponse(err)
            => HttpResponse::build(StatusCode::from_u16(500).unwrap())
                .json(ErrorResponse::from_message(21,
                                                  String::from("Unrecognisable response from kreta server"),
                                                  err.description().to_string())),

            HazizzError::KretaRequestSendFailed(err)
            => HttpResponse::build(StatusCode::from_u16(500).unwrap())
                .json(ErrorResponse::from_message(20,
                                                  String::from("Request failed"),
                                                  err.description().to_string())),
        }
    }
}