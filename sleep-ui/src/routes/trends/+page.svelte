<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { apiGet } from '$lib/api';
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { theme, type Theme } from '$lib/stores/theme';
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

  const metrics: {
    key: MetricKey;
    label: string;
    helper: string;
    chartColorVar: string;
    borderVar: string;
  }[] = [
    {
      key: 'duration',
      label: 'Duration',
      helper: 'Total sleep time',
      chartColorVar: '--color-chart-primary',
      borderVar: '--color-primary'
    },
    {
      key: 'quality',
      label: 'Quality',
      helper: 'Sleep quality scores',
      chartColorVar: '--color-chart-secondary',
      borderVar: '--color-secondary'
    },
    {
      key: 'bedtime',
      label: 'Bedtime',
      helper: 'Start time (24h)',
      chartColorVar: '--color-chart-accent',
      borderVar: '--color-accent'
    },
    {
      key: 'waketime',
      label: 'Wake time',
      helper: 'End time (24h)',
      chartColorVar: '--color-chart-danger',
      borderVar: '--color-danger'
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
      if (view === 'chart') {
        await tick();
        if (canvasEl) {
          void renderChart(sortedBars, metric, $theme);
        }
      }
    }
  }

  function cssVar(name: string): string {
    return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  }

  async function renderChart(data: SleepBarRecord[], selectedMetric: MetricKey, themeValue: Theme) {
    if (!canvasEl) return;
    if (!data.length) {
      chart?.destroy();
      chart = null;
      return;
    }

    const labels = data.map((b) => b.date);
    const values = data.map((b) => metricValue(b, selectedMetric));
    const meta = metrics.find((m) => m.key === selectedMetric) ?? metrics[0];
    const textColor = cssVar('--color-text');
    const mutedColor = cssVar('--color-text-muted');
    const borderColor = cssVar('--color-border');
    const surfaceColor = cssVar('--color-surface');
    const gridColor = borderColor;

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
            backgroundColor: cssVar(meta.chartColorVar),
            borderColor: cssVar(meta.borderVar)
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
              text: yAxisTitle,
              color: mutedColor
            },
            beginAtZero: !isTimeMetric,
            min: yAxisMin,
            max: yAxisMax,
            ticks: {
              color: mutedColor,
              callback: (value: string | number) => {
                if (selectedMetric === 'duration') return formatDurationHMM(Number(value));
                if (selectedMetric === 'quality') return `${value}`;
                return formatMinutesAsTime(Number(value));
              }
            },
            grid: {
              color: gridColor
            }
          },
          x: {
            title: { display: true, text: 'Date', color: mutedColor },
            ticks: {
              color: mutedColor,
              maxTicksLimit: 6,
              callback: (_value: string | number, index: number) => {
                const label = labels[index] ?? '';
                return label ? formatDateShort(label) : '';
              }
            },
            grid: {
              color: gridColor
            }
          }
        },
        plugins: {
          legend: { display: false },
          tooltip: {
            backgroundColor: surfaceColor,
            borderColor,
            borderWidth: 1,
            titleColor: textColor,
            bodyColor: mutedColor,
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
    if (canvasEl) {
      void renderChart(sortedBars, metric, $theme);
    }
  } else {
    chart?.destroy();
    chart = null;
  }
</script>

<section class="space-y-6">
  <div class="surface-card rounded-2xl px-5 py-4">
    <div class="flex flex-wrap items-start justify-between gap-4">
      <div>
        <h2 class="text-default text-2xl font-semibold">Trends</h2>
        <p class="text-muted text-sm">{rangeLabel} · {from} – {to}</p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        {#each views as option}
          <button
            type="button"
            class={`focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm font-semibold shadow-sm transition ${
              option.key !== view ? 'btn-outline' : 'btn-primary'
            }`}
            on:click={() => (view = option.key)}
          >
            {option.label}
          </button>
        {/each}
      </div>
    </div>
    <div class="mt-4 flex flex-wrap items-end gap-3">
      <div class="toggle-group flex flex-wrap items-center gap-1 rounded-full p-1">
        {#each presets as preset}
          <button
            type="button"
            class={`toggle-pill rounded-full px-3 py-1 text-xs font-semibold transition ${
              currentRangeDays === preset.days ? 'toggle-pill--active' : ''
            }`}
            on:click={() => applyPreset(preset.days)}
          >
            {preset.label}
          </button>
        {/each}
      </div>
      <form class="flex flex-wrap items-end gap-2" on:submit={refresh}>
        <div>
          <label for="from-date" class="text-muted block text-xs">From</label>
          <input
            id="from-date"
            type="date"
            bind:value={from}
            class="input-base text-sm"
          />
        </div>
        <div>
          <label for="to-date" class="text-muted block text-xs">To</label>
          <input
            id="to-date"
            type="date"
            bind:value={to}
            class="input-base text-sm"
          />
        </div>
        <button
          type="submit"
          class="btn-primary focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm shadow-sm disabled:cursor-not-allowed disabled:opacity-60"
          disabled={loading}
        >
          {#if loading}Loading...{:else}Apply{/if}
        </button>
      </form>
    </div>
  </div>

  {#if errorMsg}
    <div class="state-card state-card--error" role="alert">
      {errorMsg}
    </div>
  {/if}
  <div class="surface-card grid gap-3 rounded-2xl px-5 py-4 text-sm sm:grid-cols-2 lg:grid-cols-4">
    <div>
      <div class="text-muted text-xs font-semibold uppercase tracking-wide">Nights</div>
      <div class="text-default mt-1 text-lg font-semibold">{totalNights}</div>
    </div>
    <div>
      <div class="text-muted text-xs font-semibold uppercase tracking-wide">Avg duration</div>
      <div class="text-default mt-1 text-lg font-semibold">{formatDurationHMM(avgDuration)}</div>
    </div>
    <div>
      <div class="text-muted text-xs font-semibold uppercase tracking-wide">Avg quality</div>
      <div class="text-default mt-1 text-lg font-semibold">{formatQuality(avgQuality)}</div>
    </div>
    <div>
      <div class="text-muted text-xs font-semibold uppercase tracking-wide">Range</div>
      <div class="text-default mt-1 text-lg font-semibold">{from} – {to}</div>
    </div>
  </div>

  {#if view === 'chart'}
    <div class="surface-card space-y-4 rounded-2xl p-4">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="toggle-group flex flex-wrap items-center gap-2 rounded-full p-1">
          {#each metrics as option}
            <button
              type="button"
              class={`toggle-pill rounded-full px-3 py-1 text-xs font-semibold transition ${
                metric === option.key ? 'toggle-pill--active' : ''
              }`}
              on:click={() => (metric = option.key)}
            >
              {option.label}
            </button>
          {/each}
        </div>
        <div class="text-muted text-xs">
          {metrics.find((m) => m.key === metric)?.helper}
        </div>
      </div>
      <div class="h-72">
        {#if loading}
          <div class="text-muted flex h-full items-center justify-center text-sm">Loading chart…</div>
        {:else if sortedBars.length === 0}
          <div class="text-muted flex h-full items-center justify-center text-sm">No data in range.</div>
        {:else}
          <canvas bind:this={canvasEl}></canvas>
        {/if}
      </div>
    </div>
  {:else}
    <div class="surface-card space-y-4 rounded-2xl p-4">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <h3 class="text-default text-sm font-semibold">Schedule view</h3>
        <span class="text-muted text-xs">24h timeline</span>
      </div>
      {#if loading}
        <div class="text-muted text-sm">Loading schedule…</div>
      {:else if sortedBars.length === 0}
        <div class="text-muted text-sm">No data in range.</div>
      {:else}
        <div class="space-y-3">
          {#each sortedBars as bar (bar.date)}
            <div class="surface-muted grid gap-3 rounded-xl px-3 py-3 sm:grid-cols-[160px,1fr]">
              <div>
                <div class="text-default text-sm font-semibold">{bar.date}</div>
                <div class="text-muted text-xs">{formatTimeHHMM(bar.bed_time)} – {formatTimeHHMM(bar.wake_time)}</div>
              </div>
              <div class="space-y-2">
                <SleepBar bed_time={bar.bed_time} wake_time={bar.wake_time} />
                <div class="text-muted flex flex-wrap items-center gap-3 text-xs">
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
