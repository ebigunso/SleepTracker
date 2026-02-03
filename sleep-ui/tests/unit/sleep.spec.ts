import { describe, it, expect } from 'vitest';
import { computeSegments, formatDurationMin, formatIsoTime } from '../../src/lib/utils/sleep';

describe('computeSegments', () => {
  it('returns single segment when no wrap (22:00 -> 23:00)', () => {
    const segs = computeSegments('22:00:00', '23:00:00');
    expect(segs).toHaveLength(1);
    expect(segs[0]).toEqual({ start: 22 * 60, end: 23 * 60 });
  });

  it('wraps across midnight (23:00 -> 06:00) into two segments', () => {
    const segs = computeSegments('23:00:00', '06:00:00');
    expect(segs).toHaveLength(2);
    expect(segs[0]).toEqual({ start: 0, end: 6 * 60 });
    expect(segs[1]).toEqual({ start: 23 * 60, end: 24 * 60 });
  });
});

describe('formatIsoTime', () => {
  it('pads hours and minutes for HH:mm inputs', () => {
    expect(formatIsoTime('8:5')).toBe('08:05');
    expect(formatIsoTime('9:45')).toBe('09:45');
  });

  it('returns fallback when input is empty or incomplete', () => {
    expect(formatIsoTime('')).toBe('—');
    expect(formatIsoTime('7')).toBe('7');
  });
});

describe('formatDurationMin', () => {
  it('formats minutes into human readable strings', () => {
    expect(formatDurationMin(45)).toBe('45m');
    expect(formatDurationMin(60)).toBe('1h');
    expect(formatDurationMin(75)).toBe('1h 15m');
  });

  it('handles invalid or negative values', () => {
    expect(formatDurationMin(null)).toBe('—');
    expect(formatDurationMin(Number.NaN)).toBe('—');
    expect(formatDurationMin(-5)).toBe('0m');
  });
});
