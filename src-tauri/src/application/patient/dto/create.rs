use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePatientInput {
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: String,
}

pub struct CreatePatient {
    pub name: String,
    pub cpf: String,
    pub phone1: String,
    pub phone2: Option<String>,
    pub birth_date: NaiveDate,
}

impl CreatePatient {
    pub fn new(input: CreatePatientInput) {
        todo!();
        let CreatePatientInput {
            birth_date,
            cpf,
            name,
            phone1,
            phone2,
        } = input;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn should_create_a_dto_from_more_generic_data() {
    //     let input = CreatePatientInput {
    //         name: "João Carlos Sardanha".to_string(),
    //         cpf: todo!(),
    //         phone1: todo!(),
    //         phone2: todo!(),
    //         birth_date: todo!(),
    //     };
    // }
}
