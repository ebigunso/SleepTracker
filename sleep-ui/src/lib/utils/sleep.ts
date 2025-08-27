export type Segment = { start: number; end: number }; // minutes since 00:00

export function toMinutes(t: string): number {
  const [hh, mm] = t.split(':').map((v) => parseInt(v, 10));
  const hours = Number.isFinite(hh) ? hh : 0;
  const mins = Number.isFinite(mm) ? mm : 0;
  return hours * 60 + mins;
}

/**
 * Compute 24h track segments for a sleep interval that may cross midnight.
 * Inputs are "HH:mm" or "HH:mm:ss" local times.
 */
export function computeSegments(bed_time: string, wake_time: string): Segment[] {
  const total = 24 * 60;
  const bedMin = toMinutes(bed_time);
  const wakeMin = toMinutes(wake_time);

  if (!Number.isFinite(bedMin) || !Number.isFinite(wakeMin)) return [];
  if (bedMin <= wakeMin) {
    return [{ start: bedMin, end: wakeMin }];
  }
  // Wrap across midnight: [0, wake) and [bed, 1440)
  return [
    { start: 0, end: wakeMin },
    { start: bedMin, end: total }
  ];
}
