<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import { toasts, pushToast, dismissToast } from '$lib/stores/toast';
  import { readCsrfToken, setUserTimezoneIfSupported } from '$lib/api';
  import '../app.css';
  const AUTH_PREFIX = '/api';

  export let data: { session?: boolean; pathname?: string };
  let isAuthRoute = false;
  $: isAuthRoute = data?.pathname === '/login';

  onMount(async () => {
    if (!browser || !data.session) return;
    try {
      const tz = Intl.DateTimeFormat().resolvedOptions().timeZone;
      if (!tz) return;
      const key = 'sleeptracker.userTimezone';
      const last = localStorage.getItem(key);
      if (last === tz) return;
      const ok = await setUserTimezoneIfSupported(tz);
      if (ok) {
        localStorage.setItem(key, tz);
      }
    } catch {
      // ignore
    }
  });


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
<div class="min-h-screen bg-slate-50 text-slate-900">
  {#if isAuthRoute}
    <main class="mx-auto flex min-h-screen items-center justify-center px-4 py-10">
      <slot />
    </main>
  {:else}
    <header class="border-b border-slate-200/70 bg-white/90 backdrop-blur">
      <div class="mx-auto flex max-w-5xl flex-col gap-4 px-4 py-4 xs:flex-row xs:items-center xs:justify-between">
        <div class="flex items-center gap-3">
          <span class="flex h-9 w-9 items-center justify-center rounded-full bg-indigo-600 text-sm font-semibold text-white">ST</span>
          <div>
            <h1 class="text-lg font-semibold text-slate-900">SleepTracker</h1>
            <p class="text-xs text-slate-500">Calm rhythms, better rest</p>
          </div>
        </div>
        <nav class="flex flex-wrap items-center gap-2 text-sm">
          {#if data.session}
            <a href="/" class="focus-ring touch-target inline-flex items-center rounded-full px-3 py-1.5 text-slate-600 hover:text-indigo-600 hover:bg-indigo-50">Home</a>
            <a href="/trends" class="focus-ring touch-target inline-flex items-center rounded-full px-3 py-1.5 text-slate-600 hover:text-indigo-600 hover:bg-indigo-50">Trends</a>
            <button
              class="focus-ring touch-target inline-flex items-center rounded-full px-3 py-1.5 text-rose-600 hover:bg-rose-50"
              on:click|preventDefault={logout}
            >
              Logout
            </button>
          {:else}
            <a
              href="/login"
              class="focus-ring touch-target inline-flex items-center rounded-full bg-indigo-600 px-4 py-1.5 font-semibold text-white shadow-sm hover:bg-indigo-700"
            >
              Login
            </a>
          {/if}
        </nav>
      </div>
    </header>

    <main class="mx-auto max-w-5xl px-4 py-8">
      <slot />
    </main>
  {/if}
</div>

<!-- Toasts -->
<div class="fixed inset-x-0 bottom-4 z-50 flex flex-col items-center gap-2" role="status" aria-live="polite">
  {#each $toasts as t (t.id)}
    <div class="flex w-[95%] max-w-md items-start gap-3 rounded-xl border border-slate-200 bg-white/95 px-4 py-3 shadow-lg">
      <span class="text-sm {t.type === 'error' ? 'text-rose-600' : t.type === 'success' ? 'text-emerald-600' : 'text-slate-700'}">
        {t.message}
      </span>
      <button class="focus-ring touch-target ml-auto text-xs text-slate-500 hover:text-slate-800" on:click={() => dismissToast(t.id)}>Dismiss</button>
    </div>
  {/each}
</div>
