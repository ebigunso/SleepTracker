<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let initials = 'ST';
  export let label = 'Profile';

  const dispatch = createEventDispatcher<{ logout: void }>();
  let open = false;
  let menuEl: HTMLDivElement | null = null;

  function toggle() {
    open = !open;
  }

  function close() {
    open = false;
  }

  function handleLogout() {
    close();
    dispatch('logout');
  }

  function onWindowClick(e: MouseEvent) {
    if (!open || !menuEl) return;
    if (!menuEl.contains(e.target as Node)) close();
  }

  function onWindowKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }
</script>

<svelte:window on:click={onWindowClick} on:keydown={onWindowKeydown} />

<div class="relative" bind:this={menuEl}>
  <button
    type="button"
    class="flex items-center gap-2 rounded-full border border-slate-200 bg-white px-2 py-1.5 text-sm font-semibold text-slate-700 shadow-sm hover:bg-slate-50"
    aria-haspopup="menu"
    aria-expanded={open}
    on:click={toggle}
  >
    <span class="flex h-8 w-8 items-center justify-center rounded-full bg-indigo-600 text-xs font-semibold uppercase text-white">
      {initials}
    </span>
    <span class="hidden sm:block">{label}</span>
    <svg class="h-4 w-4 text-slate-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
      <path
        fill-rule="evenodd"
        d="M5.23 7.21a.75.75 0 0 1 1.06.02L10 11.19l3.71-3.96a.75.75 0 1 1 1.08 1.04l-4.25 4.53a.75.75 0 0 1-1.08 0L5.21 8.27a.75.75 0 0 1 .02-1.06Z"
        clip-rule="evenodd"
      />
    </svg>
  </button>
  {#if open}
    <div
      class="absolute right-0 mt-2 w-44 rounded-xl border border-slate-200 bg-white p-1 shadow-lg ring-1 ring-slate-100"
      role="menu"
    >
      <button
        type="button"
        class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm font-semibold text-rose-600 hover:bg-rose-50"
        role="menuitem"
        on:click={handleLogout}
      >
        Logout
      </button>
    </div>
  {/if}
</div>
