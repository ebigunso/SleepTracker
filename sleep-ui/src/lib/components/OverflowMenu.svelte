<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import Button from '$lib/components/Button.svelte';

  type MenuItem = {
    id?: string;
    label: string;
    disabled?: boolean;
    variant?: 'default' | 'danger';
  };

  export let items: ReadonlyArray<MenuItem> = [];
  export let label = 'Options';
  export let align: 'left' | 'right' = 'right';
  export let size: 'sm' | 'md' = 'sm';
  export let variant: 'ghost' | 'outline' = 'ghost';

  const dispatch = createEventDispatcher<{ select: MenuItem }>();

  let open = false;
  let menuEl: HTMLDivElement | null = null;

  function toggle() {
    open = !open;
  }

  function close() {
    open = false;
  }

  function onSelect(item: MenuItem) {
    if (item.disabled) return;
    dispatch('select', item);
    close();
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }

  onMount(() => {
    const handler = (e: MouseEvent) => {
      if (!open || !menuEl) return;
      if (!menuEl.contains(e.target as Node)) close();
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  });
</script>

<svelte:window on:keydown={onKeydown} />

<div class="relative inline-flex" bind:this={menuEl} role="presentation">
  <Button
    variant={variant === 'outline' ? 'outline' : 'ghost'}
    size={size}
    aria-haspopup="menu"
    aria-expanded={open}
    aria-label={label}
    on:click={toggle}
  >
    <slot name="trigger">⋯</slot>
  </Button>

  {#if open}
    <div
      class={`absolute z-20 mt-2 min-w-[10rem] rounded-lg border border-slate-200 bg-white shadow-lg ${
        align === 'right' ? 'right-0' : 'left-0'
      }`}
      role="menu"
    >
      {#each items as item (item.id ?? item.label)}
        <button
          type="button"
          class={`flex w-full items-center justify-between px-3 py-2 text-sm text-slate-700 hover:bg-slate-50 ${
            item.disabled ? 'cursor-not-allowed opacity-50 hover:bg-white' : ''
          } ${item.variant === 'danger' ? 'text-rose-600' : ''}`}
          on:click={() => onSelect(item)}
          role="menuitem"
          disabled={item.disabled}
        >
          {item.label}
        </button>
      {/each}
    </div>
  {/if}
</div>
