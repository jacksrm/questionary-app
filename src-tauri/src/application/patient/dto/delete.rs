use serde::Deserialize;
use uuid::Uuid;

use crate::application::patient::error::ValidationError;

#[derive(Deserialize)]
pub struct DeletePatientInput(pub String);

pub struct DeletePatient(pub Uuid);

impl DeletePatient {
    pub fn new(input: DeletePatientInput) -> Result<Self, ValidationError> {
        let DeletePatientInput(id) = input;

        let parsed_id = Uuid::parse_str(&id).map_err(|_| ValidationError::InvalidIdField)?;

        Ok(DeletePatient(parsed_id))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn patient_id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn should_parse_delete_patient_input() {
        let input = DeletePatientInput(patient_id(1).to_string());
        let result = DeletePatient::new(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, patient_id(1));
    }

    #[test]
    fn should_fail_parsing_delete_patient_input() {
        let input = DeletePatientInput("invalid-uuid".to_string());
        let result = DeletePatient::new(input);
        assert!(result.is_err());
    }
}
