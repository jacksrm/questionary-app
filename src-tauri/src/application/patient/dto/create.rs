use chrono::NaiveDate;
use serde::Deserialize;

use crate::application::patient::{
    error::ValidationError,
    validation::{cpf::validate_cpf, phone::validate_phone, simple_date::validate_simple_date},
};

#[derive(Deserialize, Debug, PartialEq)]
pub struct CreatePatientInput {
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: String,
}

#[derive(Debug, PartialEq)]
pub struct CreatePatient {
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: NaiveDate,
}

impl CreatePatient {
    pub fn new(input: CreatePatientInput) -> Result<Self, Vec<ValidationError>> {
        let CreatePatientInput {
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = input;

        let mut errors: Vec<ValidationError> = Vec::new();

        if name.trim().is_empty() {
            errors.push(ValidationError::InvalidNameField);
        }

        if !validate_cpf(&cpf) {
            errors.push(ValidationError::InvalidCpfField);
        }

        if !validate_phone(&phone1) {
            errors.push(ValidationError::InvalidPhone1Field);
        }

        if let Some(phone) = &phone2 {
            if !validate_phone(phone) {
                errors.push(ValidationError::InvalidPhone2Field);
            }
        }

        let birth_date = {
            let is_valid = validate_simple_date(&birth_date);

            match NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d") {
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
        };

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            name,
            cpf,
            phone1,
            phone2,
            birth_date: birth_date.unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_USER_NAME: &str = "João Carlos Sardanha";
    const VALID_CPF: &str = "444.896.358-69";
    const VALID_PHONE1: &str = "(85) 98765-4321";
    const VALID_PHONE2: &str = "(11) 98765-4322";
    const VALID_BIRTH_DATE: &str = "1990-01-01";

    fn valid_patient() -> CreatePatientInput {
        CreatePatientInput {
            name: VALID_USER_NAME.to_string(),
            cpf: VALID_CPF.to_string(),
            phone1: VALID_PHONE1.to_string(),
            phone2: Some(VALID_PHONE2.to_string()),
            birth_date: VALID_BIRTH_DATE.to_string(),
        }
    }

    #[test]
    fn should_create_a_dto_from_more_generic_data() {
        let input = valid_patient();
        let patient = CreatePatient::new(input).unwrap();
        assert_eq!(patient.name, VALID_USER_NAME);
        assert_eq!(patient.cpf, VALID_CPF);
        assert_eq!(patient.phone1, VALID_PHONE1);
        assert_eq!(patient.phone2, Some(VALID_PHONE2.to_string()));
        assert_eq!(
            patient.birth_date,
            NaiveDate::parse_from_str(VALID_BIRTH_DATE, "%Y-%m-%d").unwrap()
        );
    }

    #[test]
    fn should_return_errors_if_data_is_invalid() {
        let input = CreatePatientInput {
            name: "   ".to_string(),
            cpf: "123.456.789-00".to_string(),
            phone1: "invalid_phone".to_string(),
            phone2: Some("invalid_phone".to_string()),
            birth_date: "invalid_date".to_string(),
        };

        let result = CreatePatient::new(input).unwrap_err();

        assert_eq!(
            result,
            vec![
                ValidationError::InvalidNameField,
                ValidationError::InvalidCpfField,
                ValidationError::InvalidPhone1Field,
                ValidationError::InvalidPhone2Field,
                ValidationError::InvalidBirthDateField
            ]
        );
    }
}
