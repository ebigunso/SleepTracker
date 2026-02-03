<script lang="ts">
  import { goto } from '$app/navigation';
  import { pushToast } from '$lib/stores/toast';

  const AUTH_PREFIX = '/api';

  let email = '';
  let password = '';
  let loading = false;
  let showPassword = false;
  let errorMsg: string | null = null;
  let emailError: string | null = null;
  let passwordError: string | null = null;

  async function submit(e: Event) {
    e.preventDefault();
    errorMsg = null;
    emailError = null;
    passwordError = null;

    if (!email) {
      emailError = 'Email is required';
    }
    if (!password) {
      passwordError = 'Password is required';
    }
    if (emailError || passwordError) return;
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

<div class="w-full max-w-md">
  <div class="card px-6 py-7 sm:px-8 sm:py-8">
    <div class="mb-6 text-center">
      <div class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-indigo-600 text-base font-semibold text-white">
        ST
      </div>
      <h2 class="text-2xl font-semibold text-slate-900">Welcome back</h2>
      <p class="text-sm text-slate-500">Sign in to track your sleep and trends.</p>
    </div>
    {#if errorMsg}
      <div class="mb-4 rounded-lg border border-rose-200 bg-rose-50 px-3 py-2 text-sm text-rose-700" role="alert">
        {errorMsg}
      </div>
    {/if}
    <form on:submit={submit} class="space-y-5" novalidate>
      <div class="space-y-2">
        <label for="email" class="block text-sm font-medium text-slate-700">Email</label>
        <input
          id="email"
          class={`block w-full rounded-md shadow-sm focus:ring-2 ${emailError ? 'border-rose-300 focus:border-rose-500 focus:ring-rose-200' : 'border-slate-200 focus:border-indigo-500 focus:ring-indigo-200'}`}
          type="email"
          name="email"
          bind:value={email}
          required
          autocomplete="username"
          aria-invalid={emailError ? 'true' : 'false'}
          on:input={() => {
            emailError = null;
            errorMsg = null;
          }}
        />
        {#if emailError}
          <p class="text-xs text-rose-600">{emailError}</p>
        {/if}
      </div>
      <div class="space-y-2">
        <label for="password" class="block text-sm font-medium text-slate-700">Password</label>
        <div class="relative">
          <input
            id="password"
            class={`block w-full rounded-md pr-12 shadow-sm focus:ring-2 ${passwordError ? 'border-rose-300 focus:border-rose-500 focus:ring-rose-200' : 'border-slate-200 focus:border-indigo-500 focus:ring-indigo-200'}`}
            type={showPassword ? 'text' : 'password'}
            name="password"
            bind:value={password}
            required
            autocomplete="current-password"
            aria-invalid={passwordError ? 'true' : 'false'}
            on:input={() => {
              passwordError = null;
              errorMsg = null;
            }}
          />
          <button
            type="button"
            class="absolute inset-y-0 right-2 my-auto rounded-full px-3 py-1 text-xs font-semibold text-slate-600 hover:bg-slate-100"
            aria-pressed={showPassword}
            aria-label={showPassword ? 'Hide password' : 'Show password'}
            on:click={() => (showPassword = !showPassword)}
          >
            {showPassword ? 'Hide' : 'Show'}
          </button>
        </div>
        {#if passwordError}
          <p class="text-xs text-rose-600">{passwordError}</p>
        {/if}
      </div>
      <button
        type="submit"
        class="w-full inline-flex items-center justify-center rounded-full bg-indigo-600 px-3 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-700 disabled:opacity-60"
        disabled={loading}
      >
        {#if loading}Signing in...{:else}Sign in{/if}
      </button>
    </form>
  </div>
</div>
