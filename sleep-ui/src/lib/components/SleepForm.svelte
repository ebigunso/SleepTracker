<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
  import type { SleepInput, ExerciseUpsert, SleepListItem } from '$lib/api';
  import { createSleep, updateSleep, upsertExercise, apiPost } from '$lib/api';
  import { upsertRecent, setIntensity } from '$lib/stores/sleep';
  import { pushToast } from '$lib/stores/toast';

  /**
   * Props
   */
  export let mode: 'create' | 'edit' = 'create';
  export let id: number | null = null;
  export let initialDate: string | null = null; // YYYY-MM-DD
  export let initialBed: string | null = null;  // HH:mm or HH:mm:ss
  export let initialWake: string | null = null; // HH:mm or HH:mm:ss
  export let initialLatency = 0;
  export let initialAwakenings = 0;
  export let initialQuality = 3;
  export let initialIntensity: 'none' | 'light' | 'hard' = 'none';
  export let initialNotes: string = '';

  const dispatch = createEventDispatcher<{ saved: SleepListItem; deleted: void }>();

  let date = initialDate ?? today();
  let bed = normalizeTime(initialBed ?? '22:00:00');
  let wake = normalizeTime(initialWake ?? '06:00:00');
  let latency = initialLatency;
  let awakenings = initialAwakenings;
  let quality = initialQuality;
  let intensity: 'none' | 'light' | 'hard' = initialIntensity;
  let notes = initialNotes;

  let intensityDirty = false;

  let loading = false;
  let errorMsg: string | null = null;

  let warnOpen = false;
  let pendingSubmit = false;

  $: if (!intensityDirty && intensity !== initialIntensity) {
    intensity = initialIntensity;
  }

  function today(): string {
    const d = new Date();
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, '0');
    const dd = String(d.getDate()).padStart(2, '0');
    return `${yyyy}-${mm}-${dd}`;
  }

  function normalizeTime(t: string): string {
    // ensure HH:mm:ss
    if (!t) return '00:00:00';
    const parts = t.split(':');
    if (parts.length === 2) return `${parts[0]}:${parts[1]}:00`;
    if (parts.length >= 3) return `${parts[0]}:${parts[1]}:${parts[2]}`;
    return '00:00:00';
  }

  function toMinutes(t: string): number {
    const [hh, mm, ss] = t.split(':').map((v) => parseInt(v, 10));
    return (hh || 0) * 60 + (mm || 0);
  }

  function computeDurationMin(bedTime: string, wakeTime: string): number {
    const bedMin = toMinutes(bedTime);
    const wakeMin = toMinutes(wakeTime);
    if (bedMin <= wakeMin) return wakeMin - bedMin;
    // wrap across midnight
    return (24 * 60 - bedMin) + wakeMin;
  }

  function shouldWarn(durationMin: number): boolean {
    return durationMin < 120 || durationMin > 14 * 60;
  }

  function buildSleepInput(): SleepInput {
    return {
      date,
      bed_time: normalizeTime(bed),
      wake_time: normalizeTime(wake),
      latency_min: Number(latency),
      awakenings: Number(awakenings),
      quality: Number(quality)
    };
  }

  function toListItem(input: SleepInput, idNum: number): SleepListItem {
    // Build a client-side projection to update dashboard without refetch.
    const duration = computeDurationMin(input.bed_time, input.wake_time);
    return {
      id: idNum,
      date: input.date,
      bed_time: input.bed_time,
      wake_time: input.wake_time,
      latency_min: input.latency_min,
      awakenings: input.awakenings,
      quality: input.quality,
      duration_min: duration
    };
  }

  async function submitInner() {
    loading = true;
    errorMsg = null;
    try {
      const input = buildSleepInput();
      let savedId: number;
      if (mode === 'create') {
        const res = await createSleep(input);
        savedId = res.id;
      } else {
        if (id == null) throw new Error('Missing id for edit mode');
        await updateSleep(id, input);
        savedId = id;
      }

      // Exercise upsert (best-effort)
      const ex: ExerciseUpsert = { date: input.date, intensity };
      try {
        await upsertExercise(ex);
      } catch {
        // ignore errors for exercise upsert; it's optional
      }
      // Update intensity badge in dashboard store
      setIntensity(input.date, intensity);

      // Optional note (best-effort)
      if (notes.trim().length > 0 && notes.trim().length <= 280) {
        try {
          await apiPost('/api/note', { date: input.date, body: notes.trim() });
        } catch {
          // swallow
        }
      }

      const item = toListItem(input, savedId);
      upsertRecent(item);
      pushToast({ type: 'success', message: mode === 'create' ? 'Saved' : 'Updated' });
      dispatch('saved', item);
    } catch (e: any) {
      errorMsg = e?.message ?? 'Failed to save';
    } finally {
      loading = false;
      pendingSubmit = false;
    }
  }

  function onSubmit(e: Event) {
    e.preventDefault();
    const input = buildSleepInput();
    const dur = computeDurationMin(input.bed_time, input.wake_time);
    if (shouldWarn(dur)) {
      pendingSubmit = true;
      warnOpen = true;
      return;
    }
    submitInner();
  }

  function onProceed() {
    warnOpen = false;
    if (pendingSubmit) {
      submitInner();
    }
  }

  function onCancelWarn() {
    warnOpen = false;
    pendingSubmit = false;
  }
