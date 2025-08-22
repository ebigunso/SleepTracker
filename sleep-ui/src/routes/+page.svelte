<script lang="ts">
  import WeekRow from '$lib/components/WeekRow.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import type { SleepListItem } from '$lib/api';
  import { deleteSleep } from '$lib/api';
  import { recentSleep, exerciseIntensityByDate, removeRecentById } from '$lib/stores/sleep';

  export let data: { recent: SleepListItem[] };

  onMount(() => {
    // seed store with server-fetched recent data
    recentSleep.set(data.recent ?? []);
  });

  function isoDate(d: Date): string {
    // Format date in local time to avoid UTC shift (which excluded "today" in non-UTC TZs)
    const y = d.getFullYear();
    const m = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    return `${y}-${m}-${day}`;
  }

  const last7Dates = (() => {
    const today = new Date();
    const arr: string[] = [];
    for (let i = 0; i < 7; i++) {
      const d = new Date(today);
      d.setDate(today.getDate() - i);
      arr.push(isoDate(d));
    }
    return arr; // desc: today..today-6
  })();

  $: intensityMap = $exerciseIntensityByDate;

  $: rows = last7Dates.map((date) => {
    const item = $recentSleep.find((x) => x.date === date) ?? null;
    const intensity = intensityMap[date];
    return { date, item, intensity };
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
</script>

<section class="space-y-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xl font-semibold text-gray-900">This week</h2>
    <button
      class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-700"
      on:click={quickLog}
    >
      Quick Log
    </button>
  </div>

  <div class="rounded-md bg-white shadow divide-y divide-gray-200">
    {#each rows as r (r.date)}
      <WeekRow date={r.date} item={r.item} intensity={r.intensity} on:delete={handleDelete} />
    {/each}
  </div>
</section>
