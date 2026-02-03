<script lang="ts">
  export let totalSessions: number;
  export let totalSleep: string;
  export let avgDuration: string;
  export let avgQuality: string | number;
  export let avgLatency: string;
</script>

<div class="grid gap-3 rounded-2xl border border-slate-200 bg-white p-4 shadow-sm sm:grid-cols-2 lg:grid-cols-4">
  <div>
    <p class="text-xs font-medium text-slate-500">Sessions</p>
    <p class="text-lg font-semibold text-slate-900">{totalSessions}</p>
  </div>
  <div>
    <p class="text-xs font-medium text-slate-500">Total sleep</p>
    <p class="text-lg font-semibold text-slate-900">{totalSleep}</p>
  </div>
  <div>
    <p class="text-xs font-medium text-slate-500">Avg duration</p>
    <p class="text-lg font-semibold text-slate-900">{avgDuration}</p>
  </div>
  <div>
    <p class="text-xs font-medium text-slate-500">Avg quality</p>
    <p class="text-lg font-semibold text-slate-900">{avgQuality}</p>
    <p class="text-xs text-slate-500">Avg latency {avgLatency}</p>
  </div>
</div>
