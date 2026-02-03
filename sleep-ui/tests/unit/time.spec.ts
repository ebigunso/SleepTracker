import { describe, expect, it } from 'vitest';
import { formatDuration, formatTimeHHmm } from '../../src/lib/utils/time';

describe('formatTimeHHmm', () => {
  it('returns placeholder for nullish or invalid values', () => {
    expect(formatTimeHHmm(null)).toBe('—');
    expect(formatTimeHHmm(undefined)).toBe('—');
    expect(formatTimeHHmm('bad')).toBe('—');
    expect(formatTimeHHmm('ab:cd')).toBe('—');
  });

  it('formats Date instances as HH:mm', () => {
    const d = new Date(2026, 1, 4, 9, 7, 30);
    expect(formatTimeHHmm(d)).toBe('09:07');
  });

  it('normalizes string times to HH:mm', () => {
    expect(formatTimeHHmm('9:7')).toBe('09:07');
    expect(formatTimeHHmm('09:07:59')).toBe('09:07');
  });
});

describe('formatDuration', () => {
  it('returns placeholder for nullish or non-finite values', () => {
    expect(formatDuration(null)).toBe('—');
    expect(formatDuration(undefined)).toBe('—');
    expect(formatDuration(Number.NaN)).toBe('—');
    expect(formatDuration(Number.POSITIVE_INFINITY)).toBe('—');
  });

  it('clamps negative values and rounds to minutes', () => {
    expect(formatDuration(-5)).toBe('0:00');
    expect(formatDuration(0)).toBe('0:00');
    expect(formatDuration(59.6)).toBe('1:00');
    expect(formatDuration(90.4)).toBe('1:30');
  });
});
