
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
  let intensities: { date: string; intensity: 'none' | 'light' | 'hard' }[] = [];
  try {
    const res = await fetch('/api/sleep/recent?days=7');
    if (res.ok) {
      recent = await res.json();
    }
  } catch {
    // ignore; treat as empty
  }
  // Compute last 7-day window [start, end]
  function isoDate(d: Date): string {
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${day}`;
  }
  const endD = new Date();
  const startD = new Date(endD);
  startD.setDate(endD.getDate() - 6);
  const start = isoDate(startD);
  const end = isoDate(endD);
  try {
    const res2 = await fetch(`/api/exercise/intensity?from=${start}&to=${end}`);
    if (res2.ok) {
      intensities = await res2.json();
    }
  } catch {
    // ignore; treat as empty
  }
  return { recent, intensities };
};
