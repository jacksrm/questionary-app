import { useMemo, useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function maskCpf(value: string): string {
  const digits = value.replace(/\D/g, '').slice(0, 11);

  return digits
    .replace(/(\d{3})(\d)/, '$1.$2')
    .replace(/(\d{3})(\d)/, '$1.$2')
    .replace(/(\d{3})(\d{0,2})$/, '$1-$2');
}

function App() {
  const [patients, setPatients] = useState<Patient[]>([]);
  const [input, setInput] = useState('');

  const patientDefault = useMemo(
    () => ({
      name: 'Jacson Rodrigues',
      cpf: '123.456.789-00',
      phone1: '(11) 99999-9999',
      phone2: null,
      birth_date: '1990-01-01',
    }),
    [],
  );

  const clearPatients = () => {
    setPatients([]);
  };

  const getAll = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const result = await invoke<Patient[]>('get_all');
    setPatients(result);
  };

  const getOne = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    if (!input) return;
    clearPatients();
    let by: GetPatientByInput = {
      type: 'cpf',
      value: input,
    };

    if (input.length !== 11) {
      by.type = 'id';
    } else {
      by.value = maskCpf(input);
    }

    const response = await invoke<Patient | null>('get_patient', { input: by });

    setPatients(response ? [response] : []);
  };

  return (
    <main className="container">
      <h1>Test UI</h1>
      <button onClick={getAll} type="button">
        Get All
      </button>
      <button type="button" onClick={(_) => clearPatients()}>
        Clear
      </button>
      <div>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
        />
        <button type="button" onClick={getOne}>
          Get One
        </button>
      </div>
      {patients.map((patient) => (
        <div key={patient.id}>
          <h2>{patient.name}</h2>
          <p>ID: {patient.id}</p>
          <p>CPF: {patient.cpf}</p>
          <p>Phone 1: {patient.phone1}</p>
          <p>Phone 2: {patient.phone2}</p>
          <p>Birth Date: {patient.birth_date}</p>
        </div>
      ))}
    </main>
  );
}

export default App;
