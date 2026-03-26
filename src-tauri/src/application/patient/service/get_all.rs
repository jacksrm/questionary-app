use super::*;

impl PatientService {
    pub fn get_all(&self) -> Result<Vec<&Patient>, PatientError> {
        let patients = self.repo.get_all()?;

        Ok(patients)
    }
}
