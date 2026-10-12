import { apiGet } from '#lib/server/api';

export async function load({ params, fetch }) {
  const labels = await apiGet(`/labels/${encodeURIComponent(params.prefix)}`);
  return { prefix: params.prefix, labels };
}
