<script lang="ts">
  /**
   * SleepBar renders a 24h track with a filled segment for the sleep interval.
   * It supports cross-midnight by splitting into two segments when bed_time > wake_time.
   *
   * Props:
   * - bed_time, wake_time: "HH:mm:ss" (local times)
   */
  export let bed_time: string;
  export let wake_time: string;

  function toMinutes(t: string): number {
    // Expect "HH:mm:ss" or "HH:mm"
    const [hh, mm, ss] = t.split(':').map((v) => parseInt(v, 10));
    const m = (hh || 0) * 60 + (mm || 0);
    return Number.isFinite(m) ? m : 0;
  }

  const total = 24 * 60;
  const bedMin = toMinutes(bed_time);
  const wakeMin = toMinutes(wake_time);

  type Segment = { start: number; end: number }; // minutes since 00:00
  let segments: Segment[] = [];

  if (Number.isFinite(bedMin) && Number.isFinite(wakeMin)) {
    if (bedMin <= wakeMin) {
      segments = [{ start: bedMin, end: wakeMin }];
    } else {
      // Wrap across midnight: [bed -> 1440) and [0 -> wake]
      segments = [
        { start: 0, end: wakeMin },
        { start: bedMin, end: total }
      ];
    }
  }

  function pct(n: number): string {
    return `${(n / total) * 100}%`;
  }
</script>

<div class="bar relative h-3 w-full rounded bg-gray-200 overflow-hidden" aria-label="sleep-bar">
  {#each segments as seg}
    <div
      class="absolute h-full rounded bg-indigo-500"
      style={`left:${pct(seg.start)}; width:${pct(seg.end - seg.start)};`}
    ></div>
  {/each}
</div>
