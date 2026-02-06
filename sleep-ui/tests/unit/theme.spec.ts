import { beforeEach, describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';

type ThemeModule = typeof import('../../src/lib/stores/theme');

vi.mock('$app/environment', () => ({ browser: true }));

async function loadThemeModule(stored?: string): Promise<ThemeModule> {
  document.documentElement.dataset.theme = '';
  document.cookie = 'sleeptracker.theme=; Max-Age=0; path=/';
  if (stored) {
    document.cookie = `sleeptracker.theme=${stored}; path=/`;
  }
  vi.resetModules();
  return await import('../../src/lib/stores/theme');
}

describe('theme store', () => {
  beforeEach(() => {
    document.documentElement.dataset.theme = '';
    document.cookie = 'sleeptracker.theme=; Max-Age=0; path=/';
  });

  it('reads persisted theme on load', async () => {
    const { theme } = await loadThemeModule('dark');

    expect(get(theme)).toBe('dark');
    expect(document.documentElement.dataset.theme).toBe('dark');
  });

  it('writes to cookie when setting theme', async () => {
    const { theme } = await loadThemeModule();

    theme.set('dark');

    expect(document.cookie).toContain('sleeptracker.theme=dark');
    expect(document.documentElement.dataset.theme).toBe('dark');
  });

  it('toggleTheme flips the theme value', async () => {
    const { theme, toggleTheme } = await loadThemeModule('light');

    toggleTheme();

    expect(get(theme)).toBe('dark');
    expect(document.cookie).toContain('sleeptracker.theme=dark');
    expect(document.documentElement.dataset.theme).toBe('dark');
  });
});
