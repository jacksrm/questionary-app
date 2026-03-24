use chrono::NaiveDate;
use uuid::Uuid;

pub struct CreatePatient {
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: NaiveDate,
}

#[derive(Clone)]
pub struct UpdatePatient {
    pub id: Uuid,
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub phone1: Option<String>,
    pub phone2: Option<Option<String>>,
    pub birth_date: Option<NaiveDate>,
}
