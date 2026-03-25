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
    pub fn new(input: CreatePatientInput) -> Result<Self, ValidationError> {
        let CreatePatientInput {
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = input;

        if name.trim().is_empty() {
            return Err(ValidationError::InvalidNameField);
        }

        if !validate_cpf(&cpf) {
            return Err(ValidationError::InvalidCpfField);
        }

        if !validate_phone(&phone1) {
            return Err(ValidationError::InvalidPhone1Field);
        }

        if let Some(phone) = &phone2 {
            if !validate_phone(phone) {
                return Err(ValidationError::InvalidPhone2Field);
            }
        }

        if !validate_simple_date(&birth_date) {
            return Err(ValidationError::InvalidBirthDateField);
        }

        Ok(Self {
            name,
            cpf,
            phone1,
            phone2,
            birth_date: NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d").unwrap(),
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
    fn should_return_an_error_if_name_is_invalid() {
        let mut input = valid_patient();
        input.name = "   ".to_string();
        let result = CreatePatient::new(input).unwrap_err();
        assert_eq!(result, ValidationError::InvalidNameField);
    }

    #[test]
    fn should_return_an_error_if_cpf_is_invalid() {
        let mut input = valid_patient();
        input.cpf = "123.456.789-00".to_string();
        let result = CreatePatient::new(input).unwrap_err();
        assert_eq!(result, ValidationError::InvalidCpfField);
    }

    #[test]
    fn should_return_an_error_if_phone1_is_invalid() {
        let mut input = valid_patient();
        input.phone1 = "85 98765-4321".to_string();
        let result = CreatePatient::new(input).unwrap_err();
        assert_eq!(result, ValidationError::InvalidPhone1Field);
    }

    #[test]
    fn should_return_an_error_if_phone2_is_invalid() {
        let mut input = valid_patient();
        input.phone2 = Some("85 98765-4321".to_string());
        let result = CreatePatient::new(input).unwrap_err();
        assert_eq!(result, ValidationError::InvalidPhone2Field);
    }
}
