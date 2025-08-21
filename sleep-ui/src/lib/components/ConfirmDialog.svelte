<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

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

  // Close on Escape
  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onCancel();
  }

  onMount(() => {
    window.addEventListener('keydown', onKeydown);
    return () => window.removeEventListener('keydown', onKeydown);
  });
</script>

{#if open}
  <div
    bind:this={dialogEl}
    class="fixed inset-0 z-50 flex items-end sm:items-center justify-center bg-black/40"
    on:click={onBackdrop}
    aria-modal="true"
    role="dialog"
  >
    <div class="w-full sm:max-w-sm bg-white rounded-t-lg sm:rounded-lg shadow-lg p-4 sm:p-6">
      <h3 class="text-lg font-semibold text-gray-900 mb-2">{title}</h3>
      {#if message}
        <p class="text-sm text-gray-700 mb-4">{message}</p>
      {/if}
      <div class="mt-2 flex gap-2 justify-end">
        <button
          class="inline-flex items-center rounded-md border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50"
          on:click={onCancel}
        >
          {cancelText}
        </button>
        <button
          class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold text-white hover:bg-indigo-700"
          on:click={onConfirm}
        >
          {confirmText}
        </button>
      </div>
    </div>
  </div>
{/if}
