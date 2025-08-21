<script lang="ts">
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';

  export let data: {
    date: string;
    item: {
      id: number;
      date: string;
      bed_time: string;
      wake_time: string;
      latency_min: number;
      awakenings: number;
      quality: number;
      duration_min: number | null;
    } | null;
  };

  function edit() {
    if (!data.item) return;
    goto(`/sleep/${data.item.id}/edit?date=${encodeURIComponent(data.date)}`);
  }

  function fmtMin(n: number | null | undefined): string {
    if (n == null) return '—';
    const h = Math.floor(n / 60);
    const m = n % 60;
    return `${h}h ${m}m`;
  }
</script>

<section class="space-y-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xl font-semibold text-gray-900">Day view: {data.date}</h2>
    {#if data.item}
      <button
        class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-700"
        on:click={edit}
      >
        Edit
      </button>
    {/if}
  </div>

  {#if data.item}
    <div class="rounded-md bg-white shadow p-4 space-y-3">
      <SleepBar bed_time={data.item.bed_time} wake_time={data.item.wake_time} />
      <div class="grid grid-cols-2 sm:grid-cols-3 gap-x-6 gap-y-2 text-sm text-gray-700">
        <div><span class="text-gray-500">Bed:</span> <span class="font-medium">{data.item.bed_time}</span></div>
        <div><span class="text-gray-500">Wake:</span> <span class="font-medium">{data.item.wake_time}</span></div>
        <div><span class="text-gray-500">Duration:</span> <span class="font-medium">{fmtMin(data.item.duration_min)}</span></div>
        <div><span class="text-gray-500">Latency:</span> <span class="font-medium">{data.item.latency_min}m</span></div>
        <div><span class="text-gray-500">Awakenings:</span> <span class="font-medium">{data.item.awakenings}</span></div>
        <div><span class="text-gray-500">Quality:</span> <span class="font-medium">{data.item.quality}</span></div>
      </div>
    </div>
  {:else}
    <div class="rounded-md border border-gray-200 bg-white p-4 text-gray-700">
      No sleep entry for this date.
    </div>
  {/if}
</section>
