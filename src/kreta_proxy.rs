use std::collections::BTreeMap;
use std::time::Instant;

use actix_web::*;
use chrono::{Date, Datelike, NaiveDate, Utc};
use http::StatusCode;
use log::info;
use serde::Deserialize;

use crate::error::KretaError;
use crate::requests::*;
use crate::resources::*;

mod error;
mod requests;
mod resources;

#[derive(Debug, Deserialize)]
pub struct TokenCreationQuery {
    url: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct GeneralQuery {
    token: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct DateBasedQuery {
    token: String,
    url: String,
    #[serde(default)]
    from_date: String,
    #[serde(default)]
    to_date: String,
}

impl Default for DateBasedQuery {
    fn default() -> Self {
        let now: Date<_> = Utc::now().date();
        DateBasedQuery {
            url: String::from(""),
            token: String::from(""),
            from_date: now.format("%Y-%m-%d").to_string(),
            to_date: now
                .with_year(now.year() + 1)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
        }
    }
}

async fn handle_school_request(_req: HttpRequest) -> Result<HttpResponse, KretaError> {
    let schools: Vec<School> = get_schools().await?;

    Ok(HttpResponse::build(StatusCode::from_u16(200).unwrap()).json(schools))
}

#[actix_web::get("/grades")]
async fn handle_grades_request(
    query: web::Query<GeneralQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let grades = get_grades(&query.token, &query.url).await?;

    info!(
        "Grade request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(grades))
}

#[actix_web::get("/notes")]
async fn handle_notes_request(query: web::Query<GeneralQuery>) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let notes = get_notes(&query.token, &query.url).await?;

    info!(
        "Notes request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(notes))
}

#[actix_web::get("/averages")]
async fn handle_averages_request(
    query: web::Query<GeneralQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let averages = get_averages(&query.token, &query.url).await?;

    info!(
        "Averages request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(averages))
}

#[actix_web::get("/v2/schedules")]
async fn handle_schedule_request_v2(
    query: web::Query<DateBasedQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let lessons_sorted = get_schedule_v2(
        query.token.clone(),
        query.url.clone(),
        query.from_date.clone(),
        query.to_date.clone(),
    )
    .await?;

    info!(
        "Schedule V2 request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(lessons_sorted))
}

#[actix_web::get("/schedules")]
async fn handle_schedule_request(
    query: web::Query<DateBasedQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let lessons: Vec<Lesson> = get_schedule(
        query.token.clone(),
        query.url.clone(),
        query.from_date.clone(),
        query.to_date.clone(),
    )
    .await?;

    info!(
        "Schedule request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(lessons))
}

#[actix_web::get("/tasks")]
async fn handle_tasks_request(
    query: web::Query<DateBasedQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let tasks = get_tasks(&query.token, &query.url, &query.from_date, &query.to_date).await?;

    info!(
        "Tasks request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(tasks))
}

#[actix_web::get("/profile")]
async fn handle_profile_request(
    query: web::Query<GeneralQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let profile = get_profile(&query.token, &query.url).await?.refine();

    info!(
        "Profile request done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(profile))
}

#[actix_web::post("/token")]
async fn handle_create_token(
    query: web::Query<TokenCreationQuery>,
) -> Result<HttpResponse, KretaError> {
    let request_started = Instant::now();

    let lessons_sorted = create_token(&query.url, &query.username, &query.password).await?;

    info!(
        "Token creation done for {} in {}",
        &query.url,
        request_started.elapsed().as_millis()
    );

    Ok(HttpResponse::build(StatusCode::OK).json(lessons_sorted))
}

#[actix_rt::main]
async fn main() {
    let port: u32 = match std::env::var("SERVER_PORT") {
        Ok(port_string) => match port_string.parse() {
            Ok(port_parsed) => port_parsed,
            Err(_err) => {
                println!("port {} couldn't be parsed!", port_string);
                return;
            }
        },
        Err(_err) => {
            println!("server.port variable isn't present, defaulting to 9110");
            9110
        }
    };

    let address: String = match std::env::var("SERVER_ADDRESS") {
        Ok(address) => address,
        Err(_err) => String::from("127.0.0.1"),
    };

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let server = HttpServer::new(|| {
        App::new()
            .service(handle_grades_request)
            .service(handle_notes_request)
            .service(handle_averages_request)
            .service(handle_schedule_request)
            .service(handle_schedule_request_v2)
            .service(handle_tasks_request)
            .service(handle_profile_request)
            .service(handle_create_token)
    })
    .bind(format!("{}:{}", &address, &port))
    .unwrap()
    .run();

    println!("Server bind to {} with port {}!", &address, &port);

    let _ = server.await;
}
