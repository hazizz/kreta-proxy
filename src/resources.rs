use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KretaErrorResponse {
    pub error: String,
    pub error_code: String,
    pub error_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Authentication {
    pub access_token: String,
    token_type: String,
    expires_in: u16,
    refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct School {
    institute_id: u32,
    institute_code: String,
    name: String,
    url: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedLesson {
    count: i8,
    date: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    nev: Option<String>,
    class_room: Option<String>,
    class_group: Option<String>,
    teacher: Option<String>,
    state_name: Option<String>,
    theme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    subject: String,
    pub date: String,
    start_of_class: String,
    end_of_class: String,
    period_number: i8,
    cancelled: bool,
    stand_in: bool,
    class_name: String,
    teacher: String,
    room: String,
    topic: String,
}

impl UnrefinedLesson {
    pub fn refine(self) -> Lesson {
        Lesson {
            period_number: self.count,
            cancelled: self
                .state_name
                .map(|state_name| state_name.contains("Elmaradt"))
                .unwrap_or(false),
            stand_in: self
                .teacher
                .clone()
                .map(|teacher| teacher.contains("Helyettes"))
                .unwrap_or(false),
            class_name: self.class_group.unwrap_or(String::from("-")),
            teacher: self
                .teacher
                .map(|teacher| {
                    if teacher.contains("Helyettes") {
                        let words: Vec<&str> = teacher.split(":").collect();
                        words
                            .get(1)
                            .expect("Expected a TEACHER after split")
                            .trim()
                            .to_string()
                    } else {
                        teacher
                    }
                })
                .unwrap_or(String::from("-")),
            subject: self.nev.unwrap_or(String::from("-")),
            date: self
                .date
                .map(strip_time_date_to_date)
                .unwrap_or(String::from("1999-09-19")),
            start_of_class: self
                .start_time
                .map(strip_time_date_to_time)
                .unwrap_or(String::from("09:00:00")),
            end_of_class: self
                .end_time
                .map(strip_time_date_to_time)
                .unwrap_or(String::from("09:45:00")),
            room: self.class_room.unwrap_or(String::from("-")),
            topic: self.theme.unwrap_or(String::from("-")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedProfile {
    name: Option<String>,
    institute_name: Option<String>,
    student_id: u64,
    pub evaluations: Option<Vec<UnrefinedGrade>>,
    pub subject_averages: Option<Vec<UnrefinedAverage>>,
    pub notes: Option<Vec<UnrefinedNote>>,
    pub form_teacher: Option<UnrefinedFormTeacher>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    id: u64,
    name: String,
    school_name: String,
    pub grades: Vec<Grade>,
    pub averages: Vec<Average>,
    pub notes: Vec<Note>,
    form_teacher: Option<FormTeacher>,
}

impl UnrefinedProfile {
    pub fn refine(self) -> Profile {
        Profile {
            name: self.name.unwrap_or(String::from("-")),
            school_name: self.institute_name.unwrap_or(String::from("-")),
            id: self.student_id,
            grades: self
                .evaluations
                .map(|evaluations| {
                    let mut refined = Vec::new();
                    for x in evaluations {
                        refined.push(x.refine());
                    }
                    refined
                })
                .unwrap_or(Vec::new()),
            averages: self
                .subject_averages
                .map(|averages| {
                    let mut refined = Vec::new();
                    for x in averages {
                        refined.push(x.refine());
                    }
                    refined
                })
                .unwrap_or(Vec::new()),
            notes: self
                .notes
                .map(|notes| {
                    let mut refined = Vec::new();
                    for x in notes {
                        refined.push(x.refine());
                    }
                    refined
                })
                .unwrap_or(Vec::new()),
            form_teacher: self.form_teacher.map(|form| form.refine()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedGrade {
    subject: Option<String>,
    theme: Option<String>,
    weight: Option<String>,
    r#type: Option<String>,
    number_value: u8,
    value: Option<String>,
    teacher: Option<String>,
    date: Option<String>,
    creating_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    pub subject: String,
    grade_type: String,
    grade: String,
    pub date: String,
    creation_date: String,
    weight: u8,
    topic: String,
}

impl UnrefinedGrade {
    pub fn refine(self) -> Grade {
        Grade {
            subject: self.subject.unwrap_or(String::from("-")),
            grade_type: self.r#type.unwrap_or(String::from("-")),
            grade: if self.number_value == 0 {
                self.value.unwrap_or(String::from("-"))
            } else {
                format!("{}", self.number_value)
            },
            date: self
                .date
                .map(strip_time_date_to_date)
                .unwrap_or(String::from("1999-09-19")),
            creation_date: self
                .creating_time
                .unwrap_or(String::from("1999-09-19T00:00:00")),
            weight: self
                .weight
                .map(|weight| weight.replace("%", "").parse().unwrap_or(0))
                .unwrap_or(0),
            topic: self.theme.unwrap_or(String::from("-")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedAverage {
    subject: Option<String>,
    value: f64,
    class_value: f64,
    difference: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Average {
    subject: String,
    grade: f64,
    class_grade: f64,
    difference: f64,
}

impl UnrefinedAverage {
    pub fn refine(self) -> Average {
        Average {
            subject: self.subject.unwrap_or(String::from("-")),
            grade: self.value,
            class_grade: self.class_value,
            difference: self.difference,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedNote {
    note_id: i64,
    r#type: Option<String>,
    title: Option<String>,
    content: Option<String>,
    teacher: Option<String>,
    creating_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: i64,
    r#type: String,
    title: String,
    content: String,
    teacher: String,
    creation_date: String,
}

impl UnrefinedNote {
    pub fn refine(self) -> Note {
        Note {
            id: self.note_id,
            r#type: self.r#type.unwrap_or(String::from("-")),
            title: self.title.unwrap_or(String::from("-")),
            content: self.content.unwrap_or(String::from("-")),
            teacher: self.teacher.unwrap_or(String::from("-")),
            creation_date: self
                .creating_time
                .unwrap_or(String::from("1999-09-19T00:00:00")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedFormTeacher {
    teacher_id: i64,
    name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormTeacher {
    id: i64,
    name: String,
    email: String,
    phone_number: String,
}

impl UnrefinedFormTeacher {
    pub fn refine(self) -> FormTeacher {
        FormTeacher {
            id: self.teacher_id,
            name: self.name.unwrap_or(String::from("-")),
            email: self.email.unwrap_or(String::from("-")),
            phone_number: self.phone_number.unwrap_or(String::from("-")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedTask {
    id: i64,
    datum: Option<String>,
    bejelentes_datuma: Option<String>,
    tantargy: Option<String>,
    tanar: Option<String>,
    szamonkeres_megnevezese: Option<String>,
    szamonkeres_modja: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: i64,
    subject: String,
    topic: String,
    grade_type: String,
    teacher: String,
    date: String,
    creation_date: String,
}

impl UnrefinedTask {
    pub fn refine(self) -> Task {
        Task {
            id: self.id,
            date: self
                .datum
                .map(strip_time_date_to_date)
                .unwrap_or(String::from("1999-09-19")),
            creation_date: {
                self.bejelentes_datuma
                    .map(strip_time_date_to_date)
                    .unwrap_or(String::from("1999-09-19"))
            },
            subject: self.tantargy.unwrap_or(String::from("-")),
            teacher: self.tanar.unwrap_or(String::from("-")),
            topic: self.szamonkeres_megnevezese.unwrap_or(String::from("-")),
            grade_type: self.szamonkeres_modja.unwrap_or(String::from("-")),
        }
    }
}

fn strip_time_date_to_date(mut time_date: String) -> String {
    use chrono_tz::Europe::Budapest;

    if !time_date.ends_with("Z") {
        time_date.push_str("Z");
    }
    DateTime::parse_from_rfc3339(&time_date)
        .map(|date| {
            let new_date = date.with_timezone(&Budapest);
            new_date.format("%Y-%m-%d").to_string()
        })
        .unwrap()
}
fn strip_time_date_to_time(mut time_date: String) -> String {
    use chrono::offset::TimeZone;
    use chrono_tz::Europe::Budapest;

    if !time_date.ends_with("Z") {
        time_date.push_str("Z");
    }
    DateTime::parse_from_rfc3339(&time_date)
        .map(|date| {
            let new_date = Budapest.from_local_datetime(&date.naive_local()).unwrap();
            new_date.format("%H:%M:%S").to_string()
        })
        .unwrap()
}
