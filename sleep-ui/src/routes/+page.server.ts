
type SleepListItem = {
  id: number;
  date: string; // YYYY-MM-DD
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
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  return `${y}-${m}-${day}`;
}

function startOfDay(d: Date): Date {
  const copy = new Date(d);
  copy.setHours(0, 0, 0, 0);
  return copy;
}

function parseDateParam(value: string | null): Date | null {
  if (!value) return null;
  const d = new Date(`${value}T00:00:00`);
  return Number.isNaN(d.getTime()) ? null : d;
}

function addDays(d: Date, days: number): Date {
  const copy = new Date(d);
  copy.setDate(copy.getDate() + days);
  return copy;
}

function diffDays(from: Date, to: Date): number {
  const ms = startOfDay(to).getTime() - startOfDay(from).getTime();
  return Math.round(ms / (1000 * 60 * 60 * 24));
}

export const load = async ({ fetch, url }: any) => {
  let items: SleepListItem[] = [];
  let intensities: { date: string; intensity: 'none' | 'light' | 'hard' }[] = [];

  const today = startOfDay(new Date());
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
