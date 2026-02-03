<script lang="ts">
  export let title = 'Nothing here yet';
  export let message = 'Once you add data, it will show up here.';
  export let className = '';
</script>

<div class={`rounded-xl border border-dashed border-slate-200 bg-white/60 p-6 text-center ${className}`}>
  <div class="space-y-2">
    <p class="page-title">{title}</p>
    <p class="text-sm text-slate-500">{message}</p>
  </div>
  {#if $$slots.action}
    <div class="mt-4 flex justify-center">
      <slot name="action" />
    </div>
  {/if}
</div>
