use actix_web::{*};
use std::collections::{BTreeMap};
use actix_web::server::HttpServer;
use http::StatusCode;
use std::time::Instant;
use log::info;
use actix_web::http::{Method};
use chrono::{NaiveDate, Datelike, Utc, Date};
use crate::resources::*;
use crate::requests::*;
use crate::error::HazizzError;

mod resources;
mod requests;
mod error;

fn handle_create_token(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let username_opt = params.get("username");
    let username: String = username_opt.ok_or(HazizzError::KretaUsernameMissing)?.clone();

    let password_opt = params.get("password");
    let password: String = password_opt.ok_or(HazizzError::KretaPasswordMissing)?.clone();

    let lessons_sorted = create_token(&url, username, password)?;

    info!("Token creation done for {} in {}",
          url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(lessons_sorted))
}

fn handle_school_request(_req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let schools: Vec<School> = get_schools()?;

    Ok(HttpResponse::build(StatusCode::from_u16(200).unwrap())
        .json(schools))
}

fn handle_schedule_request_v2(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let token_opt = params.get("token");
    let token: String = token_opt.ok_or(HazizzError::KretaTokenMissing)?.clone();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let to_date_opt = params.get("to_date");
    let to_date: String = to_date_opt.ok_or(HazizzError::KretaToDateMissing)?.clone();

    let from_date_opt = params.get("from_date");
    let from_date: String = from_date_opt.ok_or(HazizzError::KretaFromDateMissing)?.clone();

    let lessons_sorted = get_schedule_v2(token, &url, from_date, to_date)?;

    info!("Schedule V2 request done for {} in {}",
          url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(lessons_sorted))
}

fn get_schedule_v2(token: String, url: &String, from_date: String, to_date: String)
    -> Result<BTreeMap<String, Vec<Lesson>>, actix_web::Error>{

    let lessons: Vec<Lesson> = get_schedule(token, &url, from_date, to_date)?;
    let mut lessons_sorted: BTreeMap<String, Vec<Lesson>> = BTreeMap::new();

    for lesson in lessons{
        let date = NaiveDate::parse_from_str(&lesson.date,"%Y-%m-%d").unwrap();
        let week_number: String = format!("{}", date.weekday().num_days_from_monday());
        let entry = lessons_sorted.entry(week_number).or_insert(Vec::new());
        entry.push(lesson);
    }

    Ok(lessons_sorted)
}

fn handle_schedule_request(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let token_opt = params.get("token");
    let token: String = token_opt.ok_or(HazizzError::KretaTokenMissing)?.clone();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let to_date_opt = params.get("to_date");
    let to_date: String = to_date_opt.ok_or(HazizzError::KretaToDateMissing)?.clone();

    let from_date_opt = params.get("from_date");
    let from_date: String = from_date_opt.ok_or(HazizzError::KretaFromDateMissing)?.clone();

    let lessons: Vec<Lesson> = get_schedule(token, &url, from_date, to_date)?;

    info!("Schedule request done for {} in {}",
          url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(lessons))
}

fn handle_grades_request(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let token_opt = params.get("token");
    let token: String = token_opt.ok_or(HazizzError::KretaTokenMissing)?.clone();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let grades = get_grades(token, &url)?;

    info!("Grade request done for {} in {}",
          &url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(grades))
}

fn handle_notes_request(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let token_opt = params.get("token");
    let token: String = token_opt.ok_or(HazizzError::KretaTokenMissing)?.clone();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let notes = get_notes(token, &url)?;

    info!("Notes request done for {} in {}",
          &url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(notes))
}

fn handle_averages_request(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let token_opt = params.get("token");
    let token: String = token_opt.ok_or(HazizzError::KretaTokenMissing)?.clone();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let averages = get_averages(token, &url)?;

    info!("Averages request done for {} in {}",
          &url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(averages))
}

fn handle_tasks_request(req: &HttpRequest) -> Result<HttpResponse, actix_web::Error>{
    let request_started = Instant::now();

    let params = req.query();

    let token_opt = params.get("token");
    let token: String = token_opt.ok_or(HazizzError::KretaTokenMissing)?.clone();

    let url_opt = params.get("url");
    let url: String = url_opt.ok_or(HazizzError::KretaUrlMissing)?.clone();

    let now: Date<_> = Utc::now().date();

    let from_date_opt = params.get("from_date");
    let from_date = if from_date_opt.is_some(){ from_date_opt.unwrap().clone()}
    else { now.format("%Y-%m-%d").to_string() };

    let now = NaiveDate::parse_from_str(&from_date, "%Y-%m-%d").unwrap();

    let to_date_opt = params.get("to_date");
    let to_date = if to_date_opt.is_some(){ to_date_opt.unwrap().clone() }
                    else { now
                        .with_year(now.year() + 1).unwrap()
                        .format("%Y-%m-%d").to_string() };

    let tasks = get_tasks(token, &url, from_date, to_date)?;

    info!("Tasks request done for {} in {}",
          &url,
          request_started.elapsed().as_millis());

    Ok(HttpResponse::build(StatusCode::OK)
        .json(tasks))
}

//noinspection RsTypeCheck
fn create_client_error(error: reqwest::Error) -> HazizzError {
    HazizzError::KretaRequestSendFailed(error)
}

//noinspection RsTypeCheck
fn create_json_error(error: reqwest::Error) -> HazizzError{
    HazizzError::KretaBadResponse(error)
}


fn main() {
    let sys = actix::System::new("kreta-helper");

    let port: u32 = match std::env::var("SERVER_PORT"){
        Ok(port_string) => match port_string.parse(){
            Ok(port_parsed) => port_parsed,
            Err(_err) =>{
                println!("port {} couldn't be parsed!", port_string);
                return;
            }
        },
        Err(_err) => {
            println!("server.port variable isn't present, defaulting to 9101");
            9101
        },
    };

    let address: String = match std::env::var("SERVER_ADDRESS"){
        Ok(address) => address,
        Err(_err) => String::from("127.0.0.1"),
    };

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(
        || App::new()
            .resource("/grades", |r| r.method(Method::GET).f(handle_grades_request))

            .resource("/notes", |r| r.method(Method::GET).f(handle_notes_request))

            .resource("/averages", |r| r.method(Method::GET).f(handle_averages_request))

            .resource("/schedules", |r| r.method(Method::GET).f(handle_schedule_request))

            .resource("/v2/schedules", |r| r.method(Method::GET).f(handle_schedule_request_v2))

            .resource("/token", |r| r.method(Method::POST).f(handle_create_token))

            .resource("/tasks", |r| r.method(Method::GET).f(handle_tasks_request))

            .finish())
        .bind(format!("{}:{}", &address, &port))
        .expect("Server couldn't start up")
        .start();

    println!("Server bind to {} with port {}!", &address, &port);

    let _ = sys.run();
}