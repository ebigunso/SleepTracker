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

export const load = async ({ fetch, params }: any) => {
  const date = params.date as string;
  let items: SleepListItem[] = [];
  try {
    const res = await fetch(`/sleep/range?from=${date}&to=${date}`);
    if (res.ok) {
      items = await res.json();
    }
  } catch {
    // ignore
  }
  const item = items.length > 0 ? items[0] : null;
  return { date, item };
};
