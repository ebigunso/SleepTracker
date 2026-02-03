<script lang="ts">
  export let message = 'Loading…';
  export let className = '';
</script>

<div class={`flex items-center gap-3 rounded-lg border border-slate-200 bg-white px-4 py-3 text-slate-600 ${className}`}>
  <span class="h-4 w-4 animate-spin rounded-full border-2 border-slate-300 border-t-indigo-500"></span>
  <span class="meta-text">{message}</span>
</div>
