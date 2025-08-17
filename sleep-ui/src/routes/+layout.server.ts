import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
  let session = false;
  try {
    const res = await fetch('/api/session');
    if (res.ok) {
      const data = await res.json();
      session = Boolean(data?.authenticated);
    }
  } catch {
    // ignore; treat as unauthenticated
    session = false;
  }
  if (!session && url.pathname !== '/login') throw redirect(302, '/login');
  return { session, pathname: url.pathname };
};
