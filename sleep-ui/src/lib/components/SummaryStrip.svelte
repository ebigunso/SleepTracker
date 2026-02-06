<script lang="ts">
  export let totalSessions: number;
  export let totalSleep: string;
  export let avgDuration: string;
  export let avgQuality: string | number;
  export let avgLatency: string;
</script>

<div class="surface-card grid gap-3 rounded-2xl p-4 sm:grid-cols-2 lg:grid-cols-4">
  <div>
    <p class="text-muted text-xs font-medium">Sessions</p>
    <p class="text-default text-lg font-semibold">{totalSessions}</p>
  </div>
  <div>
    <p class="text-muted text-xs font-medium">Total sleep</p>
    <p class="text-default text-lg font-semibold">{totalSleep}</p>
  </div>
  <div>
    <p class="text-muted text-xs font-medium">Avg duration</p>
    <p class="text-default text-lg font-semibold">{avgDuration}</p>
  </div>
  <div>
    <p class="text-muted text-xs font-medium">Avg quality</p>
    <p class="text-default text-lg font-semibold">{avgQuality}</p>
    <p class="text-muted text-xs">Avg latency {avgLatency}</p>
  </div>
</div>
