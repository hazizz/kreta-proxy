use serde::{Deserialize, Serialize};

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
    count: u8,
    date: String,
    start_time: String,
    end_time: String,
    nev: String,
    class_room: String,
    class_group: String,
    teacher: String,
    state_name: String,
    theme: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    subject: String,
    pub date: String,
    start_of_class: String,
    end_of_class: String,
    period_number: u8,
    cancelled: bool,
    stand_in: bool,
    class_name: String,
    teacher: String,
    room: String,
    topic: String,
}

pub fn refine_schedule(unrefined: UnrefinedLesson) -> Lesson {
    Lesson {
        period_number: unrefined.count,
        cancelled: unrefined.state_name.contains("Elmaradt"),
        stand_in: unrefined.teacher.contains("Helyettes"),
        class_name: unrefined.class_group,
        teacher: {
            if unrefined.teacher.contains("Helyettes") {
                let words: Vec<&str> = unrefined.teacher.split(":").collect();
                words.get(1).expect("Expected a TEACHER after split")
                    .trim().to_string()
            } else {
                unrefined.teacher
            }
        },
        subject: unrefined.nev,
        date: {
            let words: Vec<&str> = unrefined.date.split("T").collect();
            words.get(0).expect("Expected a date before the T").to_string()
        },
        start_of_class: {
            let words: Vec<&str> = unrefined.start_time.split("T").collect();
            words.get(1).expect("Expected a time after the T").to_string()
        },
        end_of_class: {
            let words: Vec<&str> = unrefined.end_time.split("T").collect();
            words.get(1).expect("Expected a time after the T").to_string()
        },
        room: unrefined.class_room,
        topic: unrefined.theme,
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedProfile {
    name: String,
    institute_name: String,
    student_id: u64,
    pub evaluations: Vec<UnrefinedGrade>,
    pub subject_averages: Vec<UnrefinedAverage>,
    pub notes: Vec<UnrefinedNote>,
    pub form_teacher: UnrefinedFormTeacher,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedGrade {
    subject: Option<String>,
    theme: Option<String>,
    weight: Option<String>,
    r#type: String,
    number_value: u8,
    value: String,
    teacher: String,
    date: String,
    creating_time: String,
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

pub fn refine_grades(unrefined: UnrefinedGrade) -> Grade {
    Grade {
        subject: unrefined.subject.unwrap_or("-".to_string()),
        grade_type: unrefined.r#type,
        grade: if unrefined.number_value == 0 {
            unrefined.value
        } else {
            format!("{}", unrefined.number_value)
        },
        date: {
            let words: Vec<&str> = unrefined.date.split("T").collect();
            words.get(0).expect("Expected a date before the T").to_string()
        },
        creation_date: {
            let words: Vec<&str> = unrefined.creating_time.split("T").collect();
            words.get(0).expect("Expected a date before the T").to_string()
        },
        weight: {
            unrefined.weight.unwrap_or("0".to_string()).replace("%", "").parse().unwrap_or(0)
        },
        topic: unrefined.theme.unwrap_or("-".to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedAverage {
    subject: String,
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

pub fn refine_average(average: UnrefinedAverage) -> Average {
    Average {
        subject: average.subject.clone(),
        grade: average.value,
        class_grade: average.class_value,
        difference: average.difference,
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedNote {
    note_id: u64,
    r#type: String,
    title: String,
    content: String,
    teacher: String,
    creating_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: u64,
    r#type: String,
    title: String,
    content: String,
    teacher: String,
    creation_date: String,
}

pub fn refine_note(unrefined: UnrefinedNote) -> Note {
    Note {
        id: unrefined.note_id,
        r#type: unrefined.r#type,
        title: unrefined.title,
        content: unrefined.content,
        teacher: unrefined.teacher,
        creation_date: {
            let words: Vec<&str> = unrefined.creating_time.split("T").collect();
            words.get(0).expect("Expected a date before the T").to_string()
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedFormTeacher {
    teacher_id: u64,
    name: String,
    email: String,
    phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormTeacher {
    id: u64,
    name: String,
    email: String,
}

pub fn refine_form_teacher(unrefined: UnrefinedFormTeacher) -> FormTeacher {
    FormTeacher {
        id: unrefined.teacher_id,
        name: unrefined.name,
        email: unrefined.email,
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedTask {
    id: u64,
    datum: String,
    bejelentes_datuma: String,
    tantargy: String,
    tanar: String,
    szamonkeres_megnevezese: String,
    szamonkeres_modja: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: u64,
    subject: String,
    topic: String,
    grade_type: String,
    teacher: String,
    date: String,
    creation_date: String,
}

pub fn refine_task(unrefined: UnrefinedTask) -> Task {
    Task {
        date: {
            let words: Vec<&str> = unrefined.datum.split("T").collect();
            words.get(0).expect("Expected a date before the T").to_string()
        },
        creation_date: {
            let words: Vec<&str> = unrefined.bejelentes_datuma.split("T").collect();
            words.get(0).expect("Expected a date before the T").to_string()
        },
        subject: unrefined.tantargy,
        teacher: unrefined.tanar,
        id: unrefined.id,
        topic: unrefined.szamonkeres_megnevezese,
        grade_type: unrefined.szamonkeres_modja,
    }
}