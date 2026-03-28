use serde::Deserialize;
use uuid::Uuid;

use crate::application::patient::{error::ValidationError, validation::cpf::correct_cpf_mask};

#[derive(Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum GetPatientByInput {
    Id(String),
    Cpf(String),
}

pub enum GetPatientBy {
    Id(Uuid),
    Cpf(String),
}

impl GetPatientBy {
    pub fn new(input: GetPatientByInput) -> Result<Self, ValidationError> {
        match input {
            GetPatientByInput::Cpf(c) => {
                if !correct_cpf_mask(&c) {
                    return Err(ValidationError::InvalidCpfField);
                }

                Ok(GetPatientBy::Cpf(c))
            }

            GetPatientByInput::Id(id) => {
                let parsed_id =
                    Uuid::parse_str(&id).map_err(|_| ValidationError::InvalidIdField)?;
                Ok(GetPatientBy::Id(parsed_id))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn patient_id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn should_parse_get_patient_by_input_id() {
        let input = GetPatientByInput::Id(patient_id(1).to_string());
        let result = GetPatientBy::new(input);
        assert!(result.is_ok());
    }

    #[test]
    fn should_parse_get_patient_by_input_cpf() {
        let input = GetPatientByInput::Cpf("123.456.789-09".to_string());
        let result = GetPatientBy::new(input);
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_parsing_get_patient_by_input_id() {
        let input = GetPatientByInput::Id("invalid-uuid".to_string());
        let result = GetPatientBy::new(input);
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_parsing_get_patient_by_input_cpf() {
        let input = GetPatientByInput::Cpf("invalid-cpf".to_string());
        let result = GetPatientBy::new(input);
        assert!(result.is_err());
    }
}
