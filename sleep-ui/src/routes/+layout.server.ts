import type { LayoutServerLoad } from './$types';

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
  return { session, pathname: url.pathname };
};
