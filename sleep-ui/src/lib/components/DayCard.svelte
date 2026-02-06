<script lang="ts">
  import SessionRow from '$lib/components/SessionRow.svelte';
  import Chip from '$lib/components/Chip.svelte';
  import { createEventDispatcher } from 'svelte';
  import type { SleepSession } from '$lib/api';
  import { computeDurationMin, formatDurationMin } from '$lib/utils/sleep';

  export let date: string;
  export let items: SleepSession[] = [];
  export let intensity: 'none' | 'light' | 'hard' | undefined;

  type ChipVariant = 'neutral' | 'primary' | 'success' | 'warning' | 'info';

  const dispatch = createEventDispatcher<{
    delete: { id: number; date: string };
  }>();

  function durationFor(session: SleepSession): number {
    return session.duration_min ?? computeDurationMin(session.bed_time, session.wake_time);
  }

  $: sortedItems = [...items].sort((a, b) => {
    if (a.bed_time !== b.bed_time) return a.bed_time < b.bed_time ? -1 : 1;
    return a.id < b.id ? -1 : 1;
  });

  $: sessionCount = sortedItems.length;
  $: totalDuration = sortedItems.reduce((sum, it) => sum + durationFor(it), 0);
  $: avgQualityValue = sessionCount > 0
    ? Math.round(sortedItems.reduce((sum, it) => sum + (it.quality ?? 0), 0) / sessionCount)
    : null;
  $: avgQualityLabel = avgQualityValue == null ? '—' : `${avgQualityValue}`;

  let intensityVariant: ChipVariant = 'neutral';

  $: intensityVariant =
    intensity === 'hard'
      ? 'success'
      : intensity === 'light'
      ? 'info'
      : 'neutral';

  function handleDelete(e: CustomEvent<{ id: number; date: string }>) {
    dispatch('delete', e.detail);
  }
</script>

<article class="surface-card w-full rounded-2xl p-4">
  <header class="flex flex-wrap items-start justify-between gap-4">
    <div>
      <a
        class="text-lg font-semibold text-[color:var(--color-text)] hover:text-[color:var(--color-primary)]"
        href={`/day/${date}`}
      >{date}</a>
      <p class="text-sm text-muted">Total {formatDurationMin(totalDuration)}</p>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <Chip variant="neutral">{sessionCount} sessions</Chip>
      <Chip variant="neutral">Avg quality {avgQualityLabel}</Chip>
      {#if intensity}
        <Chip variant={intensityVariant}>Exercise {intensity}</Chip>
      {/if}
      <a class="text-xs font-semibold text-[color:var(--color-primary)] hover:text-[color:var(--color-primary-hover)]" href={`/day/${date}`}>View day</a>
    </div>
  </header>

  <div class="mt-4 space-y-3">
    {#if sessionCount > 0}
      {#each sortedItems as item (item.id)}
        <SessionRow item={item} on:delete={handleDelete} />
      {/each}
    {:else}
      <div class="empty-state-box rounded-xl px-4 py-6 text-sm">
        No sleep logged for this day.
      </div>
    {/if}
  </div>
</article>
