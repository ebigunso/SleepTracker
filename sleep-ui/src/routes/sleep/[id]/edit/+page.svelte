<script lang="ts">
  import SleepForm from '$lib/components/SleepForm.svelte';
  import { goto } from '$app/navigation';
  import { deleteSleep } from '$lib/api';
  import type { SleepSession } from '$lib/api';
  import { removeRecentById } from '$lib/stores/sleep';
  import { pushToast } from '$lib/stores/toast';

  export let data: {
    rec: SleepSession;
    qDate: string | null;
  };

  const id = Number(data.rec.id);

  function normalizeTime(t: string): string {
    // ensure HH:mm:ss
    if (!t) return '00:00:00';
    const parts = t.split(':');
    if (parts.length === 2) return `${parts[0]}:${parts[1]}:00`;
    if (parts.length >= 3) return `${parts[0]}:${parts[1]}:${parts[2]}`;
    return '00:00:00';
  }

  // Form initial values derived from server data and optional query param (?date=YYYY-MM-DD)
  const initialDate: string | null = data.qDate ?? data.rec.date;
  const initialBed: string | null = normalizeTime(data.rec.bed_time);
  const initialWake: string | null = normalizeTime(data.rec.wake_time);
  const initialLatency = (data.rec as any)?.latency_min ?? 0;
  const initialAwakenings = (data.rec as any)?.awakenings ?? 0;
  const initialQuality = (data.rec as any)?.quality ?? 3;

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
</section>
