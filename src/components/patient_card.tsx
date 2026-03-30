import './patient_card.css';

type PatientCardProps = {
  patient: Patient;
};

function formatDate(dateStr: string) {
  const [year, month, day] = dateStr.split('-').map((v) => parseInt(v, 10));
  const date = new Date(year, month - 1, day);
  return date.toLocaleDateString('pt-BR');
}

export default function PatientCard({ patient }: PatientCardProps) {
  return (
    <article className="card">
      <div className="flex gap-2 grow text-xl border-b-2 border-b-black/50">
        <h3 className="font-semibold">Paciente: </h3>
        <p className="font-mono uppercase">{patient.name}</p>
      </div>
      <div className="flex flex-wrap">
        <div className="card-table-item">
          <h3 className="card-table-heading">CPF</h3>
          <p className="card-table-content">{patient.cpf}</p>
        </div>
        <div className="card-table-item">
          <h3 className="card-table-heading">Data de Nascimento</h3>
          <p className="card-table-content">{formatDate(patient.birth_date)}</p>
        </div>
        <div className="card-table-item">
          <h3 className="card-table-heading">Telefone 1</h3>
          <p className="card-table-content">{patient.phone1}</p>
        </div>
        <div className="card-table-item">
          <h3 className="card-table-heading">Telefone 2</h3>
          <p className="card-table-content">{patient.phone2}</p>
        </div>
      </div>
    </article>
  );
}
