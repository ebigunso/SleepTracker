<script lang="ts">
  import { goto } from '$app/navigation';
  import { toasts, pushToast, dismissToast } from '$lib/stores/toast';
  import { readCsrfToken } from '$lib/api';
  import '../app.css';
  const AUTH_PREFIX = '/api';

  export let data: { session?: boolean; pathname?: string };


  async function logout() {
    try {
      const csrf = readCsrfToken();
      const res = await fetch(`${AUTH_PREFIX}/logout`, {
        method: 'POST',
        credentials: 'include',
        headers: csrf ? { 'X-CSRF-Token': csrf } : {}
      });
      if (res.status === 204) {
        pushToast({ type: 'success', message: 'Logged out' });
        goto('/login');
      } else {
        pushToast({ type: 'error', message: `Logout failed: ${res.status}` });
      }
    } catch (e) {
      pushToast({ type: 'error', message: 'Network error during logout' });
    }
  }
</script>

<!-- App shell -->
<div class="min-h-screen bg-gray-50 text-gray-900">
  <header class="border-b bg-white">
    <div class="mx-auto max-w-3xl px-4 py-3 flex items-center justify-between">
      <h1 class="text-lg font-semibold">SleepTracker</h1>
      <nav class="flex items-center gap-3">
        {#if data.session}
          <a href="/" class="text-sm text-gray-700 hover:text-gray-900">Home</a>
          <a href="/trends" class="text-sm text-gray-700 hover:text-gray-900">Trends</a>
          <button class="text-sm text-red-600 hover:text-red-700" on:click|preventDefault={logout}>Logout</button>
        {:else}
          <a href="/login" class="text-sm text-blue-600 hover:text-blue-700">Login</a>
        {/if}
      </nav>
    </div>
  </header>

  <main class="mx-auto max-w-3xl px-4 py-6">
    <slot />
  </main>
</div>

<!-- Toasts -->
<div class="fixed inset-x-0 bottom-4 z-50 flex flex-col items-center gap-2">
  {#each $toasts as t (t.id)}
    <div class="rounded-md px-4 py-3 shadow bg-white border w-[95%] max-w-md flex items-start gap-3">
      <span class="text-sm {t.type === 'error' ? 'text-red-700' : t.type === 'success' ? 'text-green-700' : 'text-gray-700'}">
        {t.message}
      </span>
      <button class="ml-auto text-xs text-gray-500 hover:text-gray-800" on:click={() => dismissToast(t.id)}>Dismiss</button>
    </div>
  {/each}
</div>
