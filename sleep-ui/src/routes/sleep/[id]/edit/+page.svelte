<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { browser } from '$app/environment';
  import SleepForm from '$lib/components/SleepForm.svelte';
  import SleepEntryShell from '$lib/components/SleepEntryShell.svelte';
  import { goto } from '$app/navigation';
  import { deleteSleep, getExerciseIntensity } from '$lib/api';
  import type { SleepSession } from '$lib/api';
  import { exerciseIntensityByDate, removeRecentById, setIntensity } from '$lib/stores/sleep';
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
  const intensityDate = initialDate ?? data.rec.date;
  let initialIntensity: 'none' | 'light' | 'hard' = 'none';

  if (browser && intensityDate) {
    const stored = get(exerciseIntensityByDate)[intensityDate];
    if (stored !== undefined) {
      initialIntensity = stored;
    }
  }

  onMount(async () => {
    if (!intensityDate) return;
    const stored = get(exerciseIntensityByDate)[intensityDate];
    if (stored !== undefined) {
      initialIntensity = stored;
      return;
    }
    try {
      const list = await getExerciseIntensity(intensityDate, intensityDate);
      const found = list.find((x) => x.date === intensityDate);
      if (found?.intensity) {
        initialIntensity = found.intensity;
        setIntensity(intensityDate, found.intensity);
      }
    } catch {
      // ignore; intensity is optional
    }
  });

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

<SleepEntryShell
  title="Edit sleep entry"
  description="Update details or remove this session."
  headingTestId="sleep-edit-heading"
  formAnchorTestId="sleep-edit-form-anchor"
>
  <svelte:fragment slot="actions">
    <button
      class="btn-danger focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm font-semibold shadow-sm"
      on:click={onDelete}
      data-testid="sleep-edit-delete-button"
    >
      Delete
    </button>
  </svelte:fragment>

    <SleepForm
      mode="edit"
      {id}
      {initialDate}
      {initialBed}
      {initialWake}
      {initialLatency}
      {initialAwakenings}
      {initialQuality}
      {initialIntensity}
      on:saved={onSaved}
    />
</SleepEntryShell>
