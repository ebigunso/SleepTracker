import { beforeEach, describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';

type ThemeModule = typeof import('../../src/lib/stores/theme');

vi.mock('$app/environment', () => ({ browser: true }));

async function loadThemeModule(stored?: string): Promise<ThemeModule> {
  localStorage.clear();
  document.documentElement.dataset.theme = '';
  if (stored) {
    localStorage.setItem('sleeptracker.theme', stored);
  }
  vi.resetModules();
  return await import('../../src/lib/stores/theme');
}

describe('theme store', () => {
  beforeEach(() => {
    localStorage.clear();
    document.documentElement.dataset.theme = '';
  });

  it('reads persisted theme on load', async () => {
    const { theme } = await loadThemeModule('dark');

    expect(get(theme)).toBe('dark');
    expect(document.documentElement.dataset.theme).toBe('dark');
  });

  it('writes to localStorage when setting theme', async () => {
    const { theme } = await loadThemeModule();

    theme.set('dark');

    expect(localStorage.getItem('sleeptracker.theme')).toBe('dark');
    expect(document.documentElement.dataset.theme).toBe('dark');
  });

  it('toggleTheme flips the theme value', async () => {
    const { theme, toggleTheme } = await loadThemeModule('light');

    toggleTheme();

    expect(get(theme)).toBe('dark');
    expect(localStorage.getItem('sleeptracker.theme')).toBe('dark');
    expect(document.documentElement.dataset.theme).toBe('dark');
  });
});
