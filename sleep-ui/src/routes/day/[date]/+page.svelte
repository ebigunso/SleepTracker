<script lang="ts">
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';
  import { computeDurationMin } from '$lib/utils/sleep';

  export let data: {
    date: string;
    items: {
      id: number;
      date: string;
      bed_time: string;
      wake_time: string;
      latency_min: number;
      awakenings: number;
      quality: number;
      duration_min: number | null;
    }[];
  };

  function edit(id: number, date: string) {
    goto(`/sleep/${id}/edit?date=${encodeURIComponent(date)}`);
  }

  function add() {
    goto(`/sleep/new?date=${encodeURIComponent(data.date)}`);
  }

  function fmtMin(n: number | null | undefined): string {
    if (n == null) return '—';
    const h = Math.floor(n / 60);
    const m = n % 60;
    return `${h}h ${m}m`;
  }

  function durationFor(item: any): number {
    return item.duration_min ?? computeDurationMin(item.bed_time, item.wake_time);
  }

  $: sortedItems = [...(data.items ?? [])].sort((a, b) => {
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
</script>

<section class="space-y-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xl font-semibold text-gray-900">Day view: {data.date}</h2>
    <button
      class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-700"
      on:click={add}
    >
      Add session
    </button>
  </div>

  {#if sessionCount > 0}
    <div class="rounded-md bg-white shadow p-4 space-y-3">
      <div class="flex flex-wrap items-center gap-2 text-xs text-gray-600">
        <span>Sessions: <span class="font-medium">{sessionCount}</span></span>
        <span>Total: <span class="font-medium">{fmtMin(totalDuration)}</span></span>
        <span>Avg Quality: <span class="font-medium">{avgQuality ?? '—'}</span></span>
        <span>Avg Latency: <span class="font-medium">{avgLatency ?? '—'}m</span></span>
        <span>Awakenings: <span class="font-medium">{totalAwakenings}</span></span>
      </div>
      <div class="space-y-3">
        {#each sortedItems as item (item.id)}
          <div class="rounded border border-gray-200 bg-white p-3">
            <SleepBar bed_time={item.bed_time} wake_time={item.wake_time} />
            <div class="mt-2 grid grid-cols-2 sm:grid-cols-3 gap-x-6 gap-y-2 text-sm text-gray-700">
              <div><span class="text-gray-500">Bed:</span> <span class="font-medium">{item.bed_time}</span></div>
              <div><span class="text-gray-500">Wake:</span> <span class="font-medium">{item.wake_time}</span></div>
              <div><span class="text-gray-500">Duration:</span> <span class="font-medium">{fmtMin(durationFor(item))}</span></div>
              <div><span class="text-gray-500">Latency:</span> <span class="font-medium">{item.latency_min}m</span></div>
              <div><span class="text-gray-500">Awakenings:</span> <span class="font-medium">{item.awakenings}</span></div>
              <div><span class="text-gray-500">Quality:</span> <span class="font-medium">{item.quality}</span></div>
            </div>
            <div class="mt-3 flex justify-end">
              <button
                class="inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-700 ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                on:click={() => edit(item.id, item.date)}
              >
                Edit
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="rounded-md border border-gray-200 bg-white p-4 text-gray-700">
      No sleep entry for this date.
    </div>
  {/if}
</section>
