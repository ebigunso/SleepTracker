<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/Button.svelte';
  import Card from '$lib/components/Card.svelte';

  export let open = false;
  export let title = 'Confirm';
  export let message = '';
  export let confirmText = 'Proceed';
  export let cancelText = 'Cancel';

  const dispatch = createEventDispatcher<{ confirm: void; cancel: void }>();

  function onConfirm() {
    dispatch('confirm');
    open = false;
  }
  function onCancel() {
    dispatch('cancel');
    open = false;
  }

  let dialogEl: HTMLDivElement | null = null;
  function onBackdrop(e: MouseEvent) {
    if (e.target === dialogEl) onCancel();
  }
  function onBackdropKeydown(e: KeyboardEvent) {
    if (e.target === dialogEl && (e.key === 'Enter' || e.key === ' ' || e.key === 'Spacebar')) {
      e.preventDefault();
      onCancel();
    }
  }

  // Close on Escape
  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onCancel();
  }
</script>

<svelte:window on:keydown={onKeydown} />

{#if open}
  <div
    bind:this={dialogEl}
    class="fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40"
    on:click={onBackdrop}
    on:keydown={onBackdropKeydown}
    aria-modal="true"
    role="dialog"
    tabindex="-1"
  >
    <Card className="w-full sm:max-w-sm rounded-t-lg sm:rounded-xl" padding="lg">
      <h3 class="section-title mb-2">{title}</h3>
      {#if message}
        <p class="text-sm text-slate-600 mb-4">{message}</p>
      {/if}
      <div class="mt-2 flex gap-2 justify-end">
        <Button variant="outline" size="sm" on:click={onCancel}>{cancelText}</Button>
        <Button variant="primary" size="sm" on:click={onConfirm}>{confirmText}</Button>
      </div>
    </Card>
  </div>
{/if}
