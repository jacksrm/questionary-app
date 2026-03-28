-- Add migration script here
CREATE TABLE patients (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    cpf TEXT NOT NULL UNIQUE,
    phone1 TEXT NOT NULL,
    phone2 TEXT,
    birth_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

CREATE INDEX idx_patients_deleted_at ON patients (deleted_at);