import { describe, it, expect } from 'vitest';
import { computeSegments } from '../../src/lib/utils/sleep';

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
