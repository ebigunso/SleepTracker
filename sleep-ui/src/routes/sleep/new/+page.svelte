<script lang="ts">
  import SleepForm from '$lib/components/SleepForm.svelte';
  import { goto } from '$app/navigation';
  import type { PersonalizationResponse } from '$lib/api';

  export let data: {
    initialDate: string | null;
    personalization: PersonalizationResponse | null;
  };

  const initialDate = data.initialDate;
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
    <SleepForm
      mode="create"
      {initialDate}
      personalization={data.personalization}
      showCancel
      on:saved={onSaved}
      on:cancel={onCancel}
    />
  </div>
</section>
