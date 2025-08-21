// @vitest-environment jsdom
import { describe, it, expect, beforeEach } from 'vitest';
import { getCsrfToken, readCsrfToken } from '../../src/lib/api';

describe('getCsrfToken/readCsrfToken', () => {
  beforeEach(() => {
    // reset cookies
    document.cookie.split(';').forEach((c) => {
      const eqPos = c.indexOf('=');
      const name = eqPos > -1 ? c.substr(0, eqPos).trim() : c.trim();
      if (name) {
        // expire cookie
        document.cookie = `${name}=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/`;
      }
    });
  });

  it('returns __Host-csrf when present (secure mode)', () => {
    document.cookie = '__Host-csrf=SECURETOKEN; path=/';
    document.cookie = 'csrf=DEVTOKEN; path=/';
    expect(readCsrfToken()).toBe('SECURETOKEN');
    expect(getCsrfToken()).toBe('SECURETOKEN');
  });

  it('falls back to dev csrf cookie when __Host-csrf absent', () => {
    document.cookie = 'csrf=DEVTOKEN; path=/';
    expect(readCsrfToken()).toBe('DEVTOKEN');
    expect(getCsrfToken()).toBe('DEVTOKEN');
  });

  it('returns null when no csrf cookies present', () => {
    expect(readCsrfToken()).toBeNull();
    expect(getCsrfToken()).toBeNull();
  });
});
