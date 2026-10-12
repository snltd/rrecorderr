import { apiGet } from '#lib/server/api';

export async function load({ params, fetch }) {
  const prefixes = await apiGet('/initials/artists');
  return { prefixes };
}