</script>

{#if errorMsg}
  <div class="mb-3 rounded border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
    {errorMsg}
  </div>
{/if}

<form on:submit={onSubmit} class="space-y-4">
  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
    <div>
      <label for="date" class="block text-sm font-medium text-gray-700">Date</label>
      <input id="date" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="date" bind:value={date} required />
    </div>
    <div>
      <label for="quality" class="block text-sm font-medium text-gray-700">Quality (1-5)</label>
      <input id="quality" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="number" min="1" max="5" bind:value={quality} required />
    </div>
    <div>
      <label for="bed" class="block text-sm font-medium text-gray-700">Bed time</label>
      <input id="bed" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="time" step="60" bind:value={bed} required />
    </div>
    <div>
      <label for="wake" class="block text-sm font-medium text-gray-700">Wake time</label>
      <input id="wake" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="time" step="60" bind:value={wake} required />
    </div>
    <div>
      <label for="latency" class="block text-sm font-medium text-gray-700">Latency (min)</label>
      <input id="latency" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="number" min="0" max="180" bind:value={latency} required />
    </div>
    <div>
      <label for="awakenings" class="block text-sm font-medium text-gray-700">Awakenings</label>
      <input id="awakenings" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" type="number" min="0" max="10" bind:value={awakenings} required />
    </div>
    <div class="sm:col-span-2">
      <label for="intensity" class="block text-sm font-medium text-gray-700">Exercise intensity</label>
      <select
        id="intensity"
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
        bind:value={intensity}
        on:change={() => (intensityDirty = true)}
      >
        <option value="none">none</option>
        <option value="light">light</option>
        <option value="hard">hard</option>
      </select>
    </div>
    <div class="sm:col-span-2">
      <label for="notes" class="block text-sm font-medium text-gray-700">Notes (optional, ≤280)</label>
      <textarea id="notes" maxlength="280" rows="3" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" bind:value={notes}></textarea>
    </div>
  </div>

  <div class="flex justify-end gap-2">
    <button type="submit" class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-700 disabled:opacity-60" disabled={loading}>
      {#if loading}{mode === 'create' ? 'Saving...' : 'Updating...'}{:else}{mode === 'create' ? 'Save' : 'Update'}{/if}
    </button>
  </div>
</form>

<ConfirmDialog
  bind:open={warnOpen}
  title="Unusual duration"
  message="The sleep duration is < 2h or > 14h. Proceed anyway?"
  on:confirm={onProceed}
  on:cancel={onCancelWarn}
/>
