<script lang="ts">
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { goto } from '$app/navigation';
  import { createEventDispatcher, onMount } from 'svelte';
  import type { SleepSession } from '$lib/api';
  import { computeDurationMin, formatDurationMin, formatIsoTime } from '$lib/utils/sleep';

  export let item: SleepSession;

  const dispatch = createEventDispatcher<{
    delete: { id: number; date: string };
  }>();

  let menuOpen = false;
  let detailsEl: HTMLDetailsElement | null = null;

  function sessionDateFor(session: SleepSession): string {
    return session.session_date ?? session.date;
  }

  function durationFor(session: SleepSession): number {
    return session.duration_min ?? computeDurationMin(session.bed_time, session.wake_time);
  }

  function onEdit() {
    const sessionDate = sessionDateFor(item);
    goto(`/sleep/${item.id}/edit?date=${encodeURIComponent(sessionDate)}`);
    menuOpen = false;
  }

  function onDelete() {
    dispatch('delete', { id: item.id, date: sessionDateFor(item) });
    menuOpen = false;
  }

  onMount(() => {
    const handleDocumentClick = (event: MouseEvent) => {
      if (!menuOpen) return;
      const target = event.target as Node | null;
      if (detailsEl && target && detailsEl.contains(target)) return;
      menuOpen = false;
    };

    document.addEventListener('click', handleDocumentClick, true);
    return () => {
      document.removeEventListener('click', handleDocumentClick, true);
    };
  });

  $: timeRange = `${formatIsoTime(item.bed_time)}–${formatIsoTime(item.wake_time)}`;
  $: durationLabel = formatDurationMin(durationFor(item));
  $: qualityLabel = item.quality ?? '—';
</script>

<div class="card px-4 py-3">
  <div class="flex items-start justify-between gap-4">
    <div class="min-w-0">
      <p class="text-sm font-semibold text-default">{timeRange}</p>
      <div class="mt-1 flex flex-wrap gap-3 text-xs text-muted">
        <span>Duration <span class="font-medium text-default">{durationLabel}</span></span>
        <span>Quality <span class="font-medium text-default">{qualityLabel}</span></span>
      </div>
    </div>
    <details bind:open={menuOpen} bind:this={detailsEl} class="relative">
      <summary
        class="inline-flex h-8 w-8 list-none items-center justify-center rounded-full border border-[color:var(--color-border)] text-muted hover:bg-[color:var(--color-surface-muted)]"
        aria-label="Session actions"
      >
        &#8942;
      </summary>
      <div class="surface-card absolute right-0 z-10 mt-2 w-28 rounded-lg p-1 text-sm">
        <button
          class="flex w-full items-center rounded-md px-2 py-1.5 text-left text-default hover:bg-[color:var(--color-surface-muted)]"
          on:click={onEdit}
        >
          Edit
        </button>
        <button
          class="flex w-full items-center rounded-md px-2 py-1.5 text-left text-rose-600 hover:bg-rose-50"
          on:click={onDelete}
        >
          Delete
        </button>
      </div>
    </details>
  </div>
  <div class="mt-3">
    <SleepBar bed_time={item.bed_time} wake_time={item.wake_time} />
  </div>
</div>
