import { apiGet } from '#lib/server/api';
import { Artist } from '#lib/types';

export async function load({ params, fetch }) {
  const artists = await apiGet<Artist[]>(`/artists/${encodeURIComponent(params.prefix)}`);
  return { prefix: params.prefix, artists };
}
