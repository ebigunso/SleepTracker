import { writable } from 'svelte/store';
import type { SleepListItem } from '$lib/api';

type Intensity = 'none' | 'light' | 'hard';

export const recentSleep = writable<SleepListItem[]>([]);

// Map of date (YYYY-MM-DD) -> intensity
export const exerciseIntensityByDate = writable<Record<string, Intensity>>({});

// Helpers to update stores
export function upsertRecent(item: SleepListItem) {
  recentSleep.update((arr) => {
    const idx = arr.findIndex((x) => x.date === item.date);
    if (idx >= 0) {
      const copy = arr.slice();
      copy[idx] = item;
      // Keep sort by date desc if already sorted
      copy.sort((a, b) => (a.date < b.date ? 1 : a.date > b.date ? -1 : 0));
      return copy;
    }
    return [item, ...arr].sort((a, b) => (a.date < b.date ? 1 : a.date > b.date ? -1 : 0));
  });
}

export function removeRecentById(id: number) {
  recentSleep.update((arr) => arr.filter((x) => x.id !== id));
}

export function setIntensity(date: string, intensity: Intensity) {
  exerciseIntensityByDate.update((m) => ({ ...m, [date]: intensity }));
}
