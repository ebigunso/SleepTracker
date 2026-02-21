<script lang="ts">
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';
  import { computeDurationMin } from '$lib/utils/sleep';

  type SleepSession = {
    id: number;
    date: string;
    bed_time: string;
    wake_time: string;
    latency_min: number;
    awakenings: number;
    quality: number;
    duration_min: number | null;
  };

  export let data: {
    date: string;
    items: SleepSession[];
  };

  function edit(id: number, date: string) {
    goto(`/sleep/${id}/edit?date=${encodeURIComponent(date)}`);
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

<section class="space-y-6">
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div>
      <h2 class="text-2xl font-semibold text-default" data-testid="day-view-heading">Day view</h2>
      <p class="text-sm text-muted">{data.date}</p>
    </div>
    <a
      class="focus-ring touch-target inline-flex items-center rounded-full bg-indigo-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-700"
      href={`/sleep/new?date=${encodeURIComponent(data.date)}`}
      data-testid="day-add-session-button"
    >
      Add session
    </a>
  </div>

  {#if sessionCount > 0}
    <div class="card p-4 space-y-4">
      <div class="flex flex-wrap items-center gap-3 text-xs text-muted">
        <span>Sessions: <span class="font-semibold text-default">{sessionCount}</span></span>
        <span>Total: <span class="font-semibold text-default">{fmtMin(totalDuration)}</span></span>
        <span>Avg Quality: <span class="font-semibold text-default">{avgQuality ?? '—'}</span></span>
        <span>Avg Latency: <span class="font-semibold text-default">{avgLatency ?? '—'}m</span></span>
        <span>Awakenings: <span class="font-semibold text-default">{totalAwakenings}</span></span>
      </div>
      <div class="space-y-4">
        {#each sortedItems as item (item.id)}
          <div class="card p-4">
            <SleepBar bed_time={item.bed_time} wake_time={item.wake_time} />
            <div class="mt-3 grid grid-cols-2 gap-x-6 gap-y-2 text-sm text-default sm:grid-cols-3">
              <div><span class="text-muted">Bed:</span> <span class="font-medium">{item.bed_time}</span></div>
              <div><span class="text-muted">Wake:</span> <span class="font-medium">{item.wake_time}</span></div>
              <div><span class="text-muted">Duration:</span> <span class="font-medium">{fmtMin(durationFor(item))}</span></div>
              <div><span class="text-muted">Latency:</span> <span class="font-medium">{item.latency_min}m</span></div>
              <div><span class="text-muted">Awakenings:</span> <span class="font-medium">{item.awakenings}</span></div>
              <div><span class="text-muted">Quality:</span> <span class="font-medium">{item.quality}</span></div>
            </div>
            <div class="mt-4 flex justify-end">
              <button
                class="focus-ring touch-target inline-flex items-center rounded-full border border-[color:var(--color-border)] bg-[color:var(--color-surface)] px-4 py-2 text-sm font-semibold text-default shadow-sm hover:bg-[color:var(--color-surface-muted)]"
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
    <div class="state-card state-card--empty" data-testid="day-empty-state">
      No sleep entry for this date.
    </div>
  {/if}
</section>
