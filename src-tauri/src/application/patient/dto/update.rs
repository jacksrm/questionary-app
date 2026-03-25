use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::application::patient::{
    error::ValidationError,
    validation::{cpf::validate_cpf, phone::validate_phone, simple_date::validate_simple_date},
};

#[derive(Clone, Debug, PartialEq)]
pub enum UpdatePhone2Field {
    Clear,
    Value(String),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct UpdatePatientInput {
    pub id: String,
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub phone1: Option<String>,
    pub birth_date: Option<String>,

    #[serde(default)]
    pub phone2: Option<Option<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePatient {
    pub id: Uuid,
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub phone1: Option<String>,
    pub phone2: Option<UpdatePhone2Field>,
    pub birth_date: Option<NaiveDate>,
}

impl UpdatePatient {
    pub fn new(input: UpdatePatientInput) -> Result<Self, Vec<ValidationError>> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let UpdatePatientInput {
            id,
            name,
            cpf,
            phone1,
            phone2,
            birth_date,
        } = input;

        if let Some(ref n) = name {
            if n.trim().is_empty() {
                errors.push(ValidationError::InvalidNameField);
            }
        }

        if let Some(ref c) = cpf {
            if !validate_cpf(c) {
                errors.push(ValidationError::InvalidCpfField);
            }
        }

        if let Some(ref p1) = phone1 {
            if !validate_phone(p1) {
                errors.push(ValidationError::InvalidPhone1Field);
            }
        }

        let phone2 = match phone2 {
            Some(Some(p)) => {
                if !validate_phone(&p) {
                    errors.push(ValidationError::InvalidPhone2Field);
                }
                Some(UpdatePhone2Field::Value(p))
            }
            Some(None) => Some(UpdatePhone2Field::Clear),
            None => None,
        };

        let birth_date = match birth_date {
            Some(ref bd) => {
                let is_valid = validate_simple_date(bd);

                match NaiveDate::parse_from_str(bd, "%Y-%m-%d") {
                    Ok(date) => {
                        if !is_valid {
                            errors.push(ValidationError::InvalidBirthDateField);
                            None
                        } else {
                            Some(date)
                        }
                    }
                    Err(_) => {
                        errors.push(ValidationError::InvalidBirthDateField);
                        None
                    }
                }
            }
            None => None,
        };

        let id = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                errors.push(ValidationError::InvalidIdField);
                Uuid::nil()
            }
        };

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            id,
            name,
            cpf,
            phone1,
            phone2: phone2,
            birth_date,
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
    fn should_parse_update_patient_input() {
        let input = UpdatePatientInput {
            id: patient_id(1).to_string(),
            name: Some("John Doe".to_string()),
            cpf: Some("123.456.789-09".to_string()),
            phone1: Some("(12) 34567-8901".to_string()),
            phone2: Some(Some("(12) 34567-8902".to_string())),
            birth_date: Some("1990-01-01".to_string()),
        };

        let result = UpdatePatient::new(input);
        assert!(result.is_ok());
    }

    #[test]
    fn should_return_errors_for_invalid_update_patient_input() {
        let input = UpdatePatientInput {
            id: "invalid-uuid".to_string(),
            name: Some("   ".to_string()),
            cpf: Some("invalid-cpf".to_string()),
            phone1: Some("invalid-phone".to_string()),
            phone2: Some(Some("invalid-phone".to_string())),
            birth_date: Some("invalid-date".to_string()),
        };

        let result = UpdatePatient::new(input).unwrap_err();
        assert_eq!(
            result,
            vec![
                ValidationError::InvalidNameField,
                ValidationError::InvalidCpfField,
                ValidationError::InvalidPhone1Field,
                ValidationError::InvalidPhone2Field,
                ValidationError::InvalidBirthDateField,
                ValidationError::InvalidIdField
            ]
        );
    }
}
