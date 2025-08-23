import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

type SleepSession = {
  id: number;
  date: string;       // YYYY-MM-DD
  bed_time: string;   // HH:mm:ss
  wake_time: string;  // HH:mm:ss
  latency_min: number | null;
  awakenings: number | null;
  quality: number | null;
};

export const load: PageServerLoad = async ({ fetch, params, url }) => {
  const id = Number(params.id);
  if (!Number.isFinite(id) || id <= 0) {
    throw error(400, 'Invalid id');
  }

  const res = await fetch(`/api/sleep/${id}`);
  if (!res.ok) {
    if (res.status === 404) throw error(404, 'Not found');
    throw error(res.status, `Failed to load record: ${res.status}`);
  }

  const rec = (await res.json()) as SleepSession;
  const qDate = url.searchParams.get('date');

  return { rec, qDate };
};
