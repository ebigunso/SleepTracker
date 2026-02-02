import { beforeEach, describe, expect, it } from 'vitest';
import { get } from 'svelte/store';
import { recentSleep, upsertRecent } from '../../src/lib/stores/sleep';
import type { SleepListItem } from '../../src/lib/api';

function makeItem(overrides: Partial<SleepListItem> = {}): SleepListItem {
  return {
    id: 1,
    date: '2026-02-01',
    bed_time: '23:00:00',
    wake_time: '07:00:00',
    latency_min: 5,
    awakenings: 0,
    quality: 4,
    duration_min: 480,
    ...overrides,
  };
}

describe('recentSleep store', () => {
  beforeEach(() => {
    recentSleep.set([]);
  });

  it('replaces existing entry when date changes for same id', () => {
    const original = makeItem({ id: 10, date: '2026-01-31' });
    const updated = makeItem({ id: 10, date: '2026-02-01' });

    upsertRecent(original);
    upsertRecent(updated);

    const values = get(recentSleep);
    expect(values).toHaveLength(1);
    expect(values[0]).toEqual(updated);
  });

  it('keeps multiple entries for the same date and sorts by wake time desc', () => {
    const first = makeItem({ id: 11, date: '2026-02-01', wake_time: '06:30:00', quality: 2 });
    const second = makeItem({ id: 12, date: '2026-02-01', wake_time: '07:00:00', quality: 5 });

    upsertRecent(first);
    upsertRecent(second);

    const values = get(recentSleep);
    expect(values).toHaveLength(2);
    expect(values[0]).toEqual(second);
    expect(values[1]).toEqual(first);
  });
});
