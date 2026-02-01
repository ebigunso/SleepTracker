<script lang="ts">
  /**
   * SleepBar renders a 24h track with filled segments.
   * It supports cross-midnight by splitting into two segments when bed_time > wake_time.
   *
   * Props:
   * - bed_time, wake_time: "HH:mm:ss" (local times)
   * - segments: optional precomputed segments in minutes since 00:00
   */
  import { computeSegments, type Segment } from '$lib/utils/sleep';

  export let bed_time: string;
  export let wake_time: string;
  export let segments: Segment[] | undefined = undefined;

  const total = 24 * 60;
  let renderSegments: Segment[] = [];

  $: renderSegments = segments !== undefined ? segments : computeSegments(bed_time, wake_time);

  function pct(n: number): string {
    return `${(n / total) * 100}%`;
  }
</script>

<div class="bar relative h-3 w-full rounded bg-gray-200 overflow-hidden" aria-label="sleep-bar">
  {#each renderSegments as seg}
    <div
      class="absolute h-full rounded bg-indigo-500"
      style={`left:${pct(seg.start)}; width:${pct(seg.end - seg.start)};`}
    ></div>
  {/each}
</div>
