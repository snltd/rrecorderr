import { apiGet } from '#lib/server/api';

export async function load() {
  const [artistInitials, labelInitials, media] = await Promise.all([
    apiGet<string[]>('/initials/artists'),
    apiGet<string[]>('/initials/labels'),
    apiGet<string[]>('/media'),
  ]);
  return { artistInitials, labelInitials, media };
}
