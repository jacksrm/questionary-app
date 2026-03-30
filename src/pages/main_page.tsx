import PatientCard from '../components/patient_card';
import SearchMenu from '../components/search_menu';
import './main_page.css';

const patient: Patient = {
  id: '00000000-0000-0000-0000-000000000000',
  name: 'João Carlos Sardanha Pereira',
  cpf: '444.896.358-69',
  phone1: '(85) 90000-0000',
  phone2: '(85) 90000-0001',
  birth_date: '2000-01-01',
};

export default function MainPage() {
  const renderMultiple = () => {
    let patients = [];
    for (let i = 0; i < 50; i += 1) {
      patients.push(
        <PatientCard patient={{ ...patient, name: `${patient.name} ${i}` }} />,
      );
    }

    return patients;
  };
  return (
    <main className="flex flex-col">
      <SearchMenu />

      {renderMultiple()}
      <PatientCard patient={patient} />
    </main>
  );
}
