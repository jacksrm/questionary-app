use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::patient::Patient;
use crate::dto::patient::CreatePatient;
use crate::infra::repository::patient::PatientRepository;

pub struct PatientService {
    repo: Box<dyn PatientRepository>,
}

impl PatientService {
    pub fn new(repo: Box<dyn PatientRepository>) -> Self {
        PatientService { repo }
    }

    pub fn create_patient(&mut self, dto: CreatePatient) -> Result<(), String> {
        let CreatePatient {
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = dto;

        let Ok(birth_date) = NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d") else {
            return Err(
                "There was a problem with the data received. Check \"Birth Date\"".to_string(),
            );
        };

        let patient = Patient {
            id: Uuid::new_v4(),
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        };

        self.repo.save(patient)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use uuid::Uuid;

    use crate::{
        domain::patient::Patient, infra::repository::patient::in_memory::InMemoryUserRepository,
    };

    use super::*;

    const PATIENT_NAME: &str = "John Doe";
    const PATIENT_CPF: &str = "123.456.789-00";
    const PATIENT_PHONE1: &str = "(85) 98765-4321";
    const PATIENT_BIRTH_DATE: &str = "1988-02-26";
    const PATIENT_BIRTH_DATE_FMT: &str = "%Y-%m-%d";

    fn patient_id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn service_factory_clean() -> PatientService {
        let repo = Box::new(InMemoryUserRepository::new());
        let service = PatientService::new(repo);

        service
    }

    fn service_factory_single() -> PatientService {
        let mut repo = Box::new(InMemoryUserRepository::new());
        let patient = Patient {
            id: patient_id(1),
            name: PATIENT_NAME.to_string(),
            cpf: PATIENT_CPF.to_string(),
            phone1: PATIENT_PHONE1.to_string(),
            phone2: None,
            birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT)
                .unwrap(),
        };

        repo.save(patient).unwrap();

        let service = PatientService::new(repo);
        service
    }

    fn service_factory_many() -> PatientService {
        let mut repo = Box::new(InMemoryUserRepository::new());
        for n in 0..50 {
            let patient = Patient {
                id: patient_id(n),
                name: format!("{}{}", PATIENT_NAME, n),
                cpf: PATIENT_CPF.to_string(),
                phone1: PATIENT_PHONE1.to_string(),
                phone2: None,
                birth_date: NaiveDate::parse_from_str(PATIENT_BIRTH_DATE, PATIENT_BIRTH_DATE_FMT)
                    .unwrap(),
            };

            repo.save(patient).unwrap();
        }

        let service = PatientService::new(repo);

        service
    }

    #[test]
    fn should_be_able_to_create_a_patient() {
        let mut service = service_factory_clean();
        let to_create = CreatePatient {
            name: PATIENT_NAME.to_string(),
            cpf: PATIENT_CPF.to_string(),
            birth_date: PATIENT_BIRTH_DATE.to_string(),
            phone1: PATIENT_PHONE1.to_string(),
            phone2: None,
        };

        service.create_patient(to_create).unwrap();

        assert_eq!(service.repo.get_all().len(), 1);
    }
}
