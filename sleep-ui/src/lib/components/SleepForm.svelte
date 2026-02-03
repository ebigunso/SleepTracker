<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/Button.svelte';
  import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
  import Input from '$lib/components/Input.svelte';
  import type { SleepInput, ExerciseUpsert, SleepSession } from '$lib/api';
  import { createSleep, updateSleep, upsertExercise, apiPost } from '$lib/api';
  import { upsertRecent, setIntensity } from '$lib/stores/sleep';
  import { pushToast } from '$lib/stores/toast';
  import { computeDurationMin } from '$lib/utils/sleep';

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

  const dispatch = createEventDispatcher<{ saved: SleepSession; deleted: void }>();

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

  function toSessionItem(input: SleepInput, idNum: number): SleepSession {
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

      const item = toSessionItem(input, savedId);
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
  <div class="state-card state-card--error mb-3" role="alert">
    {errorMsg}
  </div>
{/if}

<form on:submit={onSubmit} class="space-y-4" aria-busy={loading}>
  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
    <div>
      <label for="date" class="meta-text">Date</label>
      <Input id="date" className="mt-1" type="date" bind:value={date} required />
    </div>
    <div>
      <label for="quality" class="meta-text">Quality (1-5)</label>
      <Input id="quality" className="mt-1" type="number" min="1" max="5" bind:value={quality} required />
    </div>
    <div>
      <label for="bed" class="meta-text">Bed time</label>
      <Input id="bed" className="mt-1" type="time" step="60" bind:value={bed} required />
    </div>
    <div>
      <label for="wake" class="meta-text">Wake time</label>
      <Input id="wake" className="mt-1" type="time" step="60" bind:value={wake} required />
    </div>
    <div>
      <label for="latency" class="meta-text">Latency (min)</label>
      <Input id="latency" className="mt-1" type="number" min="0" max="180" bind:value={latency} required />
    </div>
    <div>
      <label for="awakenings" class="meta-text">Awakenings</label>
      <Input id="awakenings" className="mt-1" type="number" min="0" max="10" bind:value={awakenings} required />
    </div>
    <div class="sm:col-span-2">
      <label for="intensity" class="meta-text">Exercise intensity</label>
      <Input
        id="intensity"
        as="select"
        className="mt-1"
        bind:value={intensity}
        on:change={() => (intensityDirty = true)}
      >
        <option value="none">none</option>
        <option value="light">light</option>
        <option value="hard">hard</option>
      </Input>
    </div>
    <div class="sm:col-span-2">
      <label for="notes" class="meta-text">Notes (optional, ≤280)</label>
      <Input id="notes" as="textarea" maxlength="280" rows={3} className="mt-1" bind:value={notes}></Input>
    </div>
  </div>

  <div class="flex justify-end gap-2">
    <Button type="submit" disabled={loading}>
      {#if loading}{mode === 'create' ? 'Saving...' : 'Updating...'}{:else}{mode === 'create' ? 'Save' : 'Update'}{/if}
    </Button>
  </div>
</form>

<ConfirmDialog
  bind:open={warnOpen}
  title="Unusual duration"
  message="The sleep duration is < 2h or > 14h. Proceed anyway?"
  on:confirm={onProceed}
  on:cancel={onCancelWarn}
/>
