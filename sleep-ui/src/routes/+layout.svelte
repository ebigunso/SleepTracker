<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import { toasts, pushToast, dismissToast } from '$lib/stores/toast';
  import { readCsrfToken, setUserTimezoneIfSupported } from '$lib/api';
  import ProfileMenu from '$lib/components/ProfileMenu.svelte';
  import '../app.css';
  const AUTH_PREFIX = '/api';

  export let data: { session?: boolean; pathname?: string };
  let isAuthRoute = false;
  $: isAuthRoute = data?.pathname === '/login';

  type NavItem = {
    href: string;
    label: string;
    match: (path: string) => boolean;
  };

  let pathname = '';

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

  const navItems: NavItem[] = [
    {
      href: '/',
      label: 'Home',
      match: (path) => path === '/' || path.startsWith('/sleep') || path.startsWith('/day')
    },
    {
      href: '/trends',
      label: 'Trends',
      match: (path) => path.startsWith('/trends')
    }
  ];

  $: pathname = data?.pathname ?? '';

  function isActive(item: NavItem) {
    return item.match(pathname);
  }

  function navLinkClass(item: NavItem) {
    return isActive(item)
      ? 'rounded-full bg-indigo-50 px-3 py-1.5 text-sm font-semibold text-indigo-700 ring-1 ring-indigo-200'
      : 'rounded-full px-3 py-1.5 text-sm font-semibold text-slate-600 hover:text-indigo-600 hover:bg-indigo-50';
  }

  function bottomNavClass(item: NavItem) {
    return isActive(item)
      ? 'flex flex-1 flex-col items-center justify-center gap-1 rounded-xl bg-indigo-50 text-indigo-700 ring-1 ring-indigo-200 min-h-[44px] py-2 text-xs font-semibold'
      : 'flex flex-1 flex-col items-center justify-center gap-1 rounded-xl text-slate-600 hover:text-indigo-600 hover:bg-slate-100 min-h-[44px] py-2 text-xs font-semibold';
  }


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
    <header class="border-b border-slate-200/70 bg-white/90 backdrop-blur" aria-label="Site header">
      <div class="app-container grid grid-cols-[auto,1fr,auto] items-center gap-4 py-4">
        <div class="flex items-center gap-3">
          <span class="flex h-9 w-9 items-center justify-center rounded-full bg-indigo-600 text-sm font-semibold text-white">ST</span>
          <div>
            <h1 class="text-lg font-semibold text-slate-900">SleepTracker</h1>
            <p class="text-xs text-slate-500">Calm rhythms, better rest</p>
          </div>
        </div>
        <nav class="hidden items-center justify-center gap-2 text-sm md:flex" aria-label="Primary navigation">
          {#if data.session}
            {#each navItems as item (item.href)}
              <a
                href={item.href}
                class={navLinkClass(item)}
                aria-current={isActive(item) ? 'page' : undefined}
              >
                {item.label}
              </a>
            {/each}
          {/if}
        </nav>
        <div class="flex items-center justify-end gap-2">
          {#if data.session}
            <ProfileMenu on:logout={logout} />
          {:else}
            <a
              href="/login"
              class="focus-ring touch-target rounded-full bg-indigo-600 px-4 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-700"
            >
              Login
            </a>
          {/if}
        </div>
      </div>
    </header>

    <main class={`app-container py-8 ${data.session ? 'pb-24 md:pb-8' : ''}`}>
      <slot />
    </main>

    {#if data.session}
      <nav class="mobile-bottom-nav fixed inset-x-0 bottom-0 z-40 border-t border-slate-200 bg-white/95 backdrop-blur md:hidden" aria-label="Bottom navigation">
        <div class="app-container">
          <div class="flex items-center gap-2 py-2">
            {#each navItems as item (item.href)}
              <a
                href={item.href}
                class={bottomNavClass(item)}
                aria-current={isActive(item) ? 'page' : undefined}
              >
                <span>{item.label}</span>
              </a>
            {/each}
          </div>
        </div>
      </nav>
    {/if}
  {/if}
</div>

<!-- Toasts -->
<div class="fixed inset-x-0 bottom-20 z-50 flex flex-col items-center gap-2 md:bottom-4" role="status" aria-live="polite">
  {#each $toasts as t (t.id)}
    <div class="flex w-[95%] max-w-md items-start gap-3 rounded-xl border border-slate-200 bg-white/95 px-4 py-3 shadow-lg">
      <span class="text-sm {t.type === 'error' ? 'text-rose-600' : t.type === 'success' ? 'text-emerald-600' : 'text-slate-700'}">
        {t.message}
      </span>
      <button class="focus-ring touch-target ml-auto text-xs text-slate-500 hover:text-slate-800" on:click={() => dismissToast(t.id)}>Dismiss</button>
    </div>
  {/each}
</div>
