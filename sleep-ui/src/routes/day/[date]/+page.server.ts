type SleepSession = {
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
  let items: SleepSession[] = [];
  try {
    const res = await fetch(`/api/sleep/date/${date}`);
    if (res.ok) {
      const data = await res.json();
      if (Array.isArray(data)) {
        items = data as SleepSession[];
      }
    }
  } catch {
    // ignore
  }
  return { date, items };
};
