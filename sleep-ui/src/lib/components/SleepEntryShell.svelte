<script lang="ts">
  export let title: string;
  export let description: string;
  export let headingTestId: string;
  export let formAnchorTestId: string;
</script>

<section class="space-y-4">
  {#if $$slots.actions}
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="page-title" data-testid={headingTestId}>{title}</h2>
        <p class="text-muted text-sm">{description}</p>
      </div>
      <slot name="actions" />
    </div>
  {:else}
    <div>
      <h2 class="page-title" data-testid={headingTestId}>{title}</h2>
      <p class="text-muted text-sm">{description}</p>
    </div>
  {/if}

  <div class="surface-card rounded-xl p-4" data-testid={formAnchorTestId}>
    <slot />
  </div>
</section>