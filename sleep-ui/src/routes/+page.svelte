<script lang="ts">
  import DayCard from '$lib/components/DayCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import SummaryStrip from '$lib/components/SummaryStrip.svelte';
  import { goto } from '$app/navigation';
  import { browser } from '$app/environment';
  import type { SleepSession } from '$lib/api';
  import { deleteSleep } from '$lib/api';
  import { recentSleep, exerciseIntensityByDate, removeRecentById } from '$lib/stores/sleep';
  import { computeDurationMin, formatDurationMin } from '$lib/utils/sleep';

  export let data: {
    items: SleepSession[];
    intensities?: { date: string; intensity: 'none' | 'light' | 'hard' }[];
    from: string;
    to: string;
    windowDays: number;
    today: string;
  };

  let windowDays = 14;
  let rangeDates: string[] = [];
  let canNext = false;
  let prevTo = '';
  let nextToRaw = '';
  let nextTo = '';
  let jumpTo = '';

  $: localIntensityMap = data.intensities && Array.isArray(data.intensities)
    ? Object.fromEntries(data.intensities.map((d) => [d.date, d.intensity]))
    : {};
  $: if (browser) {
    recentSleep.set(data.items ?? []);
    exerciseIntensityByDate.set(localIntensityMap);
  }

  function isoDate(d: Date): string {
    // Format date in local time to avoid UTC shift (which excluded "today" in non-UTC TZs)
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${day}`;
  }

  function parseDate(date: string): Date {
    return new Date(`${date}T00:00:00`);
  }

  function sessionDate(item: SleepSession): string {
    return item.session_date ?? item.date;
  }

  function shiftIsoDate(date: string, days: number): string {
    const d = parseDate(date);
    d.setDate(d.getDate() + days);
    return isoDate(d);
  }


  function buildRangeDates(from: string, to: string): string[] {
    const start = parseDate(from);
    const end = parseDate(to);
    const arr: string[] = [];
    for (let d = new Date(end); d >= start; d.setDate(d.getDate() - 1)) {
      arr.push(isoDate(d));
    }
    return arr;
  }

  $: intensityMap = browser ? $exerciseIntensityByDate : localIntensityMap;
  $: recentItems = browser ? $recentSleep : (data.items ?? []);

  $: windowDays = data.windowDays ?? 14;
  $: rangeDates = buildRangeDates(data.from, data.to);
  $: canNext = data.to < data.today;
  $: prevTo = shiftIsoDate(data.to, -windowDays);
  $: nextToRaw = shiftIsoDate(data.to, windowDays);
  $: nextTo = nextToRaw > data.today ? data.today : nextToRaw;
  $: jumpTo = data.to;

  $: rows = rangeDates.map((date) => {
    const items = recentItems.filter((x) => sessionDate(x) === date);
    const intensity = intensityMap[date];
    return { date, items, intensity };
  });

  function durationFor(item: SleepSession): number {
    return item.duration_min ?? computeDurationMin(item.bed_time, item.wake_time);
  }

  $: totalSessions = recentItems.length;
  $: totalSleepMin = recentItems.reduce((sum, it) => sum + durationFor(it), 0);
  $: avgDurationMin = totalSessions > 0 ? Math.round(totalSleepMin / totalSessions) : 0;
  $: avgQualityValue = totalSessions > 0
    ? Math.round(recentItems.reduce((sum, it) => sum + (it.quality ?? 0), 0) / totalSessions)
    : null;
  $: avgQuality = avgQualityValue == null ? '—' : `${avgQualityValue}`;
  $: avgLatencyMin = totalSessions > 0
    ? Math.round(recentItems.reduce((sum, it) => sum + (it.latency_min ?? 0), 0) / totalSessions)
    : 0;

  async function handleDelete(e: CustomEvent<{ id: number; date: string }>) {
    const { id } = e.detail;
    if (!confirm('Delete this entry?')) return;
    try {
      await deleteSleep(id);
      removeRecentById(id);
    } catch (err) {
      // swallow; toast handled elsewhere if needed
    }
  }

  function goPrev() {
    goto(`/?to=${prevTo}`);
  }

  function goNext() {
    if (!canNext) return;
    goto(`/?to=${nextTo}`);
  }

  function jumpToDate() {
    if (!jumpTo) return;
    const target = jumpTo > data.today ? data.today : jumpTo;
    goto(`/?to=${target}`);
  }
</script>

<section class="space-y-6">
  <div class="flex flex-wrap items-start justify-between gap-4">
    <div>
      <h2 class="page-title" data-testid="dashboard-heading">
        Last {windowDays} days
      </h2>
      <p class="text-sm text-muted">{data.from} – {data.to}</p>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <button
        class="btn-outline focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm shadow-sm"
        on:click={goPrev}
        data-testid="home-range-prev-button"
      >
        Prev
      </button>
      <button
        class="btn-outline focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm shadow-sm disabled:cursor-not-allowed disabled:opacity-60"
        on:click={goNext}
        disabled={!canNext}
        data-testid="home-range-next-button"
      >
        Next
      </button>
      <a
        class="btn-primary focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm shadow-sm"
        href="/sleep/new"
        data-testid="home-log-sleep-button"
      >
        + Log sleep
      </a>
    </div>
  </div>

  <SummaryStrip
    totalSessions={totalSessions}
    totalSleep={formatDurationMin(totalSleepMin)}
    avgDuration={totalSessions > 0 ? formatDurationMin(avgDurationMin) : '—'}
    avgQuality={avgQuality}
    avgLatency={totalSessions > 0 ? `${avgLatencyMin}m` : '—'}
  />

  <div class="surface-card flex flex-wrap items-center gap-2 rounded-xl px-4 py-3 text-sm">
    <label class="text-muted" for="jump-date">Jump to date</label>
    <input
      id="jump-date"
      type="date"
      class="input-base text-sm"
      bind:value={jumpTo}
      max={data.today}
      data-testid="home-jump-date-input"
    />
    <button
      class="btn-primary focus-ring touch-target inline-flex items-center rounded-full px-3.5 py-1.5 text-sm shadow-sm"
      on:click={jumpToDate}
      data-testid="home-jump-date-go-button"
    >
      Go
    </button>
  </div>

  {#if totalSessions === 0}
    <EmptyState />
  {:else}
    <div class="space-y-4">
      {#each rows as r (r.date)}
        <DayCard
          date={r.date}
          items={r.items}
          intensity={r.intensity}
          on:delete={handleDelete}
        />
      {/each}
    </div>
  {/if}
</section>
