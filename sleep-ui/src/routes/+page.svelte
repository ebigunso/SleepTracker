<script lang="ts">
  import WeekRow from '$lib/components/WeekRow.svelte';
  import { goto } from '$app/navigation';
  import { browser } from '$app/environment';
  import type { SleepSession } from '$lib/api';
  import { deleteSleep } from '$lib/api';
  import { recentSleep, exerciseIntensityByDate, removeRecentById } from '$lib/stores/sleep';

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

  function quickLog() {
    goto('/sleep/new');
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

<section class="space-y-4">
  <div class="flex flex-wrap items-start justify-between gap-3">
    <div>
      <h2 class="text-xl font-semibold text-gray-900" data-testid="dashboard-heading">
        Last {windowDays} days
      </h2>
      <p class="text-sm text-gray-600">{data.from} – {data.to}</p>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <button
        class="inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-700 ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
        on:click={goPrev}
      >
        Prev
      </button>
      <button
        class={`inline-flex items-center rounded-md px-3 py-2 text-sm font-semibold ring-1 ring-inset ${
          canNext ? 'bg-white text-gray-700 ring-gray-300 hover:bg-gray-50' : 'bg-gray-100 text-gray-400 ring-gray-200'
        }`}
        on:click={goNext}
        disabled={!canNext}
      >
        Next
      </button>
      <button
        class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-700"
        on:click={quickLog}
      >
        Quick Log
      </button>
    </div>
  </div>

  <div class="flex flex-wrap items-center gap-2 text-sm">
    <label class="text-gray-600" for="jump-date">Jump to date</label>
    <input
      id="jump-date"
      type="date"
      class="rounded-md border border-gray-300 px-2 py-1 text-sm text-gray-700"
      bind:value={jumpTo}
      max={data.today}
    />
    <button
      class="inline-flex items-center rounded-md bg-white px-3 py-1.5 text-sm font-semibold text-gray-700 ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
      on:click={jumpToDate}
    >
      Go
    </button>
  </div>

  <div class="rounded-md bg-white shadow divide-y divide-gray-200">
    {#each rows as r (r.date)}
      <WeekRow
        date={r.date}
        items={r.items}
        intensity={r.intensity}
        on:delete={handleDelete}
      />
    {/each}
  </div>
</section>
