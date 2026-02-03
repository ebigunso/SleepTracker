export function formatTimeHHmm(value: string | Date | null | undefined): string {
  if (value == null) return '—';
  if (value instanceof Date && !Number.isNaN(value.getTime())) {
    const hh = String(value.getHours()).padStart(2, '0');
    const mm = String(value.getMinutes()).padStart(2, '0');
    return `${hh}:${mm}`;
  }
  if (typeof value === 'string') {
    const parts = value.split(':');
    if (parts.length >= 2) {
      const hh = String(Number(parts[0])).padStart(2, '0');
      const mm = String(Number(parts[1])).padStart(2, '0');
      if (!Number.isNaN(Number(hh)) && !Number.isNaN(Number(mm))) {
        return `${hh}:${mm}`;
      }
    }
  }
  return '—';
}

export function formatDuration(value: number | null | undefined): string {
  if (value == null || !Number.isFinite(value)) return '—';
  const total = Math.max(0, Math.round(value));
  const h = Math.floor(total / 60);
  const m = total % 60;
  return `${h}:${String(m).padStart(2, '0')}`;
}
