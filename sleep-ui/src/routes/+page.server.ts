
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

export const load = async ({ fetch }: any) => {
  let recent: SleepListItem[] = [];
  try {
    const res = await fetch('/api/sleep/recent?days=7');
    if (res.ok) {
      recent = await res.json();
    }
  } catch {
    // ignore; treat as empty
  }
  return { recent };
};
