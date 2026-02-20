<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/Button.svelte';
  import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
  import Input from '$lib/components/Input.svelte';
  import type {
    SleepInput,
    ExerciseUpsert,
    SleepSession,
    PersonalizationResponse,
    ActionRecommendation
  } from '$lib/api';
  import {
    createSleep,
    updateSleep,
    upsertExercise,
    apiPost,
    getDurationWarningBoundsFromMetric,
    getDurationWarningMessage,
    shouldShowDayTypeUsualTimesAction,
    applyDayTypeUsualTimes
  } from '$lib/api';
  import { upsertRecent, setIntensity } from '$lib/stores/sleep';
  import { pushToast } from '$lib/stores/toast';
  import { computeDurationMin, formatDurationMin } from '$lib/utils/sleep';

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
  export let showCancel = false;
  export let personalization: PersonalizationResponse | null = null;

  type DayType = 'weekday' | 'weekend';
  type TimePair = { bed: string; wake: string; dayType: DayType };

  const dispatch = createEventDispatcher<{ saved: SleepSession; deleted: void; cancel: void }>();

  let date = initialDate ?? today();
  let bed = normalizeTime(initialBed ?? getDefaultTimesForDate(date)?.bed ?? '22:00:00');
  let wake = normalizeTime(initialWake ?? getDefaultTimesForDate(date)?.wake ?? '06:00:00');
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
  const inputClass = 'mt-1 surface-2';

  $: dayTypeDefault = getDefaultTimesForDate(date);
  $: usualTimesActionEnabled = shouldShowDayTypeUsualTimesAction(
    findRecommendation('day_type_default_prefill'),
    dayTypeDefault !== null
  );
  $: durationBounds = getDurationWarningBoundsFromMetric(
    findRecommendation('personal_duration_warning_tuning'),
    personalization?.metrics?.duration_baseline
  );
  $: warningMessage = getDurationWarningMessage(durationBounds, formatDurationMin);

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

  function normalizeClockMinutes(minutes: number): number {
    const rounded = Math.round(minutes);
    return ((rounded % (24 * 60)) + (24 * 60)) % (24 * 60);
  }

  function minutesToIsoTime(minutes: number | null): string | null {
    if (minutes == null || !Number.isFinite(minutes)) return null;
    const value = normalizeClockMinutes(minutes);
    const hh = String(Math.floor(value / 60)).padStart(2, '0');
    const mm = String(value % 60).padStart(2, '0');
    return `${hh}:${mm}:00`;
  }

  function getDayType(value: string): DayType {
    const d = new Date(`${value}T00:00:00Z`);
    const weekday = d.getUTCDay();
    return weekday === 0 || weekday === 6 ? 'weekend' : 'weekday';
  }

  function findRecommendation(actionKey: string): ActionRecommendation | null {
    const found = personalization?.recommendations?.find((item) => item.action_key === actionKey);
    return found ?? null;
  }

  function getDefaultTimesForDate(value: string): TimePair | null {
    if (mode !== 'create') return null;
    const metric = personalization?.metrics?.day_type_timing_baseline;
    if (!metric?.eligible) return null;
    const dayType = getDayType(value);
    const bedMin = dayType === 'weekend' ? metric.weekend_bed_median_min : metric.weekday_bed_median_min;
    const wakeMin = dayType === 'weekend' ? metric.weekend_wake_median_min : metric.weekday_wake_median_min;
    const bedTime = minutesToIsoTime(bedMin);
    const wakeTime = minutesToIsoTime(wakeMin);
    if (!bedTime || !wakeTime) return null;
    return { bed: bedTime, wake: wakeTime, dayType };
  }

  function shouldWarn(durationMin: number): boolean {
    if (durationBounds) {
      return durationMin < durationBounds.min || durationMin > durationBounds.max;
    }
    return durationMin < 120 || durationMin > 14 * 60;
  }

  function applyUsualTimes() {
    const next = applyDayTypeUsualTimes(
      bed,
      wake,
      dayTypeDefault ? { bed: dayTypeDefault.bed, wake: dayTypeDefault.wake } : null,
      usualTimesActionEnabled
    );
    bed = normalizeTime(next.bed);
    wake = normalizeTime(next.wake);
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

  function onCancel() {
    dispatch('cancel');
  }
</script>

{#if errorMsg}
  <div class="state-card state-card--error mb-3" role="alert" data-testid="sleep-form-error-state">
    {errorMsg}
  </div>
{/if}

<form
  on:submit={onSubmit}
  class="space-y-4"
  aria-busy={loading}
  data-testid={`sleep-form-${mode}-state`}
>
  <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
    <div>
      <label for="date" class="meta-text">Date</label>
      <Input id="date" className={inputClass} type="date" bind:value={date} required />
    </div>
    <div>
      <label for="quality" class="meta-text">Quality (1-5)</label>
      <Input id="quality" className={inputClass} type="number" min="1" max="5" bind:value={quality} required />
    </div>
    <div>
      <label for="bed" class="meta-text">Bed time</label>
      <Input id="bed" className={inputClass} type="time" step="60" bind:value={bed} required />
    </div>
    <div>
      <label for="wake" class="meta-text">Wake time</label>
      <Input id="wake" className={inputClass} type="time" step="60" bind:value={wake} required />
    </div>
    {#if usualTimesActionEnabled}
      <div class="sm:col-span-2">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          disabled={loading}
          on:click={applyUsualTimes}
          data-testid="sleep-form-apply-usual-times-action"
        >
          Use your usual {dayTypeDefault?.dayType === 'weekend' ? 'weekend' : 'weekday'} times
        </Button>
      </div>
    {/if}
    <div>
      <label for="latency" class="meta-text">Latency (min)</label>
      <Input id="latency" className={inputClass} type="number" min="0" max="180" bind:value={latency} required />
    </div>
    <div>
      <label for="awakenings" class="meta-text">Awakenings</label>
      <Input id="awakenings" className={inputClass} type="number" min="0" max="10" bind:value={awakenings} required />
    </div>
    <div class="sm:col-span-2">
      <label for="intensity" class="meta-text">Exercise intensity</label>
      <Input
        id="intensity"
        as="select"
        className={inputClass}
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
      <Input id="notes" as="textarea" maxlength="280" rows={3} className={inputClass} bind:value={notes}></Input>
    </div>
  </div>

  <div class="flex justify-end gap-2">
    {#if showCancel}
      <Button
        type="button"
        variant="secondary"
        disabled={loading}
        on:click={onCancel}
        data-testid="sleep-form-cancel-action"
      >
        Cancel
      </Button>
    {/if}
    <Button type="submit" variant="primary" disabled={loading} data-testid="sleep-form-submit-action">
      {#if loading}{mode === 'create' ? 'Saving...' : 'Updating...'}{:else}{mode === 'create' ? 'Save' : 'Update'}{/if}
    </Button>
  </div>
</form>

<ConfirmDialog
  bind:open={warnOpen}
  title="Unusual duration"
  message={warningMessage}
  on:confirm={onProceed}
  on:cancel={onCancelWarn}
/>
