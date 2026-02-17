import type { PersonalizationResponse } from '$lib/api';
import type { PageServerLoad } from './$types';

function parseInitialDate(value: string | null): string | null {
  if (!value) return null;
  return /^\d{4}-\d{2}-\d{2}$/.test(value) ? value : null;
}

export const load: PageServerLoad = async ({ fetch, url }) => {
  const initialDate = parseInitialDate(url.searchParams.get('date'));
  let personalization: PersonalizationResponse | null = null;

  try {
    const res = await fetch('/api/trends/personalization');
    if (res.ok) {
      personalization = (await res.json()) as PersonalizationResponse;
    }
  } catch {
    // ignore and keep fallback behavior
  }

  return { initialDate, personalization };
};
