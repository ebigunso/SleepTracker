/**
 * Thin fetch wrappers for the SleepTracker API.
 * - Always include credentials (cookies)
 * - Attach X-CSRF-Token for mutating requests by mirroring CSRF cookie
 */

export type Json = Record<string, unknown> | unknown[];

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

export interface CookieOptions {
  path?: string;
  maxAge?: number;
  sameSite?: 'Lax' | 'Strict' | 'None';
  secure?: boolean;
}

export function setCookie(name: string, value: string, options: CookieOptions = {}): void {
  if (!isBrowser()) return;
  const parts = [`${encodeURIComponent(name)}=${encodeURIComponent(value)}`];
  if (options.path) parts.push(`Path=${options.path}`);
  if (typeof options.maxAge === 'number') parts.push(`Max-Age=${options.maxAge}`);
  if (options.sameSite) parts.push(`SameSite=${options.sameSite}`);
  if (options.secure) parts.push('Secure');
  document.cookie = parts.join('; ');
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

/**
 * Alias for CSRF getter (dev or secure cookie)
 */
export const getCsrfToken = readCsrfToken;

/**
 * Low-level fetch wrapper that always includes credentials and attaches CSRF for mutating methods.
 */
export async function apiFetch(path: string, init: RequestInit = {}): Promise<Response> {
  const method = (init.method ?? 'GET').toString().toUpperCase();
  const isMutating = method === 'POST' || method === 'PUT' || method === 'DELETE' || method === 'PATCH';
  const csrf = isMutating ? readCsrfToken() : null;
  const headers = isMutating && csrf
    ? mergeHeaders({ 'X-CSRF-Token': csrf }, init.headers)
    : mergeHeaders(init.headers);
  return fetch(path, { credentials: 'include', ...init, headers });
}

// ------------------------------
// Types matching OpenAPI schemas
// ------------------------------
export type IsoDate = string; // YYYY-MM-DD
export type IsoTime = string; // HH:mm:ss

export interface SleepListItem {
  id: number;
  date: IsoDate;
  bed_time: IsoTime;
  wake_time: IsoTime;
  latency_min: number;
  awakenings: number;
  quality: number;
  duration_min: number | null;
  session_count?: number | null;
}

export interface SleepInput {
  date: IsoDate;
  bed_time: IsoTime;
  wake_time: IsoTime;
  latency_min: number;
  awakenings: number;
  quality: number;
}

export interface SleepSession extends SleepInput {
  id: number;
  duration_min?: number | null;
  session_date?: IsoDate | null;
}

export interface ExerciseUpsert {
  date: IsoDate;
  intensity: 'none' | 'light' | 'hard';
}

// ------------------------------
// Helper APIs for Sleep/Exercise
// ------------------------------
export async function getRecent(days = 7): Promise<SleepListItem[]> {
  return apiGet<SleepListItem[]>(`/api/sleep/recent?days=${days}`);
}

export async function getRange(from: IsoDate, to: IsoDate): Promise<SleepListItem[]> {
  return apiGet<SleepListItem[]>(`/api/sleep/range?from=${from}&to=${to}`);
}

export async function getSleepById(id: number): Promise<SleepSession> {
  return apiGet<SleepSession>(`/api/sleep/${id}`);
}

export async function createSleep(input: SleepInput): Promise<{ id: number }> {
  return apiPost<{ id: number }>('/api/sleep', input as unknown as Json);
}

export async function updateSleep(id: number, input: SleepInput): Promise<void> {
  await apiPut<void>(`/api/sleep/${id}`, input as unknown as Json);
}

export async function deleteSleep(id: number): Promise<void> {
  await apiDelete(`/api/sleep/${id}`);
}

export async function setUserTimezoneIfSupported(timezone: string): Promise<boolean> {
  if (!timezone) return false;
  try {
    const res = await apiFetch('/api/settings/timezone', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ timezone })
    });
    if (res.ok) return true;
    if (res.status === 404 || res.status === 405 || res.status === 501) return false;
    return false;
  } catch {
    return false;
  }
}

export async function getExerciseIntensity(from: IsoDate, to: IsoDate): Promise<{ date: IsoDate; intensity: 'none' | 'light' | 'hard' }[]> {
  return apiGet<{ date: IsoDate; intensity: 'none' | 'light' | 'hard' }[]>(`/api/exercise/intensity?from=${from}&to=${to}`);
}

export async function upsertExercise(payload: ExerciseUpsert): Promise<{ id: number }> {
  return apiPost<{ id: number }>('/api/exercise', payload as unknown as Json);
}
