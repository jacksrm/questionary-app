use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdatePatient {
    pub id: Uuid,
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub phone1: Option<String>,
    pub phone2: Option<Option<String>>,
    pub birth_date: Option<NaiveDate>,
}

pub enum GetPatientBy {
    Id(Uuid),
    Cpf(String),
}

pub mod create;
