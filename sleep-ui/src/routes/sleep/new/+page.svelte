<script lang="ts">
  import SleepForm from '$lib/components/SleepForm.svelte';
  import SleepEntryShell from '$lib/components/SleepEntryShell.svelte';
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

<SleepEntryShell
  title="New sleep entry"
  description="Log bedtime, wake time, and how you feel."
  headingTestId="sleep-new-heading"
  formAnchorTestId="sleep-new-form-anchor"
>
    <SleepForm
      mode="create"
      {initialDate}
      personalization={data.personalization}
      showCancel
      on:saved={onSaved}
      on:cancel={onCancel}
    />
</SleepEntryShell>
