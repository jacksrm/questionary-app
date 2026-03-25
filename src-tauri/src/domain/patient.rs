use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Patient {
    pub id: Uuid,
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
