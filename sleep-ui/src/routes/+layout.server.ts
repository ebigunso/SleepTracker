import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

const THEME_COOKIE = 'sleeptracker.theme';

function readThemeCookie(value: string | undefined): 'light' | 'dark' | undefined {
  if (value === 'light' || value === 'dark') return value;
  return undefined;
}

export const load: LayoutServerLoad = async ({ fetch, url, cookies }) => {
  const theme = readThemeCookie(cookies.get(THEME_COOKIE));
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
  if (session && url.pathname === '/login') throw redirect(302, '/');
  if (!session && url.pathname !== '/login') throw redirect(302, '/login');
  return { session, pathname: url.pathname, theme };
};
