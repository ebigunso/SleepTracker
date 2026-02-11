<script lang="ts">
  import SleepForm from '$lib/components/SleepForm.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';

  // Prefill date from query (?date=YYYY-MM-DD) if present
  const url = get(page).url;
  const initialDate = url.searchParams.get('date');
  function onSaved() {
    goto('/');
  }
  function onCancel() {
    goto('/');
  }
</script>

<section class="space-y-4">
  <div>
    <h2 class="text-2xl font-semibold text-slate-900">New sleep entry</h2>
    <p class="text-sm text-slate-500">Log bedtime, wake time, and how you feel.</p>
  </div>
  <div class="rounded-xl bg-white p-4 shadow-sm ring-1 ring-slate-200/70">
    <SleepForm mode="create" {initialDate} showCancel on:saved={onSaved} on:cancel={onCancel} />
  </div>
</section>
