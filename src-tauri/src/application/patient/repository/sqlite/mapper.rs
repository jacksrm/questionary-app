use chrono::{DateTime, NaiveDate, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{application::patient::error::PatientError, domain::patient::Patient};

#[derive(Debug, FromRow)]
pub struct PatientRow {
    pub id: String,
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<&Patient> for PatientRow {
    fn from(patient: &Patient) -> Self {
        Self {
            id: patient.id.to_string(),
            name: patient.name.clone(),
            cpf: patient.cpf.clone(),
            phone1: patient.phone1.clone(),
            phone2: patient.phone2.clone(),
            birth_date: patient.birth_date.to_string(),
            created_at: patient.created_at.to_rfc3339(),
            updated_at: patient.updated_at.to_rfc3339(),
            deleted_at: patient.deleted_at.map(|d| d.to_rfc3339()),
        }
    }
}

impl TryFrom<PatientRow> for Patient {
    type Error = PatientError;

    fn try_from(row: PatientRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&row.id).map_err(|_| PatientError::ErrorConvertingDbEntity)?,
            name: row.name,
            cpf: row.cpf,
            phone1: row.phone1,
            phone2: row.phone2,
            birth_date: NaiveDate::parse_from_str(&row.birth_date, "%Y-%m-%d")
                .map_err(|_| PatientError::ErrorConvertingDbEntity)?,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|_| PatientError::ErrorConvertingDbEntity)?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|_| PatientError::ErrorConvertingDbEntity)?
                .with_timezone(&Utc),
            deleted_at: match row.deleted_at {
                Some(d) => Some(
                    DateTime::parse_from_rfc3339(&d)
                        .map_err(|_| PatientError::ErrorConvertingDbEntity)?
                        .with_timezone(&Utc),
                ),
                None => None,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn patient_id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn should_convert_from_patient_to_patient_row() {
        let patient = Patient {
            id: patient_id(1),
            name: "Rubesvaldo Generico".to_string(),
            cpf: "444.896.358-69".to_string(),
            phone1: "(85) 98765-4321".to_string(),
            phone2: None,
            birth_date: NaiveDate::parse_from_str("1989-12-01", "%Y-%m-%d").unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let row = PatientRow::from(&patient);

        assert_eq!(row.id, "00000000-0000-0000-0000-000000000001");
        assert_eq!(row.name, "Rubesvaldo Generico");
        assert_eq!(row.cpf, "444.896.358-69");
        assert_eq!(row.phone1, "(85) 98765-4321");
        assert_eq!(row.phone2, None);
        assert_eq!(row.birth_date, "1989-12-01");
        assert_eq!(row.created_at, patient.created_at.to_rfc3339());
        assert_eq!(row.updated_at, patient.updated_at.to_rfc3339());
        assert_eq!(row.deleted_at, None);
    }

    #[test]
    fn should_convert_from_patient_row_to_patient() {
        let row = PatientRow {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            name: "Rubesvaldo Generico".to_string(),
            cpf: "444.896.358-69".to_string(),
            phone1: "(85) 98765-4321".to_string(),
            phone2: None,
            birth_date: "1989-12-01".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            deleted_at: None,
        };

        let patient = Patient::try_from(row).unwrap();

        assert_eq!(patient.id, patient_id(1));
        assert_eq!(patient.name, "Rubesvaldo Generico");
        assert_eq!(patient.cpf, "444.896.358-69");
        assert_eq!(patient.phone1, "(85) 98765-4321");
        assert_eq!(patient.phone2, None);
        assert_eq!(
            patient.birth_date,
            NaiveDate::parse_from_str("1989-12-01", "%Y-%m-%d").unwrap()
        );
        assert_eq!(
            patient.created_at,
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc)
        );
        assert_eq!(
            patient.updated_at,
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc)
        );
        assert_eq!(patient.deleted_at, None);
    }
}
