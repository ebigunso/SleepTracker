<script lang="ts">
  import SleepForm from '$lib/components/SleepForm.svelte';
  import { page } from '$app/stores';
  import { get } from 'svelte/store';
  import { goto } from '$app/navigation';
import { apiGet, deleteSleep } from '$lib/api';
import type { SleepListItem } from '$lib/api';
  import { removeRecentById } from '$lib/stores/sleep';
  import { pushToast } from '$lib/stores/toast';

  const p = get(page);
  const params = p.params as unknown as { id: string };
  const id = Number(params.id);

  // Expect date via query (?date=YYYY-MM-DD) to use GET by date
  const dateParam = p.url.searchParams.get('date');

  let loading = true;
  let errorMsg: string | null = null;

  // Form initial values
  let initialDate: string | null = dateParam;
  let initialBed: string | null = null;
  let initialWake: string | null = null;
  let initialLatency = 0;
  let initialAwakenings = 0;
  let initialQuality = 3;
  let initialIntensity: 'none' | 'light' | 'hard' = 'none';
  let initialNotes = '';

  function normalizeTime(t: string): string {
    // ensure HH:mm:ss
    if (!t) return '00:00:00';
    const parts = t.split(':');
    if (parts.length === 2) return `${parts[0]}:${parts[1]}:00`;
    if (parts.length >= 3) return `${parts[0]}:${parts[1]}:${parts[2]}`;
    return '00:00:00';
  }

  async function loadByDate(date: string) {
    try {
      const rec = await apiGet<SleepListItem>(`/api/sleep/date/${date}`);
      initialDate = rec.date;
      initialBed = normalizeTime(rec.bed_time);
      initialWake = normalizeTime(rec.wake_time);
      initialLatency = rec.latency_min ?? 0;
      initialAwakenings = rec.awakenings ?? 0;
      initialQuality = rec.quality ?? 3;
      // Intensity is not part of this payload; leave default or get from a separate endpoint in future
    } catch (e: any) {
      errorMsg = `Failed to load record for ${date}: ${e?.message ?? 'error'}`;
    } finally {
      loading = false;
    }
  }

  if (dateParam) {
    loadByDate(dateParam);
  } else {
    loading = false;
    errorMsg = 'Missing ?date=YYYY-MM-DD to load entry';
  }

  async function onDelete() {
    if (!confirm('Delete this entry?')) return;
    try {
      await deleteSleep(id);
      removeRecentById(id);
      pushToast({ type: 'success', message: 'Deleted' });
      goto('/');
    } catch (e: any) {
      pushToast({ type: 'error', message: e?.message ?? 'Delete failed' });
    }
  }

  function onSaved() {
    goto('/');
  }
</script>

<section class="space-y-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xl font-semibold text-gray-900">Edit sleep entry</h2>
    <button
      class="inline-flex items-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white hover:bg-red-700"
      on:click={onDelete}
    >
      Delete
    </button>
  </div>

  {#if errorMsg}
    <div class="mb-3 rounded border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
      {errorMsg}
    </div>
  {/if}

  {#if loading}
    <p class="text-gray-600 text-sm">Loading…</p>
  {:else}
    <SleepForm
      mode="edit"
      {id}
      {initialDate}
      {initialBed}
      {initialWake}
      {initialLatency}
      {initialAwakenings}
      {initialQuality}
      on:saved={onSaved}
    />
  {/if}
</section>
