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
    theme: Option<String>,
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
    topic: Option<String>,
}

impl UnrefinedLesson {
    pub fn refine(self) -> Lesson {
        Lesson {
            period_number: self.count,
            cancelled: self.state_name.contains("Elmaradt"),
            stand_in: self.teacher.contains("Helyettes"),
            class_name: self.class_group,
            teacher: {
                if self.teacher.contains("Helyettes") {
                    let words: Vec<&str> = self.teacher.split(":").collect();
                    words
                        .get(1)
                        .expect("Expected a TEACHER after split")
                        .trim()
                        .to_string()
                } else {
                    self.teacher
                }
            },
            subject: self.nev,
            date: {
                let words: Vec<&str> = self.date.split("T").collect();
                words
                    .get(0)
                    .expect("Expected a date before the T")
                    .to_string()
            },
            start_of_class: {
                let words: Vec<&str> = self.start_time.split("T").collect();
                words
                    .get(1)
                    .expect("Expected a time after the T")
                    .to_string()
            },
            end_of_class: {
                let words: Vec<&str> = self.end_time.split("T").collect();
                words
                    .get(1)
                    .expect("Expected a time after the T")
                    .to_string()
            },
            room: self.class_room,
            topic: self.theme,
        }
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
    pub form_teacher: Option<UnrefinedFormTeacher>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    id: u64,
    name: String,
    school_name: String,
    //grades: Vec<Grade>,
    //averages: Vec<Average>,
    //notes: Vec<Note>,
    form_teacher: Option<FormTeacher>,
}

impl UnrefinedProfile {
    pub fn refine(self) -> Profile {
        Profile {
            name: self.name,
            school_name: self.institute_name,
            id: self.student_id,
            //grades: self.evaluations.iter().map(|grade| {grade.refine()}).collect(),
            //averages: self.subject_averages.iter().map(|avg| avg.refine()).collect(),
            //notes: self.notes.iter().map(|note| note.refine()).collect(),
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

impl UnrefinedGrade {
    pub fn refine(self) -> Grade {
        Grade {
            subject: self.subject.unwrap_or("-".to_string()),
            grade_type: self.r#type,
            grade: if self.number_value == 0 {
                self.value
            } else {
                format!("{}", self.number_value)
            },
            date: {
                let words: Vec<&str> = self.date.split("T").collect();
                words
                    .get(0)
                    .expect("Expected a date before the T")
                    .to_string()
            },
            creation_date: self.creating_time,
            weight: {
                self.weight
                    .unwrap_or("0".to_string())
                    .replace("%", "")
                    .parse()
                    .unwrap_or(0)
            },
            topic: self.theme.unwrap_or("-".to_string()),
        }
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

impl UnrefinedAverage {
    pub fn refine(self) -> Average {
        Average {
            subject: self.subject.clone(),
            grade: self.value,
            class_grade: self.class_value,
            difference: self.difference,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedNote {
    note_id: u64,
    r#type: String,
    title: Option<String>,
    content: Option<String>,
    teacher: String,
    creating_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: u64,
    r#type: String,
    title: Option<String>,
    content: Option<String>,
    teacher: String,
    creation_date: String,
}

impl UnrefinedNote {
    pub fn refine(self) -> Note {
        Note {
            id: self.note_id,
            r#type: self.r#type,
            title: self.title,
            content: self.content,
            teacher: self.teacher,
            creation_date: self.creating_time,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedFormTeacher {
    teacher_id: u64,
    name: String,
    email: Option<String>,
    phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormTeacher {
    id: u64,
    name: String,
    email: Option<String>,
}

impl UnrefinedFormTeacher {
    pub fn refine(self) -> FormTeacher {
        FormTeacher {
            id: self.teacher_id,
            name: self.name,
            email: self.email,
        }
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

impl UnrefinedTask {
    pub fn refine(self) -> Task {
        Task {
            date: {
                let words: Vec<&str> = self.datum.split("T").collect();
                words
                    .get(0)
                    .expect("Expected a date before the T")
                    .to_string()
            },
            creation_date: {
                let words: Vec<&str> = self.bejelentes_datuma.split("T").collect();
                words
                    .get(0)
                    .expect("Expected a date before the T")
                    .to_string()
            },
            subject: self.tantargy,
            teacher: self.tanar,
            id: self.id,
            topic: self.szamonkeres_megnevezese,
            grade_type: self.szamonkeres_modja,
        }
    }
}
