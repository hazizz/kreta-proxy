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
    #[serde(default)]
    count: i8,
    #[serde(default)]
    date: String,
    #[serde(default)]
    start_time: String,
    #[serde(default)]
    end_time: String,
    #[serde(default)]
    nev: String,
    #[serde(default)]
    class_room: String,
    #[serde(default)]
    class_group: String,
    #[serde(default)]
    teacher: String,
    #[serde(default)]
    state_name: String,
    #[serde(default)]
    theme: Option<String>,
}

impl Default for UnrefinedLesson {
    fn default() -> Self {
        UnrefinedLesson {
            count: -1,
            date: String::from("1999-09-19T00:00:00"),
            start_time: String::from("1999-09-19T09:00:00"),
            end_time: String::from("1999-09-19T09:45:00"),
            nev: String::from("-"),
            class_room: String::from("-"),
            class_group: String::from("-"),
            teacher: String::from("-"),
            state_name: String::from("-"),
            theme: None,
        }
    }
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
    #[serde(default)]
    name: String,
    #[serde(default)]
    institute_name: String,
    #[serde(default)]
    student_id: u64,
    #[serde(default)]
    pub evaluations: Vec<UnrefinedGrade>,
    #[serde(default)]
    pub subject_averages: Vec<UnrefinedAverage>,
    #[serde(default)]
    pub notes: Vec<UnrefinedNote>,
    #[serde(default)]
    pub form_teacher: Option<UnrefinedFormTeacher>,
}

impl Default for UnrefinedProfile {
    fn default() -> Self {
        UnrefinedProfile {
            name: String::from("-"),
            institute_name: String::from("-"),
            student_id: 0,
            evaluations: Vec::new(),
            subject_averages: Vec::new(),
            notes: Vec::new(),
            form_teacher: None,
        }
    }
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
    #[serde(default)]
    r#type: String,
    #[serde(default)]
    number_value: u8,
    #[serde(default)]
    value: String,
    #[serde(default)]
    teacher: String,
    #[serde(default)]
    date: String,
    #[serde(default)]
    creating_time: String,
}

impl Default for UnrefinedGrade {
    fn default() -> Self {
        UnrefinedGrade {
            subject: None,
            theme: None,
            weight: None,
            r#type: String::from("-"),
            number_value: 0,
            value: String::from("-"),
            teacher: String::from("-"),
            date: String::from("1999-09-19T00:00:00"),
            creating_time: String::from("1999-09-19T00:00:00"),
        }
    }
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
    #[serde(default)]
    subject: String,
    #[serde(default)]
    value: f64,
    #[serde(default)]
    class_value: f64,
    #[serde(default)]
    difference: f64,
}

impl Default for UnrefinedAverage {
    fn default() -> Self {
        UnrefinedAverage {
            subject: String::from("-"),
            value: 0.0,
            class_value: 0.0,
            difference: 0.0,
        }
    }
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
    #[serde(default)]
    note_id: u64,
    r#type: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    content: Option<String>,
    teacher: Option<String>,
    creating_time: String,
}

impl Default for UnrefinedNote {
    fn default() -> Self {
        UnrefinedNote {
            note_id: 0,
            r#type: String::from("-"),
            title: None,
            content: None,
            teacher: None,
            creating_time: String::from("1999-09-19T00:00:00"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: u64,
    r#type: String,
    title: Option<String>,
    content: Option<String>,
    teacher: Option<String>,
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
    #[serde(default)]
    teacher_id: u64,
    name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
}

impl Default for UnrefinedFormTeacher {
    fn default() -> Self {
        UnrefinedFormTeacher {
            teacher_id: 0,
            r#name: None,
            email: None,
            phone_number: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormTeacher {
    id: u64,
    name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
}

impl UnrefinedFormTeacher {
    pub fn refine(self) -> FormTeacher {
        FormTeacher {
            id: self.teacher_id,
            name: self.name,
            email: self.email,
            phone_number: self.phone_number,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnrefinedTask {
    #[serde(default)]
    id: u64,
    #[serde(default)]
    datum: String,
    #[serde(default)]
    bejelentes_datuma: String,
    tantargy: Option<String>,
    tanar: Option<String>,
    szamonkeres_megnevezese: Option<String>,
    szamonkeres_modja: Option<String>,
}

impl Default for UnrefinedTask {
    fn default() -> Self {
        UnrefinedTask {
            id: 0,
            datum: String::from("1999-09-19T00:00:00"),
            bejelentes_datuma: String::from("1999-09-19T00:00:00"),
            tantargy: None,
            tanar: None,
            szamonkeres_megnevezese: None,
            szamonkeres_modja: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: u64,
    subject: Option<String>,
    topic: Option<String>,
    grade_type: Option<String>,
    teacher: Option<String>,
    date: String,
    creation_date: String,
}

impl UnrefinedTask {
    pub fn refine(self) -> Task {
        Task {
            id: self.id,
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
            topic: self.szamonkeres_megnevezese,
            grade_type: self.szamonkeres_modja,
        }
    }
}
