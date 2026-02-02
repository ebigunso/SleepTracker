<script lang="ts">
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';
  import { createEventDispatcher } from 'svelte';
  import type { SleepSession } from '$lib/api';
  import { computeDurationMin } from '$lib/utils/sleep';

  export let date: string; // YYYY-MM-DD (display date)
  export let items: SleepSession[] = [];
  export let intensity: 'none' | 'light' | 'hard' | undefined;

  const dispatch = createEventDispatcher<{
    delete: { id: number; date: string };
  }>();

  function onAdd() {
    // Prefill date in query for Quick Log
    goto(`/sleep/new?date=${encodeURIComponent(date)}`);
  }

  function onEdit(id: number, itemDate: string) {
    goto(`/sleep/${id}/edit?date=${encodeURIComponent(itemDate)}`);
  }

  function onDelete(id: number) {
    dispatch('delete', { id, date });
  }

  function fmtMin(n: number | null | undefined): string {
    if (n == null) return '—';
    const h = Math.floor(n / 60);
    const m = n % 60;
    return `${h}h ${m}m`;
  }

  function durationFor(item: SleepSession): number {
    return item.duration_min ?? computeDurationMin(item.bed_time, item.wake_time);
  }

  $: sortedItems = [...items].sort((a, b) => {
    if (a.bed_time !== b.bed_time) return a.bed_time < b.bed_time ? -1 : 1;
    return a.id < b.id ? -1 : 1;
  });

  $: sessionCount = sortedItems.length;
  $: totalDuration = sortedItems.reduce((sum, it) => sum + durationFor(it), 0);
  $: avgQuality = sessionCount > 0
    ? Math.round(sortedItems.reduce((sum, it) => sum + (it.quality ?? 0), 0) / sessionCount)
    : null;
  $: avgLatency = sessionCount > 0
    ? Math.round(sortedItems.reduce((sum, it) => sum + (it.latency_min ?? 0), 0) / sessionCount)
    : null;
  $: totalAwakenings = sortedItems.reduce((sum, it) => sum + (it.awakenings ?? 0), 0);

  const badgeColor =
    intensity === 'hard'
      ? 'bg-red-100 text-red-700'
      : intensity === 'light'
      ? 'bg-emerald-100 text-emerald-700'
      : 'bg-gray-100 text-gray-700';
</script>

<div class="flex items-center gap-3 py-2 border-b border-gray-200">
  <div class="w-28 shrink-0 text-sm text-gray-700 font-medium">
    <a class="text-indigo-600 hover:text-indigo-700" href={`/day/${date}`}>{date}</a>
  </div>

  {#if sessionCount > 0}
    <div class="flex-1 min-w-0 space-y-2">
      <div class="flex flex-wrap items-center gap-2 text-xs text-gray-600">
        <span>Sessions: <span class="font-medium">{sessionCount}</span></span>
        <span>Total: <span class="font-medium">{fmtMin(totalDuration)}</span></span>
        <span>Avg Quality: <span class="font-medium">{avgQuality ?? '—'}</span></span>
        <span>Avg Latency: <span class="font-medium">{avgLatency ?? '—'}m</span></span>
        <span>Awakenings: <span class="font-medium">{totalAwakenings}</span></span>
        {#if intensity}
          <span class={`inline-flex items-center rounded px-1.5 py-0.5 ${badgeColor}`}>Exercise: {intensity}</span>
        {/if}
      </div>
      <div class="space-y-2">
        {#each sortedItems as item (item.id)}
          <div class="rounded border border-gray-200 bg-white p-2">
            <SleepBar bed_time={item.bed_time} wake_time={item.wake_time} />
            <div class="mt-1 flex flex-wrap items-center gap-2 text-xs text-gray-600">
              <span>Bed: <span class="font-medium">{item.bed_time}</span></span>
              <span>Wake: <span class="font-medium">{item.wake_time}</span></span>
              <span>Duration: <span class="font-medium">{fmtMin(durationFor(item))}</span></span>
              <span>Quality: <span class="font-medium">{item.quality}</span></span>
              <span>Latency: <span class="font-medium">{item.latency_min}m</span></span>
            </div>
            <div class="mt-2 flex gap-2 justify-end">
              <button
                class="inline-flex items-center rounded-md bg-white px-2 py-1 text-xs font-medium text-gray-700 ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                on:click={() => onEdit(item.id, item.date)}
                aria-label="Edit"
              >
                Edit
              </button>
              <button
                class="inline-flex items-center rounded-md bg-red-600 px-2 py-1 text-xs font-medium text-white hover:bg-red-700"
                on:click={() => onDelete(item.id)}
                aria-label="Delete"
              >
                Delete
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
    <div class="flex gap-2 shrink-0">
      <button
        class="inline-flex items-center rounded-md bg-indigo-600 px-2.5 py-1.5 text-xs font-semibold text-white hover:bg-indigo-700"
        on:click={onAdd}
      >
        Add entry
      </button>
    </div>
  {:else}
    <div class="flex-1 text-sm text-gray-500">No entry</div>
    <div class="shrink-0">
      <button
        class="inline-flex items-center rounded-md bg-indigo-600 px-2.5 py-1.5 text-xs font-semibold text-white hover:bg-indigo-700"
        on:click={onAdd}
      >
        Add entry
      </button>
    </div>
  {/if}
</div>
