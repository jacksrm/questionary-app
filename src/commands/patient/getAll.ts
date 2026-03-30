import { invoke } from '@tauri-apps/api/core';

export async function getAll(): Promise<Patient[]> {
  return await invoke<Patient[]>('get_all');
}
