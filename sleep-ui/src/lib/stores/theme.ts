import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';
import { getCookie, setCookie } from '$lib/api';

export type Theme = 'light' | 'dark';

const THEME_COOKIE = 'sleeptracker.theme';
const THEME_COOKIE_MAX_AGE = 60 * 60 * 24 * 365;

function readStoredTheme(): Theme | null {
  if (!browser) return null;
  const stored = getCookie(THEME_COOKIE);
  return stored === 'light' || stored === 'dark' ? stored : null;
}

function applyTheme(value: Theme) {
  if (!browser) return;
  document.documentElement.dataset.theme = value;
}

function writeThemeCookie(value: Theme) {
  const secure = typeof location !== 'undefined' && location.protocol === 'https:';
  setCookie(THEME_COOKIE, value, {
    path: '/',
    maxAge: THEME_COOKIE_MAX_AGE,
    sameSite: 'Lax',
    secure,
  });
}

const initialTheme: Theme = readStoredTheme() ?? 'light';
const { subscribe, set } = writable<Theme>(initialTheme);

if (browser) {
  applyTheme(initialTheme);
}

export const theme = {
  subscribe,
  set: (value: Theme) => {
    set(value);
    applyTheme(value);
    writeThemeCookie(value);
  }
};

export function toggleTheme() {
  const next: Theme = get(theme) === 'dark' ? 'light' : 'dark';
  theme.set(next);
}
