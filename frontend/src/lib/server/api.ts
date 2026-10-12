import { error } from '@sveltejs/kit';
// import { env } from '$env/dynamic/private';

export const API = process.env.API_URL ?? 'http://localhost:3456';

export async function apiGet<T>(path: string): Promise<T> {
  const response = await fetch(`${API}${path}`);
  if (!response.ok) {
    error(response.status, `API returned ${response.status} for ${path}`);
  }
  return response.json();
}
