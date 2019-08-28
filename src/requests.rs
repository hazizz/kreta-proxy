use reqwest::header::{HeaderMap, HeaderValue};
use crate::resources::*;
use crate::*;

pub fn create_token(url: &String, username: String, password: String) -> Result<Authentication, actix_web::Error>{
    let body = format!("institute_code={}&userName={}&password={}&grant_type=password&client_id=919e0c1c-76a2-4646-a2fb-7085bbbf3c56", url, username, password);

    let url = format!("https://{}.e-kreta.hu/idp/api/v1/Token", url);
    let client = reqwest::Client::new();

    let resp: Authentication = client.get(&url)
        .body(body)
        .send().map_err(create_client_error)?
        .json().map_err(create_json_error)?;

    Ok(resp)
}

pub fn get_schools() -> Result<Vec<School>, actix_web::Error>{
    let mut headers = HeaderMap::new();
    headers.append("apiKey", HeaderValue::from_static("7856d350-1fda-45f5-822d-e1a2f3f1acf0"));

    let client = reqwest::Client::new();

    let request = client.get("https://kretaglobalmobileapi.ekreta.hu/api/v1/Institute")
        .headers(headers);

    let schools: Vec<School> = request
        .send().map_err(create_client_error)?
        .json().map_err(create_json_error)?;

    Ok(schools)
}


pub fn get_schedule(token: String, url: &String, from_date: String, to_date: String) -> Result<Vec<Lesson>, actix_web::Error>{
    let url = format!("https://{}.e-kreta.hu/mapi/api/v1/Lesson?fromDate={}&toDate={}", url, from_date, to_date);
    let client = reqwest::Client::new();

    let resp: Vec<UnrefinedLesson> = client.get(&url)
        .bearer_auth(token)
        .send().map_err(create_client_error)?
        .json().map_err(create_json_error)?;

    let mut lessons: Vec<Lesson> = Vec::new();

    for unrefined in resp {
        let refined = refine_schedule(unrefined);
        lessons.push(refined);
    }

    Ok(lessons)
}


pub fn get_grades(token: String, url: &String) -> Result<BTreeMap<String, Vec<Grade>>, actix_web::Error>{
    let mut grades: BTreeMap<String, Vec<Grade>> = BTreeMap::new();
    let mut subjects: Vec<String> = Vec::new();

    let profile = get_profile(token, &url)?;

    for grade in profile.evaluations {
        let refined = refine_grades(grade);

        let vec = grades.entry(refined.subject.clone()).or_insert(Vec::new());

        subjects.push(refined.subject.clone());
        vec.push(refined);
    }

    for (_, val) in grades.iter_mut(){
        val.sort_by(|a, b| a.date.cmp(&b.date));
    }

    Ok(grades)
}

pub fn get_notes(token: String, url: &String) -> Result<Vec<Note>, actix_web::Error>{
    let mut notes: Vec<Note> = Vec::new();

    let profile = get_profile(token, &url)?;

    for note in profile.notes {
        let refined = refine_note(note);
        notes.push(refined);
    }

    Ok(notes)
}

pub fn get_averages(token: String, url: &String) -> Result<Vec<Average>, actix_web::Error>{
    let mut averages: Vec<Average> = Vec::new();

    let profile = get_profile(token, &url)?;

    for average in profile.subject_averages {
        let refined = refine_average(average);
        averages.push(refined);
    }

    Ok(averages)
}

fn get_profile(token: String, url: &String) -> Result<UnrefinedProfile, actix_web::Error>{
    let url = format!("https://{}.e-kreta.hu/mapi/api/v1/Student", url);
    let client = reqwest::Client::new();
    let profile: UnrefinedProfile = client.get(&url)
        .bearer_auth(token)
        .send().map_err(create_client_error)?
        .json().map_err(create_json_error)?;
    return Ok(profile);
}

pub fn get_tasks(token: String, url: &String, from_date: String, to_date: String) -> Result<Vec<Task>, actix_web::Error> {
    let url = format!("https://{}.e-kreta.hu/mapi/api/v1/BejelentettSzamonkeres?DatumTol={}&DatumIg={}",
                      url,
                      from_date,
                      to_date);
    let client = reqwest::Client::new();

    let mut tasks: Vec<Task> = Vec::new();

    let resp: Vec<UnrefinedTask> = client.get(&url)
        .bearer_auth(token)
        .send().map_err(create_client_error)?
        .json().map_err(create_json_error)?;

    for unrefined in resp{
        tasks.push(refine_task(unrefined));
    }

    Ok(tasks)
}


#[cfg(test)]
mod requests_integration_test {
    use super::*;

    fn get_username() -> String{
        match std::env::var("USERNAME"){
            Ok(username) => username,
            Err(_err) => panic!("Username not specified!"),
        }
    }

    fn get_password() -> String{
        match std::env::var("PASSWORD"){
            Ok(password) => password,
            Err(_err) => panic!("Password not specified!"),
        }
    }

    fn get_url() -> String{
        match std::env::var("SCHOOL_URL"){
            Ok(school_url) => school_url,
            Err(_err) => panic!("School url not specified!"),
        }
    }

    fn get_token() -> String{
        create_token(&get_url(), get_username(), get_password()).unwrap().access_token
    }

    #[test]
    fn test_schedules() {
        let schedules = get_schedule(get_token(), &get_url(), String::from("2019-04-22"), String::from("2019-04-27"));
        assert!(schedules.is_ok());
    }

    #[test]
    fn test_schedules_v2() {
        let schedules = get_schedule_v2(get_token(), &get_url(), String::from("2019-04-22"), String::from("2019-04-27"));
        assert!(schedules.is_ok());
    }

    #[test]
    fn test_grades() {
        let grades = get_grades(get_token(), &get_url());
        assert!(grades.is_ok());
    }

    #[test]
    fn test_schools(){
        let schools = get_schools();
        println!("Schools: {:?}", &schools);
    }

    #[test]
    fn test_tasks(){
        let tasks = get_tasks(get_token(), &get_url(), String::from("2019-06-02"), String::from("2019-06-10"));
        assert!(tasks.is_ok());
    }

    #[test]
    fn test_notes(){
        let notes = get_notes(get_token(), &get_url());
        assert!(notes.is_ok());
    }

    #[test]
    fn test_averages(){
        let averages = get_averages(get_token(), &get_url());
        assert!(averages.is_ok());
    }

}