<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import { toasts, pushToast, dismissToast } from '$lib/stores/toast';
  import { readCsrfToken, setUserTimezoneIfSupported } from '$lib/api';
  import ProfileMenu from '$lib/components/ProfileMenu.svelte';
  import { theme, toggleTheme } from '$lib/stores/theme';
  import '../app.css';
  const AUTH_PREFIX = '/api';

  export let data: { session?: boolean; theme?: 'light' | 'dark' };
  let isAuthRoute = false;

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

  $: pathname = $page.url.pathname ?? '';
  $: isAuthRoute = pathname === '/login';

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
<div class="app-shell">
  {#if isAuthRoute}
    <main class="mx-auto flex min-h-screen items-center justify-center px-4 py-10">
      <slot />
    </main>
  {:else}
    <header class="app-header backdrop-blur" aria-label="Site header">
      <div class="app-container grid grid-cols-[auto,1fr,auto] items-center gap-4 py-4">
        <a href="/" class="focus-ring inline-flex w-fit items-center gap-3 justify-self-start self-start rounded-xl">
          <span class="brand-badge flex h-9 w-9 items-center justify-center rounded-full text-sm font-semibold">ST</span>
          <div>
            <h1 class="text-lg font-semibold">SleepTracker</h1>
            <p class="brand-subtitle text-xs">Calm rhythms, better rest</p>
          </div>
        </a>
        <nav class="hidden items-center justify-center gap-2 text-sm md:flex" aria-label="Primary navigation">
          {#if data.session}
            {#each navItems as item (item.href)}
              <a
                href={item.href}
                class="nav-link focus-ring"
                class:nav-link--active={item.match(pathname)}
                aria-current={item.match(pathname) ? 'page' : undefined}
              >
                {item.label}
              </a>
            {/each}
          {/if}
        </nav>
        <div class="flex items-center justify-end gap-2">
          {#if data.session}
            <ProfileMenu theme={$theme} on:toggleTheme={toggleTheme} on:logout={logout} />
          {:else}
            <a
              href="/login"
              class="btn-primary focus-ring touch-target rounded-full px-4 py-1.5 text-sm shadow-sm"
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
      <nav class="mobile-bottom-nav bottom-nav fixed inset-x-0 bottom-0 z-40 backdrop-blur md:hidden" aria-label="Bottom navigation">
        <div class="app-container">
          <div class="flex items-center gap-2 py-2">
            {#each navItems as item (item.href)}
              <a
                href={item.href}
                class="bottom-nav-link focus-ring"
                class:bottom-nav-link--active={item.match(pathname)}
                aria-current={item.match(pathname) ? 'page' : undefined}
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
    <div class="toast flex w-[95%] max-w-md items-start gap-3 rounded-xl px-4 py-3 shadow-lg">
      <span
        class={`toast-message text-sm ${
          t.type === 'error'
            ? 'toast-message--error'
            : t.type === 'success'
              ? 'toast-message--success'
              : ''
        }`}
      >
        {t.message}
      </span>
      <button class="toast-dismiss focus-ring touch-target ml-auto text-xs" on:click={() => dismissToast(t.id)}>Dismiss</button>
    </div>
  {/each}
</div>
