<script lang="ts">
  import { onMount, tick } from 'svelte';
  import {
    apiGet,
    getPersonalization,
    getPersonalDurationReferenceBand,
    getTrendsSummary,
    getScheduleVariabilityDeltaMin,
    selectPrioritizedTrendsExplanation,
    type ActionRecommendation,
    type PersonalizationResponse,
    type TrendsSummaryResponse
  } from '$lib/api';
  import SleepBar from '$lib/components/SleepBar.svelte';
  import { theme, type Theme } from '$lib/stores/theme';
  import {
    computeDurationMin,
    formatDurationHMM,
    formatMinutesAsTime,
    formatTimeHHMM,
    toMinutes,
    unwrapClockMinutes,
    wrapClockMinutes
  } from '$lib/utils/sleep';
  import {
    addDays,
    averageMetricValues,
    dateToIsoLocal,
    isoWeekBucket,
    parseLocalDate,
    priorRange,
    rangeDays
  } from '$lib/utils/trends';
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
  type EvidenceFocusKey = 'schedule_shift' | 'variability';

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
  let priorBars: SleepBarRecord[] = [];
  let weeklySummary: TrendsSummaryResponse | null = null;
  let personalization: PersonalizationResponse | null = null;
  let loading = false;
  let errorMsg: string | null = null;
  let metric: MetricKey = 'duration';
  let view: ViewMode = 'chart';
  let showPriorComparator = false;
  let showWeeklyLens = false;
  let activeEvidenceFocusKey: EvidenceFocusKey | null = null;

  const scheduleShiftInsightKey = 'social_jetlag_schedule_shift_insight';
  const regularityInsightKey = 'regularity_insight_priority';
  const qualityExplanationKey = 'quality_aligned_factor_explanation';
  const metricQuestions: Record<MetricKey, string> = {
    duration: 'Is my sleep duration changing?',
    quality: 'Is my sleep quality changing?',
    bedtime: 'Is my bedtime shifting?',
    waketime: 'Is my wake time shifting?'
  };

  function formatDateShort(date: string): string {
    return parseLocalDate(date).toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
  }

  function formatDateLong(date: string): string {
    return parseLocalDate(date).toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
  }

  function dayOfWeek(date: string): number {
    return parseLocalDate(date).getDay();
  }

  function isWeekendDate(date: string): boolean {
    const day = dayOfWeek(date);
    return day === 0 || day === 6;
  }

  function setDefaultRange(days = 14) {
    const end = new Date();
    const start = new Date();
    start.setDate(end.getDate() - (days - 1));
    from = dateToIsoLocal(start);
    to = dateToIsoLocal(end);
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

  function averageMetric(barsInput: SleepBarRecord[], key: MetricKey): number | null {
    const values = barsInput
      .map((bar) => metricValue(bar, key))
      .filter((value): value is number => value != null && Number.isFinite(value));
    return averageMetricValues(values, key, wrappedTimeAnchorMinutes);
  }

  function recommendationByKey(actionKey: string): ActionRecommendation | null {
    const recommendations = personalization?.recommendations;
    if (!recommendations || !Array.isArray(recommendations)) return null;
    const match = recommendations.find((item) => {
      const recommendation = item as ActionRecommendation & { actionKey?: string };
      return recommendation.action_key === actionKey || recommendation.actionKey === actionKey;
    });
    return match ?? null;
  }

  function isRecommended(rec: ActionRecommendation | null): boolean {
    if (!rec) return false;
    const normalized = String((rec as ActionRecommendation & { status?: string }).status ?? '')
      .trim()
      .toLowerCase();
    return normalized === 'recommended';
  }

  function formatSignedMinutes(value: number | null | undefined): string {
    if (value == null || !Number.isFinite(value)) return '—';
    const rounded = Math.round(value);
    const sign = rounded > 0 ? '+' : '';
    return `${sign}${rounded} min`;
  }

  function formatSignedQuality(value: number | null | undefined): string {
    if (value == null || !Number.isFinite(value)) return '—';
    const rounded = Math.round(value * 10) / 10;
    const sign = rounded > 0 ? '+' : '';
    return `${sign}${rounded}`;
  }

  function formatPercent(value: number | null | undefined): string {
    if (value == null || !Number.isFinite(value)) return '—';
    return `${Math.round(value)}%`;
  }

  function formatDeltaSummary(selectedMetric: MetricKey, delta: number | null): string {
    if (delta == null || !Number.isFinite(delta)) return 'No prior-period comparison yet.';
    if (selectedMetric === 'duration') {
      return `${formatSignedMinutes(delta)} vs prior period`;
    }
    if (selectedMetric === 'quality') {
      return `${formatSignedQuality(delta)} vs prior period`;
    }
    const rounded = Math.round(delta);
    if (rounded === 0) return 'No timing shift vs prior period';
    const direction = rounded > 0 ? 'later' : 'earlier';
    return `${Math.abs(rounded)} min ${direction} vs prior period`;
  }

  async function loadBars() {
    if (!from || !to) setDefaultRange();
    loading = true;
    errorMsg = null;
    try {
      const q = new URLSearchParams({ from, to }).toString();
      const days = rangeDays(from, to);
      const prior = priorRange(from, to);
      const personalizationQuery = {
        to,
        ...(typeof days === 'number' ? { window_days: days } : {})
      };
      const priorPromise = prior
        ? apiGet<SleepBarRecord[]>(
            `/api/trends/sleep-bars?${new URLSearchParams({ from: prior.from, to: prior.to }).toString()}`
          ).catch(() => [])
        : Promise.resolve([] as SleepBarRecord[]);
      const weeklySummaryPromise = showWeeklyLens
        ? getTrendsSummary({ from, to, bucket: 'week' }).catch(() => null)
        : Promise.resolve(null as TrendsSummaryResponse | null);
      const [data, priorData, personalizationData, weeklySummaryData] = await Promise.all([
        apiGet<SleepBarRecord[]>(`/api/trends/sleep-bars?${q}`),
        priorPromise,
        getPersonalization(personalizationQuery).catch(() => null),
        weeklySummaryPromise
      ]);
      bars = data;
      priorBars = priorData;
      personalization = personalizationData;
      weeklySummary = weeklySummaryData;
    } catch (e) {
      console.error(e);
      errorMsg = 'Failed to load trends';
      priorBars = [];
      personalization = null;
      weeklySummary = null;
    } finally {
      loading = false;
      if (view === 'chart') {
        await tick();
        if (canvasEl) {
          void renderChart(
            sortedBars,
            sortedPriorBars,
            weeklySummary,
            metric,
            $theme,
            showPriorComparator,
            showWeeklyLens,
            currentRangeDays
          );
        }
      }
    }
  }

  function cssVar(name: string): string {
    return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  }

  type MetricRenderContext = {
    labels: string[];
    data: SleepBarRecord[];
    priorData: SleepBarRecord[];
    priorValues: Array<number | null>;
    priorDateLabels: Array<string | null>;
    showPriorComparator: boolean;
    showWeeklyLens: boolean;
    weeklyValues: Array<number | null>;
    weeklySupported: boolean;
    selectedMetric: MetricKey;
    meta: (typeof metrics)[number];
    colors: {
      textColor: string;
      mutedColor: string;
      borderColor: string;
      surfaceColor: string;
      gridColor: string;
    };
    evidenceFocus: {
      key: EvidenceFocusKey;
      title: string;
      description: string;
      dates: string[];
    } | null;
  };

  type MetricChartConfig = {
    chartType: 'bar' | 'line';
    yAxisTitle: string;
    beginAtZero: boolean;
    yAxisMin?: number;
    yAxisMax?: number;
    yAxisTickStepSize?: number;
    pointOnly?: boolean;
    transformValue?: (value: number | null) => number | null;
    inverseTransformValue?: (value: number | null) => number | null;
    yAxisTick: (value: string | number) => string;
  };

  const wrappedTimeAnchorMinutes = 12 * 60;
  const wrappedTimeAxisMin = wrappedTimeAnchorMinutes;
  const wrappedTimeAxisMax = wrappedTimeAnchorMinutes + 24 * 60;

  function buildPriorComparatorSeries(
    currentData: SleepBarRecord[],
    priorData: SleepBarRecord[],
    selectedMetric: MetricKey,
    periodDays: number | null
  ): { values: Array<number | null>; dates: Array<string | null> } {
    if (!periodDays || !priorData.length) {
      return {
        values: currentData.map(() => null),
        dates: currentData.map(() => null)
      };
    }
    const priorByDate = new Map(priorData.map((bar) => [bar.date, bar]));
    return {
      values: currentData.map((bar) => {
        const priorDate = addDays(bar.date, -periodDays);
        const priorEntry = priorByDate.get(priorDate);
        if (!priorEntry) return null;
        return metricValue(priorEntry, selectedMetric);
      }),
      dates: currentData.map((bar) => {
        const priorDate = addDays(bar.date, -periodDays);
        return priorByDate.has(priorDate) ? priorDate : null;
      })
    };
  }

  function buildWeeklyLensSeries(
    currentData: SleepBarRecord[],
    summary: TrendsSummaryResponse | null,
    selectedMetric: MetricKey
  ): Array<number | null> {
    if (!summary) {
      return currentData.map(() => null);
    }
    if (selectedMetric === 'duration') {
      const durationByWeek = new Map(summary.duration_by_bucket.map((item) => [item.bucket, item.avg_min]));
      return currentData.map((bar) => durationByWeek.get(isoWeekBucket(bar.date)) ?? null);
    }
    if (selectedMetric === 'quality') {
      const qualityByWeek = new Map(summary.quality_by_bucket.map((item) => [item.bucket, item.avg]));
      return currentData.map((bar) => qualityByWeek.get(isoWeekBucket(bar.date)) ?? null);
    }
    return currentData.map(() => null);
  }

  const metricChartConfigs: Record<MetricKey, MetricChartConfig> = {
    duration: {
      chartType: 'line',
      yAxisTitle: 'Duration (h:mm)',
      beginAtZero: true,
      yAxisTick: (value) => formatDurationHMM(Number(value))
    },
    quality: {
      chartType: 'line',
      yAxisTitle: 'Quality',
      beginAtZero: false,
      yAxisMin: 1,
      yAxisMax: 5,
      yAxisTickStepSize: 1,
      pointOnly: true,
      yAxisTick: (value) => `${value}`
    },
    bedtime: {
      chartType: 'line',
      yAxisTitle: 'Time (24h)',
      beginAtZero: false,
      yAxisMin: wrappedTimeAxisMin,
      yAxisMax: wrappedTimeAxisMax,
      transformValue: (value) => wrapClockMinutes(value, wrappedTimeAnchorMinutes),
      inverseTransformValue: (value) => unwrapClockMinutes(value),
      yAxisTick: (value) => formatMinutesAsTime(unwrapClockMinutes(Number(value)))
    },
    waketime: {
      chartType: 'line',
      yAxisTitle: 'Time (24h)',
      beginAtZero: false,
      yAxisMin: wrappedTimeAxisMin,
      yAxisMax: wrappedTimeAxisMax,
      transformValue: (value) => wrapClockMinutes(value, wrappedTimeAnchorMinutes),
      inverseTransformValue: (value) => unwrapClockMinutes(value),
      yAxisTick: (value) => formatMinutesAsTime(unwrapClockMinutes(Number(value)))
    }
  };

  function buildChartConfig(ctx: MetricRenderContext) {
    const metricConfig = metricChartConfigs[ctx.selectedMetric];
    const hasPriorComparator =
      ctx.showPriorComparator &&
      ctx.priorValues.some((value) => value != null && Number.isFinite(value));
    const hasWeeklyLens =
      ctx.showWeeklyLens &&
      ctx.weeklySupported &&
      ctx.weeklyValues.some((value) => value != null && Number.isFinite(value));

    const datasets: Array<Record<string, unknown>> = [
      {
        label: 'Current period',
        tooltipSeriesKind: 'current',
        data: ctx.data.map((b) => {
          const value = metricValue(b, ctx.selectedMetric);
          return metricConfig.transformValue ? metricConfig.transformValue(value) : value;
        }),
        spanGaps: false,
        borderWidth: 1,
        backgroundColor: cssVar(ctx.meta.chartColorVar),
        borderColor: cssVar(ctx.meta.borderVar),
        fill: false,
        tension: metricConfig.chartType === 'line' ? 0.2 : 0,
        showLine: metricConfig.chartType === 'line' && !metricConfig.pointOnly,
        pointRadius: metricConfig.chartType === 'line' ? 3 : 0,
        pointHoverRadius: metricConfig.chartType === 'line' ? 5 : 0
      }
    ];

    if (hasPriorComparator) {
      datasets.push({
        label: 'Prior period',
        tooltipSeriesKind: 'prior',
        data: ctx.priorValues.map((value) =>
          metricConfig.transformValue ? metricConfig.transformValue(value) : value
        ),
        spanGaps: false,
        borderWidth: 1,
        borderDash: [6, 4],
        backgroundColor: ctx.colors.mutedColor,
        borderColor: ctx.colors.mutedColor,
        fill: false,
        tension: metricConfig.chartType === 'line' ? 0.2 : 0,
        showLine: metricConfig.chartType === 'line' && !metricConfig.pointOnly,
        pointRadius: metricConfig.chartType === 'line' ? 2 : 0,
        pointHoverRadius: metricConfig.chartType === 'line' ? 4 : 0
      });
    }

    if (hasWeeklyLens) {
      datasets.push({
        label: 'Weekly lens',
        tooltipSeriesKind: 'weekly',
        data: ctx.weeklyValues.map((value) =>
          metricConfig.transformValue ? metricConfig.transformValue(value) : value
        ),
        spanGaps: false,
        borderWidth: 2,
        borderDash: [3, 3],
        backgroundColor: ctx.colors.textColor,
        borderColor: ctx.colors.textColor,
        fill: false,
        tension: 0.2,
        showLine: true,
        pointRadius: 0,
        pointHoverRadius: 3
      });
    }

    const hasEvidenceFocus =
      !!ctx.evidenceFocus &&
      ctx.evidenceFocus.dates.length > 0 &&
      ctx.evidenceFocus.dates.some((date) => ctx.data.some((entry) => entry.date === date));

    if (hasEvidenceFocus && ctx.evidenceFocus) {
      const evidenceDates = new Set(ctx.evidenceFocus.dates);
      datasets.push({
        label: ctx.evidenceFocus.title,
        tooltipSeriesKind: 'evidence',
        tooltipSeriesTitle: ctx.evidenceFocus.title,
        data: ctx.data.map((bar) => {
          if (!evidenceDates.has(bar.date)) return null;
          const value = metricValue(bar, ctx.selectedMetric);
          return metricConfig.transformValue ? metricConfig.transformValue(value) : value;
        }),
        spanGaps: false,
        borderWidth: 0,
        backgroundColor: cssVar(ctx.meta.borderVar),
        borderColor: cssVar(ctx.meta.borderVar),
        fill: false,
        showLine: false,
        pointRadius: 5,
        pointHoverRadius: 7
      });
    }

    return {
      type: metricConfig.chartType,
      data: {
        labels: ctx.labels,
        datasets
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: { mode: 'index', intersect: false },
        scales: {
          y: {
            title: {
              display: true,
              text: metricConfig.yAxisTitle,
              color: ctx.colors.mutedColor
            },
            beginAtZero: metricConfig.beginAtZero,
            min: metricConfig.yAxisMin,
            max: metricConfig.yAxisMax,
            ticks: {
              color: ctx.colors.mutedColor,
              stepSize: metricConfig.yAxisTickStepSize,
              callback: metricConfig.yAxisTick
            },
            grid: {
              color: ctx.colors.gridColor
            }
          },
          x: {
            title: { display: true, text: 'Date', color: ctx.colors.mutedColor },
            ticks: {
              color: ctx.colors.mutedColor,
              maxTicksLimit: 6,
              callback: (_value: string | number, index: number) => {
                const label = ctx.labels[index] ?? '';
                return label ? formatDateShort(label) : '';
              }
            },
            grid: {
              color: ctx.colors.gridColor
            }
          }
        },
        plugins: {
          legend: { display: false },
          tooltip: {
            backgroundColor: ctx.colors.surfaceColor,
            borderColor: ctx.colors.borderColor,
            borderWidth: 1,
            titleColor: ctx.colors.textColor,
            bodyColor: ctx.colors.mutedColor,
            callbacks: {
              title(tooltipContext: any) {
                if (!tooltipContext?.length) return '';
                const i = tooltipContext[0].dataIndex;
                const label = ctx.labels[i] ?? '';
                return label ? formatDateLong(label) : '';
              },
              label(tooltipContext: any) {
                if (!tooltipContext) return '';
                const rawValue = (tooltipContext.parsed?.y ?? null) as number | null;
                const value = metricConfig.inverseTransformValue
                  ? metricConfig.inverseTransformValue(rawValue)
                  : rawValue;
                const dataset = (tooltipContext.dataset ?? {}) as {
                  tooltipSeriesKind?: 'current' | 'prior' | 'weekly' | 'evidence';
                  tooltipSeriesTitle?: string;
                };
                const seriesKind = dataset.tooltipSeriesKind ?? 'current';
                const priorDate = ctx.priorDateLabels[tooltipContext.dataIndex] ?? null;
                const seriesLabel =
                  seriesKind === 'prior'
                    ? priorDate
                      ? `Prior (${formatDateShort(priorDate)})`
                      : 'Prior period'
                    : seriesKind === 'weekly'
                      ? 'Weekly lens'
                      : seriesKind === 'evidence'
                        ? dataset.tooltipSeriesTitle
                          ? `Evidence highlight (${dataset.tooltipSeriesTitle})`
                          : 'Evidence highlight'
                        : 'Current period';
                return `${seriesLabel}: ${formatMetricValue(ctx.selectedMetric, value)}`;
              },
              afterBody(tooltipContext: any) {
                if (!tooltipContext?.length) return '';
                const i = tooltipContext[0].dataIndex;
                const rows: string[] = [];
                const currentEntry = ctx.data[i];
                const priorByDate = new Map(ctx.priorData.map((bar) => [bar.date, bar]));
                const activeSeriesKinds = new Set(
                  tooltipContext.map((item: any) => {
                    const itemDataset = (item.dataset ?? {}) as {
                      tooltipSeriesKind?: 'current' | 'prior' | 'weekly' | 'evidence';
                    };
                    return itemDataset.tooltipSeriesKind ?? 'current';
                  })
                );

                if (activeSeriesKinds.has('current') && currentEntry) {
                  const currentDuration =
                    currentEntry.duration_min ?? computeDurationMin(currentEntry.bed_time, currentEntry.wake_time);
                  rows.push(
                    `Current bed/wake: ${formatTimeHHMM(currentEntry.bed_time)} → ${formatTimeHHMM(currentEntry.wake_time)}`
                  );
                  rows.push(`Current duration: ${formatDurationHMM(currentDuration)}`);
                  if (currentEntry.quality != null) {
                    rows.push(`Current quality: ${formatQuality(currentEntry.quality)}`);
                  }
                }

                if (activeSeriesKinds.has('prior')) {
                  const priorDate = ctx.priorDateLabels[i] ?? null;
                  const priorEntry = priorDate ? priorByDate.get(priorDate) : undefined;
                  if (priorEntry) {
                    const priorDuration =
                      priorEntry.duration_min ?? computeDurationMin(priorEntry.bed_time, priorEntry.wake_time);
                    rows.push(
                      `Prior bed/wake (${formatDateShort(priorEntry.date)}): ${formatTimeHHMM(priorEntry.bed_time)} → ${formatTimeHHMM(priorEntry.wake_time)}`
                    );
                    rows.push(`Prior duration: ${formatDurationHMM(priorDuration)}`);
                    if (priorEntry.quality != null) {
                      rows.push(`Prior quality: ${formatQuality(priorEntry.quality)}`);
                    }
                  } else if (priorDate) {
                    rows.push(`Prior context (${formatDateShort(priorDate)}) unavailable.`);
                  }
                }

                if (activeSeriesKinds.has('weekly')) {
                  rows.push('Weekly lens uses weekly averages; single-session bed/wake context is not shown.');
                }

                if (activeSeriesKinds.has('evidence')) {
                  rows.push('Evidence highlight marks recommendation-supporting points in the current period.');
                  if (ctx.evidenceFocus?.description) {
                    rows.push(ctx.evidenceFocus.description);
                  }
                }

                if (!rows.length) return '';
                return rows;
              }
            }
          }
        }
      }
    };
  }

  async function renderChart(
    data: SleepBarRecord[],
    priorData: SleepBarRecord[],
    summary: TrendsSummaryResponse | null,
    selectedMetric: MetricKey,
    themeValue: Theme,
    showComparator: boolean,
    showWeekly: boolean,
    periodDays: number | null
  ) {
    if (!data.length) {
      chart?.destroy();
      chart = null;
      return;
    }
    if (!canvasEl) return;

    const labels = data.map((b) => b.date);
    const meta = metrics.find((m) => m.key === selectedMetric) ?? metrics[0];
    const textColor = cssVar('--color-text');
    const mutedColor = cssVar('--color-text-muted');
    const borderColor = cssVar('--color-border');
    const surfaceColor = cssVar('--color-surface');
    const gridColor = borderColor;
    const priorSeries = buildPriorComparatorSeries(data, priorData, selectedMetric, periodDays);
    const weeklySupported = selectedMetric === 'duration' || selectedMetric === 'quality';
    const weeklyValues = buildWeeklyLensSeries(data, summary, selectedMetric);

    if (!ChartJS) {
      // typed dynamic import for chart.js to satisfy TS under bundler mode
      const mod = (await import('chart.js')) as typeof import('chart.js');
      ChartJS = mod.Chart;
      ChartJS.register(...mod.registerables);
    }

    chart?.destroy();
    const config = buildChartConfig({
      labels,
      data,
      priorData,
      priorValues: priorSeries.values,
      priorDateLabels: priorSeries.dates,
      showPriorComparator: showComparator,
      showWeeklyLens: showWeekly,
      weeklyValues,
      weeklySupported,
      selectedMetric,
      meta,
      colors: { textColor, mutedColor, borderColor, surfaceColor, gridColor },
      evidenceFocus: selectedMetric === activeEvidenceMetric ? activeEvidence : null
    });
    chart = new ChartJS(canvasEl, config);
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
  $: sortedPriorBars = [...priorBars].sort((a, b) => a.date.localeCompare(b.date));
  $: scheduleShiftInsight = recommendationByKey(scheduleShiftInsightKey);
  $: regularityInsight = recommendationByKey(regularityInsightKey);
  $: qualityExplanation = recommendationByKey(qualityExplanationKey);
  $: hasScheduleShiftMetric = personalization?.metrics.social_jetlag.current_delta_min != null;
  $: hasVariabilityMetric = personalization?.metrics.schedule_variability.current_variability_min != null;
  $: showScheduleShiftInsight = isRecommended(scheduleShiftInsight) || hasScheduleShiftMetric;
  $: showVariabilityInsight = isRecommended(regularityInsight) || hasVariabilityMetric;
  $: hasInsightCards = showScheduleShiftInsight || showVariabilityInsight;
  $: prioritizedExplanation = selectPrioritizedTrendsExplanation(
    regularityInsight,
    qualityExplanation,
    metrics.find((m) => m.key === metric)?.helper
  );
  $: durations = sortedBars.map((b) => b.duration_min ?? computeDurationMin(b.bed_time, b.wake_time));
  $: avgDuration = average(durations);
  $: avgQuality = average(sortedBars.map((b) => b.quality).filter((v): v is number => v != null));
  $: totalNights = sortedBars.length;
  $: selectedMetricQuestion = metricQuestions[metric];
  $: currentMetricAvg = averageMetric(sortedBars, metric);
  $: priorMetricAvg = averageMetric(sortedPriorBars, metric);
  $: metricDelta =
    currentMetricAvg != null && priorMetricAvg != null ? currentMetricAvg - priorMetricAvg : null;
  $: metricDeltaSummary = formatDeltaSummary(metric, metricDelta);
  $: durationReferenceBand = getPersonalDurationReferenceBand(
    personalization?.metrics.duration_baseline
  );
  $: durationOutOfRangePct =
    personalization?.metrics.duration_baseline.recent_out_of_range_incidence_pct ?? null;
  $: qualityMidpointDelta = avgQuality != null ? avgQuality - 3 : null;
  $: scheduleVariabilityMetric = personalization?.metrics.schedule_variability;
  $: scheduleVariabilityDeltaMin = getScheduleVariabilityDeltaMin(scheduleVariabilityMetric);
  $: scheduleShiftEvidenceDates = sortedBars
    .filter((bar) => isWeekendDate(bar.date))
    .map((bar) => bar.date);
  $: variabilityEvidenceDates = sortedBars.slice(Math.max(0, sortedBars.length - 7)).map((bar) => bar.date);
  $: activeEvidence =
    activeEvidenceFocusKey === 'schedule_shift'
      ? {
          key: 'schedule_shift' as const,
          title: 'Weekend timing evidence',
          description:
            scheduleShiftEvidenceDates.length > 0
              ? `${scheduleShiftEvidenceDates.length} weekend points highlighted in bedtime chart.`
              : 'No weekend points in this range.',
          dates: scheduleShiftEvidenceDates
        }
      : activeEvidenceFocusKey === 'variability'
        ? {
            key: 'variability' as const,
            title: 'Recent timing variability evidence',
            description:
              variabilityEvidenceDates.length > 0
                ? `Last ${variabilityEvidenceDates.length} points highlighted in waketime chart.`
                : 'No recent points in this range.',
            dates: variabilityEvidenceDates
          }
        : null;
  $: activeEvidenceMetric =
    activeEvidence?.key === 'schedule_shift'
      ? ('bedtime' as const)
      : activeEvidence?.key === 'variability'
        ? ('waketime' as const)
        : null;
  $: weeklyLensSupportedForMetric = metric === 'duration' || metric === 'quality';
  $: weeklyLensHint =
    showWeeklyLens && !weeklyLensSupportedForMetric
      ? 'Weekly lens is available for duration and quality. Daily data remains shown for this metric.'
      : showWeeklyLens && weeklyLensSupportedForMetric && !weeklySummary
        ? 'Weekly lens data is temporarily unavailable; showing daily data.'
        : null;
  $: scheduleShiftEvidenceAvailable = scheduleShiftEvidenceDates.length > 0;
  $: variabilityEvidenceAvailable = variabilityEvidenceDates.length > 0;

  function focusRecommendationEvidence(focusKey: EvidenceFocusKey) {
    activeEvidenceFocusKey = focusKey;
    view = 'chart';
    showPriorComparator = true;
    setMetric(focusKey === 'schedule_shift' ? 'bedtime' : 'waketime');
  }

  function clearEvidenceFocus() {
    activeEvidenceFocusKey = null;
  }

  function setMetric(nextMetric: MetricKey) {
    const focusMetric =
      activeEvidenceFocusKey === 'schedule_shift'
        ? 'bedtime'
        : activeEvidenceFocusKey === 'variability'
          ? 'waketime'
          : null;
    if (focusMetric && focusMetric !== nextMetric) {
      activeEvidenceFocusKey = null;
    }
    metric = nextMetric;
  }

  $: if (view === 'chart') {
    void renderChart(
      sortedBars,
      sortedPriorBars,
      weeklySummary,
      metric,
      $theme,
      showPriorComparator,
      showWeeklyLens,
      currentRangeDays
    );
  } else {
    chart?.destroy();
    chart = null;
  }
</script>

<section class="space-y-6" data-testid="trends-page">
  <div class="surface-card rounded-2xl px-5 py-4" data-testid="trends-controls">
    <div class="flex flex-wrap items-start justify-between gap-4">
      <div>
        <h2 class="text-default text-2xl font-semibold">Trends</h2>
        <p class="text-muted text-sm">{rangeLabel} · {from} – {to}</p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        {#each views as option}
          <button
            type="button"
            data-testid={`trends-view-${option.key}`}
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
            data-testid={`trends-preset-${preset.days}`}
            class={`toggle-pill rounded-full px-3 py-1 text-xs font-semibold transition ${
              currentRangeDays === preset.days ? 'toggle-pill--active' : ''
            }`}
            on:click={() => applyPreset(preset.days)}
          >
            {preset.label}
          </button>
        {/each}
      </div>
      <form class="flex flex-wrap items-end gap-2" on:submit={refresh} data-testid="trends-range-form">
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
          data-testid="trends-apply-range"
          class="btn-primary focus-ring touch-target inline-flex items-center rounded-full px-4 py-2 text-sm shadow-sm disabled:cursor-not-allowed disabled:opacity-60"
          disabled={loading}
        >
          {#if loading}Loading...{:else}Apply{/if}
        </button>
      </form>
    </div>
  </div>

  {#if errorMsg}
    <div class="state-card state-card--error" role="alert" data-testid="trends-state-error">
      {errorMsg}
    </div>
  {/if}
  <div class="surface-card grid gap-3 rounded-2xl px-5 py-4 text-sm sm:grid-cols-2 lg:grid-cols-4" data-testid="trends-summary-metrics">
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

  {#if hasInsightCards}
    <div class="surface-card grid gap-3 rounded-2xl px-5 py-4 text-sm md:grid-cols-2">
      {#if showScheduleShiftInsight}
        <div class="surface-muted rounded-xl px-4 py-3">
          <div class="text-muted text-xs font-semibold uppercase tracking-wide">Schedule shift insight</div>
          <div class="text-default mt-1 text-sm font-semibold">
            Weekend midpoint shift {formatSignedMinutes(personalization?.metrics.social_jetlag.current_delta_min)}
          </div>
          <p class="text-muted mt-1 text-xs">{scheduleShiftInsight?.rationale}</p>
          <div class="mt-2 flex flex-wrap items-center justify-between gap-2 text-xs">
            <span class="text-muted">
              Evidence: {scheduleShiftEvidenceAvailable
                ? `${scheduleShiftEvidenceDates.length} weekend chart points`
                : 'No weekend points in this range'}
            </span>
            <button
              type="button"
              class="btn-outline focus-ring rounded-full px-3 py-1 text-xs font-semibold disabled:cursor-not-allowed disabled:opacity-60"
              on:click={() => focusRecommendationEvidence('schedule_shift')}
              disabled={!scheduleShiftEvidenceAvailable}
            >
              {activeEvidenceFocusKey === 'schedule_shift' ? 'Evidence shown' : 'Show in chart'}
            </button>
          </div>
        </div>
      {/if}
      {#if showVariabilityInsight}
        <div class="surface-muted rounded-xl px-4 py-3">
          <div class="text-muted text-xs font-semibold uppercase tracking-wide">Variability insight</div>
          <div class="text-default mt-1 text-sm font-semibold">
            Current timing variability {formatDurationHMM(personalization?.metrics.schedule_variability.current_variability_min)}
          </div>
          <p class="text-muted mt-1 text-xs">{regularityInsight?.rationale}</p>
          <div class="mt-2 flex flex-wrap items-center justify-between gap-2 text-xs">
            <span class="text-muted">
              Evidence: {variabilityEvidenceAvailable
                ? `Last ${variabilityEvidenceDates.length} chart points`
                : 'No recent points in this range'}
            </span>
            <button
              type="button"
              class="btn-outline focus-ring rounded-full px-3 py-1 text-xs font-semibold disabled:cursor-not-allowed disabled:opacity-60"
              on:click={() => focusRecommendationEvidence('variability')}
              disabled={!variabilityEvidenceAvailable}
            >
              {activeEvidenceFocusKey === 'variability' ? 'Evidence shown' : 'Show in chart'}
            </button>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#if view === 'chart'}
    <div class="surface-card space-y-4 rounded-2xl p-4" data-testid="trends-chart-view">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="toggle-group flex flex-wrap items-center gap-2 rounded-full p-1" data-testid="trends-metric-toggle-group">
          {#each metrics as option}
            <button
              type="button"
              data-testid={`trends-metric-${option.key}`}
              class={`toggle-pill rounded-full px-3 py-1 text-xs font-semibold transition ${
                metric === option.key ? 'toggle-pill--active' : ''
              }`}
              on:click={() => setMetric(option.key)}
            >
              {option.label}
            </button>
          {/each}
        </div>
        <div class="flex flex-wrap items-center gap-3">
          <label class="text-muted flex items-center gap-2 text-xs font-medium">
            <input
              type="checkbox"
              data-testid="trends-toggle-prior-comparator"
              class="input-base h-4 w-4 rounded"
              bind:checked={showPriorComparator}
            />
            Compare to prior period
          </label>
          <label class="text-muted flex items-center gap-2 text-xs font-medium">
            <input
              type="checkbox"
              data-testid="trends-toggle-weekly-lens"
              class="input-base h-4 w-4 rounded"
              bind:checked={showWeeklyLens}
              on:change={() => {
                if (showWeeklyLens) {
                  loadBars();
                }
              }}
            />
            Weekly smoothing lens
          </label>
          <div class="text-muted text-xs">{prioritizedExplanation}</div>
        </div>
      </div>
      {#if weeklyLensHint}
        <div class="surface-muted rounded-xl px-3 py-2 text-xs">
          <div class="text-muted">{weeklyLensHint}</div>
        </div>
      {/if}
      <div class="surface-muted flex flex-wrap items-center justify-between gap-2 rounded-xl px-3 py-2 text-xs">
        <div class="text-default font-medium">{selectedMetricQuestion}</div>
        <div class="text-muted">{metricDeltaSummary}</div>
      </div>
      {#if activeEvidence}
        <div class="surface-muted flex flex-wrap items-center justify-between gap-2 rounded-xl px-3 py-2 text-xs">
          <div>
            <div class="text-default font-medium">{activeEvidence.title}</div>
            <div class="text-muted mt-1">{activeEvidence.description}</div>
          </div>
          <button
            type="button"
            class="btn-outline focus-ring rounded-full px-3 py-1 text-xs font-semibold"
            on:click={clearEvidenceFocus}
          >
            Clear focus
          </button>
        </div>
      {/if}
      <div class="surface-muted rounded-xl px-3 py-2 text-xs">
        {#if metric === 'duration'}
          {#if durationReferenceBand}
            <div class="text-default font-medium">
              Personal reference band: {formatDurationHMM(durationReferenceBand.min)}–{formatDurationHMM(durationReferenceBand.max)}
            </div>
            <div class="text-muted mt-1">
              Outside-band nights recently: {formatPercent(durationOutOfRangePct)}
            </div>
          {:else}
            <div class="text-muted">Personal reference band will appear after enough duration history is available.</div>
          {/if}
        {:else if metric === 'quality'}
          <div class="text-default font-medium">Quality midpoint context: 3 is the middle of the 1–5 scale.</div>
          <div class="text-muted mt-1">
            {#if avgQuality != null}
              Average quality is {formatQuality(avgQuality)} ({formatSignedQuality(qualityMidpointDelta)} vs midpoint).
            {:else}
              Add quality scores to compare your average against midpoint.
            {/if}
          </div>
        {:else}
          {#if scheduleVariabilityMetric?.eligible && scheduleVariabilityMetric.current_variability_min != null}
            <div class="text-default font-medium">
              Timing variability: {formatDurationHMM(scheduleVariabilityMetric.current_variability_min)}
            </div>
            <div class="text-muted mt-1">
              {#if scheduleVariabilityDeltaMin != null}
                {Math.abs(Math.round(scheduleVariabilityDeltaMin))} min {scheduleVariabilityDeltaMin > 0 ? 'more variable' : scheduleVariabilityDeltaMin < 0 ? 'more consistent' : 'unchanged'} vs prior period.
              {:else}
                Prior-period variability comparison is not available yet.
              {/if}
            </div>
          {:else}
            <div class="text-muted">Variability cue appears once enough bedtime/wake-time history is available.</div>
          {/if}
        {/if}
      </div>
      <div class="h-72" data-testid="trends-chart-panel">
        {#if loading}
          <div class="text-muted flex h-full items-center justify-center text-sm" data-testid="trends-state-loading-chart">Loading chart…</div>
        {:else if sortedBars.length === 0}
          <div class="text-muted flex h-full items-center justify-center text-sm" data-testid="trends-state-empty-chart">No data in range.</div>
        {:else}
          <canvas bind:this={canvasEl} data-testid="trends-chart-canvas"></canvas>
        {/if}
      </div>
    </div>
  {:else}
    <div class="surface-card space-y-4 rounded-2xl p-4" data-testid="trends-schedule-view">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <h3 class="text-default text-sm font-semibold">Schedule view</h3>
        <span class="text-muted text-xs">24h timeline</span>
      </div>
      {#if loading}
        <div class="text-muted text-sm" data-testid="trends-state-loading-schedule">Loading schedule…</div>
      {:else if sortedBars.length === 0}
        <div class="text-muted text-sm" data-testid="trends-state-empty-schedule">No data in range.</div>
      {:else}
        <div class="space-y-3" data-testid="trends-schedule-list">
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
