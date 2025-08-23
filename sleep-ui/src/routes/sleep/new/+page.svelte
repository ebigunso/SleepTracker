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
</script>

<section class="space-y-4">
  <h2 class="text-xl font-semibold text-gray-900">New sleep entry</h2>
  <SleepForm mode="create" {initialDate} on:saved={onSaved} />
</section>
