use super::*;

impl PatientService {
    pub async fn update(&self, dto: UpdatePatient) -> Result<Patient, PatientError> {
        let UpdatePatient {
            id,
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = dto;

        let to_update = self.repo.find_by_id(&id).await?;

        let Some(mut to_update) = to_update else {
            return Err(PatientError::NotFound);
        };

        if let Some(ref v) = cpf {
            if self.repo.find_by_cpf(&v).await?.is_some_and(|p| p.id != id) {
                return Err(PatientError::CpfAlreadyInUse);
            }
        }

        if let Some(v) = cpf {
            to_update.cpf = v;
        }

        if let Some(v) = birth_date {
            to_update.birth_date = v;
        }

        if let Some(v) = name {
            to_update.name = v;
        }

        if let Some(v) = phone1 {
            to_update.phone1 = v;
        }

        if let Some(v) = phone2 {
            to_update.phone2 = match v {
                UpdatePhone2Field::Clear => None,
                UpdatePhone2Field::Value(v) => Some(v),
            };
        }

        self.repo.save(&to_update).await?;

        Ok(to_update)
    }
}
