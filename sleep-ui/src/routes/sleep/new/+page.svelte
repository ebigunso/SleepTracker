<script lang="ts">
  import SleepForm from '$lib/components/SleepForm.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';

  // Prefill date from query (?date=YYYY-MM-DD) if present
  const url = get(page).url;
  const initialDate = url.searchParams.get('date');
  function goHome() {
    goto('/');
  }

  function onSaved() {
    goHome();
  }

  function onCancel() {
    goHome();
  }
</script>

<section class="space-y-4">
  <div>
    <h2 class="page-title">New sleep entry</h2>
    <p class="text-muted text-sm">Log bedtime, wake time, and how you feel.</p>
  </div>
  <div class="surface-card rounded-xl p-4">
    <SleepForm mode="create" {initialDate} showCancel on:saved={onSaved} on:cancel={onCancel} />
  </div>
</section>
