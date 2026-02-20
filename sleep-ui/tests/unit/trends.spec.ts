import { describe, expect, it } from 'vitest';
import {
  addDays,
  averageMetricValues,
  isoWeekBucket,
  priorRange,
  rangeDays,
  type TrendsMetricKey
} from '../../src/lib/utils/trends';

describe('trends date utilities', () => {
  it('computes inclusive day span and prior range', () => {
    expect(rangeDays('2026-02-01', '2026-02-14')).toBe(14);
    expect(priorRange('2026-02-01', '2026-02-14')).toEqual({
      from: '2026-01-18',
      to: '2026-01-31'
    });
    expect(priorRange('2026-02-14', '2026-02-01')).toBeNull();
  });

  it('adds days using local date semantics', () => {
    expect(addDays('2026-02-01', -1)).toBe('2026-01-31');
    expect(addDays('2026-12-31', 1)).toBe('2027-01-01');
  });

  it('computes ISO week bucket across year boundaries', () => {
    expect(isoWeekBucket('2024-12-30')).toBe('2025-W01');
    expect(isoWeekBucket('2025-01-01')).toBe('2025-W01');
    expect(isoWeekBucket('2026-01-04')).toBe('2026-W01');
  });
});

describe('averageMetricValues', () => {
  it('averages duration and quality directly', () => {
    expect(averageMetricValues([420, 480, 450], 'duration', 12 * 60)).toBe(450);
    expect(averageMetricValues([2.5, 3.5, 4], 'quality', 12 * 60)).toBeCloseTo(10 / 3, 5);
  });

  it('wraps bedtime/waketime around anchor before averaging', () => {
    const aroundMidnight = [23 * 60 + 30, 15];
    const bedtimeAvg = averageMetricValues(aroundMidnight, 'bedtime', 12 * 60);
    expect(bedtimeAvg).toBeCloseTo(1432.5, 5);

    const wakeAvg = averageMetricValues([6 * 60 + 30, 7 * 60], 'waketime', 12 * 60);
    expect(wakeAvg).toBeCloseTo(1845, 5);
  });

  it('returns null for empty values', () => {
    expect(averageMetricValues([], 'duration', 12 * 60)).toBeNull();
  });

  it('accepts all metric keys', () => {
    const keys: TrendsMetricKey[] = ['duration', 'quality', 'bedtime', 'waketime'];
    expect(keys).toHaveLength(4);
  });
});
