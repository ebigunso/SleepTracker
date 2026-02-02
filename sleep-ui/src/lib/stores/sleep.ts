import { writable } from 'svelte/store';
import type { SleepSession } from '$lib/api';

type Intensity = 'none' | 'light' | 'hard';

export const recentSleep = writable<SleepSession[]>([]);

// Map of date (YYYY-MM-DD) -> intensity
export const exerciseIntensityByDate = writable<Record<string, Intensity>>({});

// Helpers to update stores
export function upsertRecent(item: SleepSession) {
  recentSleep.update((arr) => {
    const withoutId = arr.filter((x) => x.id !== item.id);
    const next = [item, ...withoutId];
    next.sort((a, b) => {
      if (a.date !== b.date) return a.date < b.date ? 1 : -1;
      if (a.wake_time !== b.wake_time) return a.wake_time < b.wake_time ? 1 : -1;
      return a.id < b.id ? 1 : a.id > b.id ? -1 : 0;
    });
    return next;
  });
}

export function removeRecentById(id: number) {
  recentSleep.update((arr) => arr.filter((x) => x.id !== id));
}

export function setIntensity(date: string, intensity: Intensity) {
  exerciseIntensityByDate.update((m) => ({ ...m, [date]: intensity }));
}
