<script lang="ts">
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';
  import { createEventDispatcher } from 'svelte';
  import type { SleepListItem } from '$lib/api';

  export let date: string; // YYYY-MM-DD (wake date)
  export let item: SleepListItem | null = null;
  export let intensity: 'none' | 'light' | 'hard' | undefined;

  const dispatch = createEventDispatcher<{
    delete: { id: number; date: string };
  }>();

  function onAdd() {
    // Prefill date in query for Quick Log
    goto(`/sleep/new?date=${encodeURIComponent(date)}`);
  }

  function onEdit() {
    if (!item) return;
    goto(`/sleep/${item.id}/edit?date=${encodeURIComponent(date)}`);
  }

  function onDelete() {
    if (!item) return;
    dispatch('delete', { id: item.id, date });
  }

  function fmtMin(n: number | null | undefined): string {
    if (n == null) return '—';
    const h = Math.floor(n / 60);
    const m = n % 60;
    return `${h}h ${m}m`;
  }

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

  {#if item}
    <div class="flex-1 min-w-0">
      <SleepBar bed_time={item.bed_time} wake_time={item.wake_time} />
      <div class="mt-1 flex flex-wrap items-center gap-2 text-xs text-gray-600">
        <span>Duration: <span class="font-medium">{fmtMin(item.duration_min ?? null)}</span></span>
        <span>Quality: <span class="font-medium">{item.quality}</span></span>
        <span>Latency: <span class="font-medium">{item.latency_min}m</span></span>
        {#if intensity}
          <span class={`inline-flex items-center rounded px-1.5 py-0.5 ${badgeColor}`}>Exercise: {intensity}</span>
        {/if}
      </div>
    </div>
    <div class="flex gap-2 shrink-0">
      <button
        class="inline-flex items-center rounded-md bg-white px-2 py-1 text-xs font-medium text-gray-700 ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
        on:click={onEdit}
        aria-label="Edit"
      >
        Edit
      </button>
      <button
        class="inline-flex items-center rounded-md bg-red-600 px-2 py-1 text-xs font-medium text-white hover:bg-red-700"
        on:click={onDelete}
        aria-label="Delete"
      >
        Delete
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
