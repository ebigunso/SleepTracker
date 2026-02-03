import { describe, it, expect } from 'vitest';
import { computeSegments, formatDurationHMM, formatMinutesAsTime, formatTimeHHMM } from '../../src/lib/utils/sleep';

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

describe('formatDurationHMM', () => {
  it('formats minutes as h:mm', () => {
    expect(formatDurationHMM(125)).toBe('2:05');
  });

  it('rounds and clamps negative values', () => {
    expect(formatDurationHMM(89.6)).toBe('1:30');
    expect(formatDurationHMM(-5)).toBe('0:00');
  });

  it('returns em dash for invalid input', () => {
    expect(formatDurationHMM(null)).toBe('—');
    expect(formatDurationHMM(undefined)).toBe('—');
    expect(formatDurationHMM(Number.NaN)).toBe('—');
  });
});

describe('formatMinutesAsTime', () => {
  it('formats minutes as HH:MM in 24h time', () => {
    expect(formatMinutesAsTime(90)).toBe('01:30');
    expect(formatMinutesAsTime(24 * 60)).toBe('00:00');
  });

  it('wraps negative minutes', () => {
    expect(formatMinutesAsTime(-30)).toBe('23:30');
  });

  it('returns em dash for invalid input', () => {
    expect(formatMinutesAsTime(null)).toBe('—');
    expect(formatMinutesAsTime(undefined)).toBe('—');
    expect(formatMinutesAsTime(Number.NaN)).toBe('—');
  });
});

describe('formatTimeHHMM', () => {
  it('formats HH:mm:ss as HH:MM', () => {
    expect(formatTimeHHMM('23:15:00')).toBe('23:15');
  });

  it('returns em dash for missing input', () => {
    expect(formatTimeHHMM(null)).toBe('—');
    expect(formatTimeHHMM(undefined)).toBe('—');
  });
});
