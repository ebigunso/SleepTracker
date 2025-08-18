/**
 * Thin fetch wrappers for the SleepTracker API.
 * - Always include credentials (cookies)
 * - Attach X-CSRF-Token for mutating requests by mirroring CSRF cookie
 */

type Json = Record<string, unknown> | unknown[];

function isBrowser(): boolean {
  return typeof window !== 'undefined' && typeof document !== 'undefined';
}

export function getCookie(name: string): string | null {
  if (!isBrowser()) return null;
  const cookies = document.cookie ? document.cookie.split('; ') : [];
  for (const c of cookies) {
    const [k, ...rest] = c.split('=');
    if (decodeURIComponent(k) === name) {
      return decodeURIComponent(rest.join('='));
    }
  }
  return null;
}

/**
 * Reads CSRF token from either "__Host-csrf" (secure) or "csrf" (dev)
 */
export function readCsrfToken(): string | null {
  return getCookie('__Host-csrf') ?? getCookie('csrf');
}

function mergeHeaders(a?: HeadersInit, b?: HeadersInit): Headers {
  const h = new Headers(a ?? {});
  const bH = new Headers(b ?? {});
  bH.forEach((v, k) => h.set(k, v));
  return h;
}

export async function apiGet<T = unknown>(path: string, init: RequestInit = {}): Promise<T> {
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'GET',
    headers: mergeHeaders(init.headers),
  });
  if (!res.ok) {
    throw new Error(`GET ${path} failed: ${res.status}`);
  }
  const ct = res.headers.get('content-type') ?? '';
  if (ct.includes('application/json')) {
    return (await res.json()) as T;
  }
  return (await res.text()) as unknown as T;
}

export async function apiPost<T = unknown>(path: string, body?: Json, init: RequestInit = {}): Promise<T> {
  const csrf = readCsrfToken();
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'POST',
    headers: mergeHeaders(
      {
        'Content-Type': 'application/json',
        ...(csrf ? { 'X-CSRF-Token': csrf } : {}),
      },
      init.headers
    ),
    body: body !== undefined ? JSON.stringify(body) : init.body,
  });
  if (!res.ok) {
    throw new Error(`POST ${path} failed: ${res.status}`);
  }
  const ct = res.headers.get('content-type') ?? '';
  if (ct.includes('application/json')) {
    return (await res.json()) as T;
  }
  return (await res.text()) as unknown as T;
}

export async function apiPut<T = unknown>(path: string, body?: Json, init: RequestInit = {}): Promise<T> {
  const csrf = readCsrfToken();
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'PUT',
    headers: mergeHeaders(
      {
        'Content-Type': 'application/json',
        ...(csrf ? { 'X-CSRF-Token': csrf } : {}),
      },
      init.headers
    ),
    body: body !== undefined ? JSON.stringify(body) : init.body,
  });
  if (!res.ok) {
    throw new Error(`PUT ${path} failed: ${res.status}`);
  }
  const ct = res.headers.get('content-type') ?? '';
  if (ct.includes('application/json')) {
    return (await res.json()) as T;
  }
  return (await res.text()) as unknown as T;
}

export async function apiDelete(path: string, init: RequestInit = {}): Promise<void> {
  const csrf = readCsrfToken();
  const res = await fetch(path, {
    credentials: 'include',
    ...init,
    method: 'DELETE',
    headers: mergeHeaders(
      {
        ...(csrf ? { 'X-CSRF-Token': csrf } : {}),
      },
      init.headers
    ),
  });
  if (!res.ok) {
    throw new Error(`DELETE ${path} failed: ${res.status}`);
  }
}
