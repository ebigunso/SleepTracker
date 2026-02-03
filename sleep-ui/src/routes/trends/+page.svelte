<script lang="ts">
  import { onMount } from 'svelte';
  import { apiGet } from '$lib/api';
  import SleepBar from '$lib/components/SleepBar.svelte';
  import {
    computeDurationMin,
    formatDurationHMM,
    formatMinutesAsTime,
    formatTimeHHMM,
    toMinutes
  } from '$lib/utils/sleep';
  let ChartJS: any = null;

  type SleepBarRecord = {
    date: string; // ISO date (YYYY-MM-DD)
    bed_time: string; // HH:MM:SS
    wake_time: string; // HH:MM:SS
    quality?: number | null;
    duration_min?: number | null;
  };

  type MetricKey = 'duration' | 'quality' | 'bedtime' | 'waketime';
  type ViewMode = 'chart' | 'schedule';

  const presets = [
    { label: '7d', days: 7 },
    { label: '14d', days: 14 },
    { label: '30d', days: 30 }
  ];

  const metrics: { key: MetricKey; label: string; helper: string; color: string; border: string }[] = [
    {
      key: 'duration',
      label: 'Duration',
      helper: 'Total sleep time',
      color: 'rgba(99, 102, 241, 0.45)',
      border: 'rgb(99, 102, 241)'
    },
    {
      key: 'quality',
      label: 'Quality',
      helper: 'Sleep quality scores',
      color: 'rgba(14, 165, 233, 0.35)',
      border: 'rgb(14, 165, 233)'
    },
    {
      key: 'bedtime',
      label: 'Bedtime',
      helper: 'Start time (24h)',
      color: 'rgba(34, 197, 94, 0.35)',
      border: 'rgb(34, 197, 94)'
    },
    {
      key: 'waketime',
      label: 'Wake time',
      helper: 'End time (24h)',
      color: 'rgba(248, 113, 113, 0.35)',
      border: 'rgb(248, 113, 113)'
    }
  ];

  const views: { key: ViewMode; label: string }[] = [
    { key: 'chart', label: 'Chart' },
    { key: 'schedule', label: 'Schedule' }
  ];

  let canvasEl: HTMLCanvasElement | null = null;
  let chart: any | null = null;

  let from = '';
  let to = '';
  let bars: SleepBarRecord[] = [];
  let loading = false;
  let errorMsg: string | null = null;
  let metric: MetricKey = 'duration';
  let view: ViewMode = 'chart';

  function iso(d: Date) {
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, '0');
    const dd = String(d.getDate()).padStart(2, '0');
    return `${yyyy}-${mm}-${dd}`;
  }

  function parseLocalDate(date: string): Date {
    return new Date(`${date}T00:00:00`);
  }

  function formatDateShort(date: string): string {
    return parseLocalDate(date).toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
  }

  function formatDateLong(date: string): string {
    return parseLocalDate(date).toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
  }

  function rangeDays(start: string, end: string): number | null {
    if (!start || !end) return null;
    const s = parseLocalDate(start);
    const e = parseLocalDate(end);
    const diff = Math.floor((e.getTime() - s.getTime()) / (1000 * 60 * 60 * 24)) + 1;
    return diff > 0 ? diff : null;
  }

  function setDefaultRange(days = 14) {
    const end = new Date();
    const start = new Date();
    start.setDate(end.getDate() - (days - 1));
    from = iso(start);
    to = iso(end);
  }

  function metricValue(bar: SleepBarRecord, key: MetricKey): number | null {
    if (key === 'duration') {
      return bar.duration_min ?? computeDurationMin(bar.bed_time, bar.wake_time);
    }
    if (key === 'quality') return bar.quality ?? null;
    if (key === 'bedtime') return toMinutes(bar.bed_time);
    return toMinutes(bar.wake_time);
  }

  function formatQuality(value: number | null | undefined): string {
    if (value == null || !Number.isFinite(value)) return '—';
    const rounded = Math.round(value * 10) / 10;
    return `${rounded}`;
  }

  function formatMetricValue(key: MetricKey, value: number | null | undefined): string {
    if (key === 'duration') return formatDurationHMM(value);
    if (key === 'quality') return formatQuality(value);
    return formatMinutesAsTime(value);
  }

  function average(values: number[]): number | null {
    if (!values.length) return null;
    const total = values.reduce((sum, v) => sum + v, 0);
    return total / values.length;
  }

  async function loadBars() {
    if (!from || !to) setDefaultRange();
    loading = true;
    errorMsg = null;
    try {
      const q = new URLSearchParams({ from, to }).toString();
      const data = await apiGet<SleepBarRecord[]>(`/api/trends/sleep-bars?${q}`);
      bars = data;
    } catch (e) {
      console.error(e);
      errorMsg = 'Failed to load trends';
    } finally {
      loading = false;
    }
  }

  async function renderChart(data: SleepBarRecord[], selectedMetric: MetricKey) {
    if (!canvasEl) return;
    if (!data.length) {
      chart?.destroy();
      chart = null;
      return;
    }

    const labels = data.map((b) => b.date);
    const values = data.map((b) => metricValue(b, selectedMetric));
    const meta = metrics.find((m) => m.key === selectedMetric) ?? metrics[0];

    if (!ChartJS) {
      // typed dynamic import for chart.js to satisfy TS under bundler mode
      const mod = (await import('chart.js')) as typeof import('chart.js');
      ChartJS = mod.Chart;
      ChartJS.register(...mod.registerables);
    }

    const isTimeMetric = selectedMetric === 'bedtime' || selectedMetric === 'waketime';
    const yAxisTitle = selectedMetric === 'duration'
      ? 'Duration (h:mm)'
      : selectedMetric === 'quality'
        ? 'Quality'
        : 'Time (24h)';
    const yAxisMin = isTimeMetric ? 0 : undefined;
    let yAxisMax: number | undefined;
    if (isTimeMetric) {
      yAxisMax = 24 * 60;
    } else if (selectedMetric === 'quality') {
      yAxisMax = 5;
    }

    chart?.destroy();
    chart = new ChartJS(canvasEl, {
      type: 'bar',
      data: {
        labels,
        datasets: [
          {
            label: meta.label,
            data: values,
            borderWidth: 1,
            backgroundColor: meta.color,
            borderColor: meta.border
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: { mode: 'index', intersect: false },
        scales: {
          y: {
            title: {
              display: true,
              text: yAxisTitle
            },
            beginAtZero: !isTimeMetric,
            min: yAxisMin,
            max: yAxisMax,
            ticks: {
              callback: (value: string | number) => {
                if (selectedMetric === 'duration') return formatDurationHMM(Number(value));
                if (selectedMetric === 'quality') return `${value}`;
                return formatMinutesAsTime(Number(value));
              }
            }
          },
          x: {
            title: { display: true, text: 'Date' },
            ticks: {
              maxTicksLimit: 6,
              callback: (_value: string | number, index: number) => {
                const label = labels[index] ?? '';
                return label ? formatDateShort(label) : '';
              }
            }
          }
        },
        plugins: {
          legend: { display: false },
          tooltip: {
            callbacks: {
              title(ctx: any) {
                if (!ctx?.length) return '';
                const i = ctx[0].dataIndex;
                const label = labels[i] ?? '';
                return label ? formatDateLong(label) : '';
              },
              label(ctx: any) {
                if (!ctx) return '';
                const value = ctx.parsed?.y as number | null | undefined;
                return `${meta.label}: ${formatMetricValue(selectedMetric, value)}`;
              },
              afterBody(ctx: any) {
                if (!ctx?.length) return '';
                const i = ctx[0].dataIndex;
                const entry = data[i];
                if (!entry) return '';
                const duration = entry.duration_min ?? computeDurationMin(entry.bed_time, entry.wake_time);
                const rows = [
                  `Bed: ${formatTimeHHMM(entry.bed_time)}`,
                  `Wake: ${formatTimeHHMM(entry.wake_time)}`,
                  `Duration: ${formatDurationHMM(duration)}`
                ];
                if (entry.quality != null) rows.push(`Quality: ${formatQuality(entry.quality)}`);
                return rows;
              }
            }
          }
        }
      }
    });
  }

  onMount(() => {
    setDefaultRange();
    loadBars();
    return () => {
      chart?.destroy();
    };
  });

  function refresh(e: Event) {
    e.preventDefault();
    loadBars();
  }

  function applyPreset(days: number) {
    setDefaultRange(days);
    loadBars();
  }

  $: currentRangeDays = rangeDays(from, to);
  $: rangeLabel = currentRangeDays ? `Last ${currentRangeDays} days` : 'Custom range';
  $: sortedBars = [...bars].sort((a, b) => a.date.localeCompare(b.date));
  $: durations = sortedBars.map((b) => b.duration_min ?? computeDurationMin(b.bed_time, b.wake_time));
  $: avgDuration = average(durations);
  $: avgQuality = average(sortedBars.map((b) => b.quality).filter((v): v is number => v != null));
  $: totalNights = sortedBars.length;
  $: if (view === 'chart') {
    void renderChart(sortedBars, metric);
  } else {
    chart?.destroy();
    chart = null;
  }
</script>

<section class="space-y-6">
  <div class="rounded-2xl border border-slate-200 bg-white px-5 py-4 shadow-sm">
    <div class="flex flex-wrap items-start justify-between gap-4">
      <div>
        <h2 class="text-2xl font-semibold text-slate-900">Trends</h2>
        <p class="text-sm text-slate-500">{rangeLabel} · {from} – {to}</p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        {#each views as option}
          <button
            type="button"
            class={`inline-flex items-center rounded-full px-4 py-2 text-sm font-semibold shadow-sm transition ${
              option.key !== view
                ? 'border border-slate-200 bg-white text-slate-700 hover:bg-slate-50'
                : 'bg-indigo-600 text-white hover:bg-indigo-700'
            }`}
            on:click={() => (view = option.key)}
          >
            {option.label}
          </button>
        {/each}
      </div>
    </div>
    <div class="mt-4 flex flex-wrap items-end gap-3">
      <div class="flex flex-wrap items-center gap-1 rounded-full bg-slate-100 p-1">
        {#each presets as preset}
          <button
            type="button"
            class={`rounded-full px-3 py-1 text-xs font-semibold transition ${
              currentRangeDays === preset.days
                ? 'bg-white text-slate-700 shadow-sm'
                : 'text-slate-500 hover:text-slate-700'
            }`}
            on:click={() => applyPreset(preset.days)}
          >
            {preset.label}
          </button>
        {/each}
      </div>
      <form class="flex flex-wrap items-end gap-2" on:submit={refresh}>
        <div>
          <label for="from-date" class="block text-xs text-slate-500">From</label>
          <input
            id="from-date"
            type="date"
            bind:value={from}
            class="rounded-md border-slate-200 text-sm focus:border-indigo-500 focus:ring-indigo-500"
          />
        </div>
        <div>
          <label for="to-date" class="block text-xs text-slate-500">To</label>
          <input
            id="to-date"
            type="date"
            bind:value={to}
            class="rounded-md border-slate-200 text-sm focus:border-indigo-500 focus:ring-indigo-500"
          />
        </div>
        <button
          type="submit"
          class="inline-flex items-center rounded-full bg-indigo-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-700"
          disabled={loading}
        >
          {#if loading}Loading...{:else}Apply{/if}
        </button>
      </form>
    </div>
  </div>

  {#if errorMsg}
    <div class="rounded-lg border border-rose-200 bg-rose-50 px-3 py-2 text-sm text-rose-700">
      {errorMsg}
    </div>
  {/if}

  <div class="grid gap-3 rounded-2xl border border-slate-200 bg-white px-5 py-4 text-sm shadow-sm sm:grid-cols-2 lg:grid-cols-4">
    <div>
      <div class="text-xs font-semibold uppercase tracking-wide text-slate-400">Nights</div>
      <div class="mt-1 text-lg font-semibold text-slate-900">{totalNights}</div>
    </div>
    <div>
      <div class="text-xs font-semibold uppercase tracking-wide text-slate-400">Avg duration</div>
      <div class="mt-1 text-lg font-semibold text-slate-900">{formatDurationHMM(avgDuration)}</div>
    </div>
    <div>
      <div class="text-xs font-semibold uppercase tracking-wide text-slate-400">Avg quality</div>
      <div class="mt-1 text-lg font-semibold text-slate-900">{formatQuality(avgQuality)}</div>
    </div>
    <div>
      <div class="text-xs font-semibold uppercase tracking-wide text-slate-400">Range</div>
      <div class="mt-1 text-lg font-semibold text-slate-900">{from} – {to}</div>
    </div>
  </div>

  {#if view === 'chart'}
    <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm space-y-4">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="flex flex-wrap items-center gap-2 rounded-full bg-slate-100 p-1">
          {#each metrics as option}
            <button
              type="button"
              class={`rounded-full px-3 py-1 text-xs font-semibold transition ${
                metric === option.key
                  ? 'bg-white text-slate-700 shadow-sm'
                  : 'text-slate-500 hover:text-slate-700'
              }`}
              on:click={() => (metric = option.key)}
            >
              {option.label}
            </button>
          {/each}
        </div>
        <div class="text-xs text-slate-500">
          {metrics.find((m) => m.key === metric)?.helper}
        </div>
      </div>
      <div class="h-72">
        {#if loading}
          <div class="flex h-full items-center justify-center text-sm text-slate-500">Loading chart…</div>
        {:else if sortedBars.length === 0}
          <div class="flex h-full items-center justify-center text-sm text-slate-500">No data in range.</div>
        {:else}
          <canvas bind:this={canvasEl}></canvas>
        {/if}
      </div>
    </div>
  {:else}
    <div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm space-y-4">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <h3 class="text-sm font-semibold text-slate-700">Schedule view</h3>
        <span class="text-xs text-slate-500">24h timeline</span>
      </div>
      {#if loading}
        <div class="text-sm text-slate-500">Loading schedule…</div>
      {:else if sortedBars.length === 0}
        <div class="text-sm text-slate-500">No data in range.</div>
      {:else}
        <div class="space-y-3">
          {#each sortedBars as bar (bar.date)}
            <div class="grid gap-3 rounded-xl border border-slate-100 bg-slate-50/50 px-3 py-3 sm:grid-cols-[160px,1fr]">
              <div>
                <div class="text-sm font-semibold text-slate-900">{bar.date}</div>
                <div class="text-xs text-slate-500">{formatTimeHHMM(bar.bed_time)} – {formatTimeHHMM(bar.wake_time)}</div>
              </div>
              <div class="space-y-2">
                <SleepBar bed_time={bar.bed_time} wake_time={bar.wake_time} />
                <div class="flex flex-wrap items-center gap-3 text-xs text-slate-500">
                  <span>Duration {formatDurationHMM(bar.duration_min ?? computeDurationMin(bar.bed_time, bar.wake_time))}</span>
                  {#if bar.quality != null}
                    <span>Quality {formatQuality(bar.quality)}</span>
                  {/if}
                </div>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</section>
