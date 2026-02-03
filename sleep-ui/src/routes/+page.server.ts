
type SleepSession = {
  id: number;
  date: string; // YYYY-MM-DD (wake date)
  bed_time: string;
  wake_time: string;
  latency_min: number;
  awakenings: number;
  quality: number;
  duration_min: number | null;
};

const WINDOW_DAYS = 14;
const MAX_DAYS = 62;

function isoDate(d: Date): string {
  const y = d.getUTCFullYear();
  const m = String(d.getUTCMonth() + 1).padStart(2, '0');
  const day = String(d.getUTCDate()).padStart(2, '0');
  return `${y}-${m}-${day}`;
}

function startOfDay(d: Date): Date {
  return new Date(Date.UTC(d.getUTCFullYear(), d.getUTCMonth(), d.getUTCDate()));
}

function parseDateParam(value: string | null): Date | null {
  if (!value) return null;
  const d = new Date(`${value}T00:00:00Z`);
  return Number.isNaN(d.getTime()) ? null : d;
}

function addDays(d: Date, days: number): Date {
  const copy = new Date(d);
  copy.setUTCDate(copy.getUTCDate() + days);
  return copy;
}

function diffDays(from: Date, to: Date): number {
  const ms = startOfDay(to).getTime() - startOfDay(from).getTime();
  return Math.round(ms / (1000 * 60 * 60 * 24));
}

function dateFromParts(year: number, month: number, day: number): Date {
  return new Date(Date.UTC(year, month - 1, day));
}

function getTodayPartsForTimeZone(now: Date, timeZone: string): { year: number; month: number; day: number } | null {
  try {
    const parts = new Intl.DateTimeFormat('en-CA', {
      timeZone,
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    }).formatToParts(now);
    const lookup = Object.fromEntries(parts.map((p) => [p.type, p.value]));
    const year = Number(lookup.year);
    const month = Number(lookup.month);
    const day = Number(lookup.day);
    if (!year || !month || !day) return null;
    return { year, month, day };
  } catch {
    return null;
  }
}

export const load = async ({ fetch, url }: any) => {
  let items: SleepSession[] = [];
  let intensities: { date: string; intensity: 'none' | 'light' | 'hard' }[] = [];

  let timezone: string | null = null;
  try {
    const tzRes = await fetch('/api/settings/timezone');
    if (tzRes.ok) {
      const body = await tzRes.json();
      if (typeof body?.timezone === 'string') {
        timezone = body.timezone;
      }
    }
  } catch {
    // ignore
  }

  const now = new Date();
  let todayParts: { year: number; month: number; day: number } | null = null;
  if (timezone) {
    todayParts = getTodayPartsForTimeZone(now, timezone);
  }
  if (!todayParts) {
    todayParts = { year: now.getFullYear(), month: now.getMonth() + 1, day: now.getDate() };
  }

  const today = startOfDay(dateFromParts(todayParts.year, todayParts.month, todayParts.day));
  const toParam = parseDateParam(url.searchParams.get('to'));
  const fromParam = parseDateParam(url.searchParams.get('from'));

  let toDate = toParam ?? (fromParam ? addDays(fromParam, WINDOW_DAYS - 1) : today);
  if (toDate > today) {
    toDate = new Date(today);
  }

  let fromDate = fromParam ?? addDays(toDate, -(WINDOW_DAYS - 1));
  if (fromDate > toDate) {
    fromDate = addDays(toDate, -(WINDOW_DAYS - 1));
  }

  let span = diffDays(fromDate, toDate) + 1;
  if (span !== WINDOW_DAYS) {
    fromDate = addDays(toDate, -(WINDOW_DAYS - 1));
    span = WINDOW_DAYS;
  }
  if (span > MAX_DAYS) {
    fromDate = addDays(toDate, -(MAX_DAYS - 1));
  }

  const from = isoDate(fromDate);
  const to = isoDate(toDate);

  try {
    const res = await fetch(`/api/sleep/range?from=${from}&to=${to}`);
    if (res.ok) {
      items = await res.json();
    }
  } catch {
    // ignore; treat as empty
  }

  try {
    const res2 = await fetch(`/api/exercise/intensity?from=${from}&to=${to}`);
    if (res2.ok) {
      intensities = await res2.json();
    }
  } catch {
    // ignore; treat as empty
  }

  return { items, intensities, from, to, windowDays: WINDOW_DAYS, today: isoDate(today) };
};
