<script lang="ts">
  import SessionRow from '$lib/components/SessionRow.svelte';
  import { createEventDispatcher } from 'svelte';
  import type { SleepSession } from '$lib/api';
  import { computeDurationMin, formatDurationMin } from '$lib/utils/sleep';

  export let date: string;
  export let items: SleepSession[] = [];
  export let intensity: 'none' | 'light' | 'hard' | undefined;

  const dispatch = createEventDispatcher<{
    delete: { id: number; date: string };
  }>();

  function durationFor(session: SleepSession): number {
    return session.duration_min ?? computeDurationMin(session.bed_time, session.wake_time);
  }

  $: sortedItems = [...items].sort((a, b) => {
    if (a.bed_time !== b.bed_time) return a.bed_time < b.bed_time ? -1 : 1;
    return a.id < b.id ? -1 : 1;
  });

  $: sessionCount = sortedItems.length;
  $: totalDuration = sortedItems.reduce((sum, it) => sum + durationFor(it), 0);
  $: avgQualityValue = sessionCount > 0
    ? Math.round(sortedItems.reduce((sum, it) => sum + (it.quality ?? 0), 0) / sessionCount)
    : null;
  $: avgQualityLabel = avgQualityValue == null ? '—' : `${avgQualityValue}`;

  $: intensityChip =
    intensity === 'hard'
      ? 'bg-green-100 text-green-700 ring-1 ring-inset ring-green-200'
      : intensity === 'light'
      ? 'bg-sky-100 text-sky-700 ring-1 ring-inset ring-sky-200'
      : 'bg-gray-100 text-gray-700 ring-1 ring-inset ring-gray-200';

  function handleDelete(e: CustomEvent<{ id: number; date: string }>) {
    dispatch('delete', e.detail);
  }
</script>

<article class="w-full rounded-2xl border border-slate-200 bg-slate-50/40 p-4 shadow-sm">
  <header class="flex flex-wrap items-start justify-between gap-4">
    <div>
      <a class="text-lg font-semibold text-slate-900 hover:text-indigo-600" href={`/day/${date}`}>{date}</a>
      <p class="text-sm text-slate-500">Total {formatDurationMin(totalDuration)}</p>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <span class="inline-flex items-center rounded-full bg-white px-2.5 py-1 text-xs font-medium text-slate-700 ring-1 ring-inset ring-slate-200">
        {sessionCount} sessions
      </span>
      <span class="inline-flex items-center rounded-full bg-white px-2.5 py-1 text-xs font-medium text-slate-700 ring-1 ring-inset ring-slate-200">
        Avg quality {avgQualityLabel}
      </span>
      {#if intensity}
        <span class={`inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium ${intensityChip}`}>
          Exercise {intensity}
        </span>
      {/if}
      <a class="text-xs font-semibold text-indigo-600 hover:text-indigo-500" href={`/day/${date}`}>View day</a>
    </div>
  </header>

  <div class="mt-4 space-y-3">
    {#if sessionCount > 0}
      {#each sortedItems as item (item.id)}
        <SessionRow item={item} on:delete={handleDelete} />
      {/each}
    {:else}
      <div class="rounded-xl border border-dashed border-slate-200 bg-white px-4 py-6 text-sm text-slate-500">
        No sleep logged for this day.
      </div>
    {/if}
  </div>
</article>
