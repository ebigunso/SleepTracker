<script lang="ts">
  import { goto } from '$app/navigation';
  import { pushToast } from '$lib/stores/toast';

  const AUTH_PREFIX = '/api';

  let email = '';
  let password = '';
  let loading = false;
  let errorMsg: string | null = null;

  async function submit(e: Event) {
    e.preventDefault();
    errorMsg = null;

    if (!email || !password) {
      errorMsg = 'Email and password are required';
      return;
    }
    loading = true;
    try {
      const res = await fetch(`${AUTH_PREFIX}/login`, {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: new URLSearchParams({ email, password })
      });
      if (res.ok) {
        pushToast({ type: 'success', message: 'Logged in' });
        goto('/');
      } else if (res.status === 401) {
        errorMsg = 'Invalid credentials';
      } else {
        errorMsg = `Login failed: ${res.status}`;
      }
    } catch (err) {
      errorMsg = 'Network error';
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-slate-50 px-4">
  <div class="w-full max-w-sm rounded-2xl bg-white p-6 shadow-lg ring-1 ring-slate-200/70">
    <div class="mb-5">
      <h2 class="text-2xl font-semibold text-slate-900">Welcome back</h2>
      <p class="text-sm text-slate-500">Sign in to track your sleep and trends.</p>
    </div>
    {#if errorMsg}
      <div class="state-card state-card--error mb-3" role="alert" aria-live="polite">
        {errorMsg}
      </div>
    {/if}
    <form on:submit={submit} class="space-y-4">
      <div>
        <label for="email" class="block text-sm font-medium text-slate-700">Email</label>
        <input
          id="email"
          class="mt-1 block w-full rounded-md border-slate-200 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          type="email"
          name="email"
          bind:value={email}
          required
          autocomplete="username"
        />
      </div>
      <div>
        <label for="password" class="block text-sm font-medium text-slate-700">Password</label>
        <input
          id="password"
          class="mt-1 block w-full rounded-md border-slate-200 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          type="password"
          name="password"
          bind:value={password}
          required
          autocomplete="current-password"
        />
      </div>
      <button
        type="submit"
        class="focus-ring touch-target w-full inline-flex items-center justify-center rounded-full bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-700 disabled:opacity-60"
        disabled={loading}
      >
        {#if loading}Signing in...{:else}Sign in{/if}
      </button>
    </form>
  </div>
</div>
