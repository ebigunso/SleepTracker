<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let initials = 'ST';
  export let label = 'Profile';
  export let theme: 'light' | 'dark' = 'light';

  const dispatch = createEventDispatcher<{ logout: void; toggleTheme: void }>();
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

  function handleToggleTheme() {
    close();
    dispatch('toggleTheme');
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
    class="menu-trigger flex items-center gap-2 rounded-full px-2 py-1.5 text-sm font-semibold shadow-sm"
    aria-haspopup="menu"
    aria-expanded={open}
    on:click={toggle}
  >
    <span class="brand-badge flex h-8 w-8 items-center justify-center rounded-full text-xs font-semibold uppercase">
      {initials}
    </span>
    <span class="hidden sm:block">{label}</span>
    <svg class="menu-trigger-icon h-4 w-4" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
      <path
        fill-rule="evenodd"
        d="M5.23 7.21a.75.75 0 0 1 1.06.02L10 11.19l3.71-3.96a.75.75 0 1 1 1.08 1.04l-4.25 4.53a.75.75 0 0 1-1.08 0L5.21 8.27a.75.75 0 0 1 .02-1.06Z"
        clip-rule="evenodd"
      />
    </svg>
  </button>
  {#if open}
    <div
      class="menu-dropdown absolute right-0 mt-2 w-52 rounded-xl p-0"
      role="menu"
    >
      <button
        type="button"
        class="menu-item flex w-full items-center gap-2 px-3 py-2 text-sm font-semibold"
        role="menuitem"
        on:click={handleToggleTheme}
      >
        <img
          class="menu-item-icon"
          src={theme === 'dark' ? '/icons/sun-brightness.png' : '/icons/moon-night.png'}
          alt=""
          aria-hidden="true"
        />
        <span class="menu-item-label">{theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}</span>
      </button>
      <div class="menu-divider" role="separator"></div>
      <button
        type="button"
        class="menu-item menu-item--danger flex w-full items-center gap-2 px-3 py-2 text-sm font-semibold"
        role="menuitem"
        on:click={handleLogout}
      >
        <span class="menu-item-icon menu-item-icon--danger" aria-hidden="true"></span>
        <span class="menu-item-label">Logout</span>
      </button>
    </div>
  {/if}
</div>
