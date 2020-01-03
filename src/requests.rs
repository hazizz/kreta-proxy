use reqwest::header::{HeaderMap, HeaderValue};

use crate::*;

pub async fn create_token(
    url: &str,
    username: &str,
    password: &str,
) -> Result<Authentication, KretaError> {
    let body = format!("institute_code={}&userName={}&password={}&grant_type=password&client_id=919e0c1c-76a2-4646-a2fb-7085bbbf3c56", url, username, password);

    let url = format!("https://{}.e-kreta.hu/idp/api/v1/Token", url);
    let client = reqwest::Client::new();

    let resp: Authentication = parse_body(client.get(&url).body(body).send().await).await?;
    Ok(resp)
}

pub async fn get_schools() -> Result<Vec<School>, KretaError> {
    let mut headers = HeaderMap::new();
    headers.append(
        "apiKey",
        HeaderValue::from_static("7856d350-1fda-45f5-822d-e1a2f3f1acf0"),
    );

    let client = reqwest::Client::new();

    let request = client
        .get("https://kretaglobalmobileapi.ekreta.hu/api/v1/Institute")
        .headers(headers);

    let schools: Vec<School> = parse_body(request.send().await).await?;

    Ok(schools)
}

pub async fn get_schedule_v2(
    token: String,
    url: String,
    from_date: String,
    to_date: String,
) -> Result<BTreeMap<String, Vec<Lesson>>, KretaError> {
    let lessons: Vec<Lesson> = get_schedule(token, url, from_date, to_date).await?;
    let mut lessons_sorted: BTreeMap<String, Vec<Lesson>> = BTreeMap::new();

    for lesson in lessons {
        let date = NaiveDate::parse_from_str(&lesson.date, "%Y-%m-%d").unwrap();
        let week_number: String = format!("{}", date.weekday().num_days_from_monday());
        let entry = lessons_sorted.entry(week_number).or_insert(Vec::new());
        entry.push(lesson);
    }

    Ok(lessons_sorted)
}

pub async fn get_schedule(
    token: String,
    url: String,
    from_date: String,
    to_date: String,
) -> Result<Vec<Lesson>, KretaError> {
    let url = format!(
        "https://{}.e-kreta.hu/mapi/api/v1/Lesson?fromDate={}&toDate={}",
        url, from_date, to_date
    );
    let client = reqwest::Client::new();

    let resp: Vec<UnrefinedLesson> =
        parse_body(client.get(&url).bearer_auth(token).send().await).await?;

    let mut lessons: Vec<Lesson> = Vec::new();

    for unrefined in resp {
        let refined = unrefined.refine();
        lessons.push(refined);
    }

    Ok(lessons)
}

pub async fn get_grades(
    token: &str,
    url: &str,
) -> Result<BTreeMap<String, Vec<Grade>>, KretaError> {
    let mut grades: BTreeMap<String, Vec<Grade>> = BTreeMap::new();
    let mut subjects: Vec<String> = Vec::new();

    let profile = get_profile(token, &url).await?.refine();

    for grade in profile.grades {
        let vec = grades.entry(grade.subject.clone()).or_insert(Vec::new());

        subjects.push(grade.subject.clone());
        vec.push(grade);
    }

    for (_, val) in grades.iter_mut() {
        val.sort_by(|a, b| a.date.cmp(&b.date));
    }

    Ok(grades)
}

pub async fn get_notes(token: &str, url: &str) -> Result<Vec<Note>, KretaError> {
    let profile = get_profile(token, &url).await?.refine();
    Ok(profile.notes)
}

pub async fn get_averages(token: &str, url: &str) -> Result<Vec<Average>, KretaError> {
    let profile = get_profile(token, &url).await?.refine();
    Ok(profile.averages)
}

pub async fn get_profile(token: &str, url: &str) -> Result<UnrefinedProfile, KretaError> {
    let url = format!("https://{}.e-kreta.hu/mapi/api/v1/Student", url);
    let client = reqwest::Client::new();
    let profile: UnrefinedProfile =
        parse_body(client.get(&url).bearer_auth(token).send().await).await?;
    return Ok(profile);
}

pub async fn get_tasks(
    token: &str,
    url: &str,
    from_date: &str,
    to_date: &str,
) -> Result<Vec<Task>, KretaError> {
    let url = format!(
        "https://{}.e-kreta.hu/mapi/api/v1/BejelentettSzamonkeres?DatumTol={}&DatumIg={}",
        url, from_date, to_date
    );
    let client = reqwest::Client::new();

    let mut tasks: Vec<Task> = Vec::new();

    let resp: Vec<UnrefinedTask> =
        parse_body(client.get(&url).bearer_auth(token).send().await).await?;

    for unrefined in resp {
        tasks.push(unrefined.refine());
    }

    Ok(tasks)
}

async fn parse_body<T>(result: Result<reqwest::Response, reqwest::Error>) -> Result<T, KretaError>
where
    T: serde::de::DeserializeOwned,
{
    match result {
        Err(err) => Err(KretaError::KretaRequestSendFailed(err)),
        Ok(response) => response
            .json()
            .await
            .map_err(|err| KretaError::KretaBadResponse(err)),
    }
}

#[cfg(test)]
mod requests_integration_test {
    use super::*;

    fn get_username() -> String {
        match std::env::var("USERNAME") {
            Ok(username) => username,
            Err(_err) => panic!("Username not specified!"),
        }
    }

    fn get_password() -> String {
        match std::env::var("PASSWORD") {
            Ok(password) => password,
            Err(_err) => panic!("Password not specified!"),
        }
    }

    fn get_url() -> String {
        match std::env::var("SCHOOL_URL") {
            Ok(school_url) => school_url,
            Err(_err) => panic!("School url not specified!"),
        }
    }

    async fn get_token() -> String {
        create_token(&get_url(), &get_username(), &get_password())
            .await
            .unwrap()
            .access_token
    }

    #[tokio::test]
    async fn test_schedules() {
        let schedules = get_schedule(
            get_token().await,
            get_url(),
            String::from("2019-04-22"),
            String::from("2019-04-27"),
        )
        .await;
        assert!(&schedules.is_ok(), schedules);
    }

    #[tokio::test]
    async fn test_schedules_v2() {
        let schedules = get_schedule_v2(
            get_token().await,
            get_url(),
            String::from("2019-04-22"),
            String::from("2019-04-27"),
        )
        .await;
        assert!(&schedules.is_ok(), schedules);
    }

    #[tokio::test]
    async fn test_grades() {
        let grades = get_grades(&get_token().await, &get_url()).await;
        assert!(&grades.is_ok(), grades);
    }

    #[tokio::test]
    async fn test_schools() {
        let schools = get_schools().await;
        assert!(&schools.is_err(), schools);
    }

    #[tokio::test]
    async fn test_tasks() {
        let tasks = get_tasks(&get_token().await, &get_url(), "2019-06-02", "2019-06-10").await;
        assert!(&tasks.is_ok(), tasks);
    }

    #[tokio::test]
    async fn test_notes() {
        let notes = get_notes(&get_token().await, &get_url()).await;
        assert!(&notes.is_ok(), notes);
    }

    #[tokio::test]
    async fn test_averages() {
        let averages = get_averages(&get_token().await, &get_url()).await;
        assert!(&averages.is_ok(), averages);
    }
}
