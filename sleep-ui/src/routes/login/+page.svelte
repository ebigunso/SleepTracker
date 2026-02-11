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
      <h2 class="auth-title text-2xl font-semibold">Welcome back</h2>
      <p class="auth-helper text-sm">Sign in to track your sleep and trends.</p>
    </div>
    {#if errorMsg}
      <div class="auth-error-box mb-4 rounded-lg border px-3 py-2 text-sm" role="alert">
        {errorMsg}
      </div>
    {/if}
    <form on:submit={submit} class="space-y-5" novalidate>
      <div class="space-y-2">
        <label for="email" class="auth-label block text-sm font-medium">Email</label>
        <input
          id="email"
          class={`auth-input ${emailError ? 'auth-input--error' : ''}`}
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
          <p class="auth-error-text text-xs">{emailError}</p>
        {/if}
      </div>
      <div class="space-y-2">
        <label for="password" class="auth-label block text-sm font-medium">Password</label>
        <div class="relative">
          <input
            id="password"
            class={`auth-input pr-14 ${passwordError ? 'auth-input--error' : ''}`}
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
            class="auth-show-toggle absolute inset-y-0 right-2 my-auto rounded-full px-3 text-xs font-semibold"
            aria-pressed={showPassword}
            aria-label={showPassword ? 'Hide password' : 'Show password'}
            on:click={() => (showPassword = !showPassword)}
          >
            {showPassword ? 'Hide' : 'Show'}
          </button>
        </div>
        {#if passwordError}
          <p class="auth-error-text text-xs">{passwordError}</p>
        {/if}
      </div>
      <button
        type="submit"
        class="auth-primary-button inline-flex w-full items-center justify-center rounded-full px-3 py-2.5 text-sm font-semibold shadow-sm disabled:opacity-60"
        disabled={loading}
      >
        {#if loading}Signing in...{:else}Sign in{/if}
      </button>
    </form>
  </div>
</div>
