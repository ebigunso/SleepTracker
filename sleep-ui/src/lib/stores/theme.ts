import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark';

const THEME_KEY = 'sleeptracker.theme';

function readStoredTheme(): Theme | null {
  if (!browser) return null;
  try {
    const stored = localStorage.getItem(THEME_KEY);
    return stored === 'light' || stored === 'dark' ? stored : null;
  } catch {
    return null;
  }
}

function applyTheme(value: Theme) {
  if (!browser) return;
  document.documentElement.dataset.theme = value;
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
    try {
      localStorage.setItem(THEME_KEY, value);
    } catch {
      // ignore
    }
  }
};

export function toggleTheme() {
  const next: Theme = get(theme) === 'dark' ? 'light' : 'dark';
  theme.set(next);
}
