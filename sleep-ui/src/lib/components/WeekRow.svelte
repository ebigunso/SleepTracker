<script lang="ts">
  import Button from '$lib/components/Button.svelte';
  import Card from '$lib/components/Card.svelte';
  import Chip from '$lib/components/Chip.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import OverflowMenu from '$lib/components/OverflowMenu.svelte';
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';
  import { createEventDispatcher } from 'svelte';
  import type { SleepSession } from '$lib/api';
  import { computeDurationMin } from '$lib/utils/sleep';
  import { formatDuration, formatTimeHHmm } from '$lib/utils/time';

  export let date: string; // YYYY-MM-DD (display date)
  export let items: SleepSession[] = [];
  export let intensity: 'none' | 'light' | 'hard' | undefined;

  const dispatch = createEventDispatcher<{
    delete: { id: number; date: string };
  }>();

  function onAdd() {
    // Prefill date in query for Quick Log
    goto(`/sleep/new?date=${encodeURIComponent(date)}`);
  }

  function onEdit(id: number, itemDate: string) {
    goto(`/sleep/${id}/edit?date=${encodeURIComponent(itemDate)}`);
  }

  function sessionDateFor(item: SleepSession): string {
    return item.session_date ?? item.date;
  }

  function onDelete(id: number) {
    dispatch('delete', { id, date });
  }

  function fmtMin(n: number | null | undefined): string {
    return formatDuration(n ?? null);
  }

  function durationFor(item: SleepSession): number {
    return item.duration_min ?? computeDurationMin(item.bed_time, item.wake_time);
  }

  $: sortedItems = [...items].sort((a, b) => {
    if (a.bed_time !== b.bed_time) return a.bed_time < b.bed_time ? -1 : 1;
    return a.id < b.id ? -1 : 1;
  });

  $: sessionCount = sortedItems.length;
  $: totalDuration = sortedItems.reduce((sum, it) => sum + durationFor(it), 0);
  $: avgQuality = sessionCount > 0
    ? Math.round(sortedItems.reduce((sum, it) => sum + (it.quality ?? 0), 0) / sessionCount)
    : null;
  $: avgLatency = sessionCount > 0
    ? Math.round(sortedItems.reduce((sum, it) => sum + (it.latency_min ?? 0), 0) / sessionCount)
    : null;
  $: totalAwakenings = sortedItems.reduce((sum, it) => sum + (it.awakenings ?? 0), 0);

  const badgeColor =
    intensity === 'hard'
      ? 'success'
      : intensity === 'light'
      ? 'info'
      : 'neutral';

  const itemActions = [
    { id: 'edit', label: 'Edit' },
    { id: 'delete', label: 'Delete', variant: 'danger' }
  ] as const;

  function onSelectAction(actionId: string, item: SleepSession) {
    if (actionId === 'edit') {
      onEdit(item.id, sessionDateFor(item));
      return;
    }
    if (actionId === 'delete') {
      onDelete(item.id);
    }
  }
</script>

<div class="flex items-center gap-3 py-3 border-b border-gray-200">
  <div class="w-28 shrink-0">
    <a class="section-title text-sm text-indigo-600 hover:text-indigo-500" href={`/day/${date}`}>{date}</a>
  </div>

  {#if sessionCount > 0}
    <div class="flex-1 min-w-0 space-y-2">
      <div class="flex flex-wrap items-center gap-3 text-xs text-gray-600">
        <span><span class="meta-text">Sessions</span> <span class="font-medium text-slate-800">{sessionCount}</span></span>
        <span><span class="meta-text">Total</span> <span class="text-base stat-value">{fmtMin(totalDuration)}</span></span>
        <span><span class="meta-text">Avg Quality</span> <span class="font-medium text-slate-800">{avgQuality ?? '—'}</span></span>
        <span><span class="meta-text">Avg Latency</span> <span class="font-medium text-slate-800">{avgLatency ?? '—'}m</span></span>
        <span><span class="meta-text">Awakenings</span> <span class="font-medium text-slate-800">{totalAwakenings}</span></span>
        {#if intensity}
          <Chip variant={badgeColor} size="sm">Exercise: {intensity}</Chip>
        {/if}
      </div>
      <div class="space-y-2">
        {#each sortedItems as item (item.id)}
          <Card padding="md">
            <SleepBar bed_time={item.bed_time} wake_time={item.wake_time} />
            <div class="mt-2 flex flex-wrap items-center gap-2 text-xs text-gray-600">
              <span><span class="meta-text">Bed</span> <span class="font-medium text-slate-800">{formatTimeHHmm(item.bed_time)}</span></span>
              <span><span class="meta-text">Wake</span> <span class="font-medium text-slate-800">{formatTimeHHmm(item.wake_time)}</span></span>
              <span><span class="meta-text">Duration</span> <span class="font-medium text-slate-800">{fmtMin(durationFor(item))}</span></span>
              <span><span class="meta-text">Quality</span> <span class="font-medium text-slate-800">{item.quality}</span></span>
              <span><span class="meta-text">Latency</span> <span class="font-medium text-slate-800">{item.latency_min}m</span></span>
            </div>
            <div class="mt-3 flex justify-end">
              <div class="hidden sm:flex gap-2">
                <Button
                  variant="secondary"
                  size="sm"
                  on:click={() => onEdit(item.id, sessionDateFor(item))}
                  aria-label="Edit"
                >
                  Edit
                </Button>
                <Button variant="danger" size="sm" on:click={() => onDelete(item.id)} aria-label="Delete">
                  Delete
                </Button>
              </div>
              <div class="sm:hidden">
                <OverflowMenu
                  items={itemActions}
                  on:select={(event) => onSelectAction(event.detail.id ?? event.detail.label.toLowerCase(), item)}
                >
                  <span slot="trigger">Actions</span>
                </OverflowMenu>
              </div>
            </div>
          </Card>
        {/each}
      </div>
    </div>
    <div class="flex gap-2 shrink-0">
      <Button size="sm" on:click={onAdd}>Add entry</Button>
    </div>
  {:else}
    <div class="flex-1">
      <EmptyState title="No entry" message="Log your sleep to start tracking trends.">
        <span slot="action">
          <Button size="sm" on:click={onAdd}>Add entry</Button>
        </span>
      </EmptyState>
    </div>
    <div class="shrink-0">
      <Button size="sm" on:click={onAdd}>Add entry</Button>
    </div>
  {/if}
</div>
