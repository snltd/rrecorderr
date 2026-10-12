import type { ValuedRelease } from '#lib/types';
import { apiGet } from '#lib/server/api';

export async function load({ params, fetch }) {
  const releases = await apiGet<ValuedRelease[]>(
    `/most_expensive/${encodeURIComponent(params.medium)}/${encodeURIComponent(params.limit)}`,
  );
  return { releases };
}
