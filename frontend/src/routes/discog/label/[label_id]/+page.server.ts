import type { Release } from '#lib/types';
import { apiGet } from '#lib/server/api';

export async function load({ params, fetch }) {
  const releases = await apiGet<Release[]>(
    `/discog/label/${encodeURIComponent(params.label_id)}`,
  );
  return { releases };
}
