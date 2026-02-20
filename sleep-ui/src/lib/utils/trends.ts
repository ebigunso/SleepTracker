import { wrapClockMinutes } from '$lib/utils/sleep';

export type TrendsMetricKey = 'duration' | 'quality' | 'bedtime' | 'waketime';

function iso(d: Date): string {
  const yyyy = d.getFullYear();
  const mm = String(d.getMonth() + 1).padStart(2, '0');
  const dd = String(d.getDate()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd}`;
}

export function parseLocalDate(date: string): Date {
  return new Date(`${date}T00:00:00`);
}

export function addDays(date: string, days: number): string {
  const next = parseLocalDate(date);
  next.setDate(next.getDate() + days);
  return iso(next);
}

export function rangeDays(start: string, end: string): number | null {
  if (!start || !end) return null;
  const s = parseLocalDate(start);
  const e = parseLocalDate(end);
  const diff = Math.floor((e.getTime() - s.getTime()) / (1000 * 60 * 60 * 24)) + 1;
  return diff > 0 ? diff : null;
}

export function priorRange(start: string, end: string): { from: string; to: string } | null {
  const days = rangeDays(start, end);
  if (!days) return null;
  const priorTo = addDays(start, -1);
  const priorFrom = addDays(priorTo, -(days - 1));
  return { from: priorFrom, to: priorTo };
}

export function isoWeekBucket(date: string): string {
  const d = parseLocalDate(date);
  d.setHours(0, 0, 0, 0);
  const day = d.getDay() || 7;
  d.setDate(d.getDate() + 4 - day);
  const isoYear = d.getFullYear();
  const yearStart = new Date(isoYear, 0, 1);
  const week = Math.ceil((((d.getTime() - yearStart.getTime()) / 86400000) + 1) / 7);
  return `${isoYear}-W${String(week).padStart(2, '0')}`;
}

function average(values: number[]): number | null {
  if (!values.length) return null;
  const total = values.reduce((sum, v) => sum + v, 0);
  return total / values.length;
}

export function averageMetricValues(
  values: number[],
  key: TrendsMetricKey,
  wrappedTimeAnchorMinutes: number
): number | null {
  if (!values.length) return null;
  if (key === 'bedtime' || key === 'waketime') {
    const wrapped = values
      .map((value) => wrapClockMinutes(value, wrappedTimeAnchorMinutes))
      .filter((value): value is number => value != null && Number.isFinite(value));
    if (!wrapped.length) return null;
    return average(wrapped);
  }
  return average(values);
}

export function dateToIsoLocal(d: Date): string {
  return iso(d);
}