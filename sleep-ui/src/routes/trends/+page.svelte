<script lang="ts">
  import { onMount } from 'svelte';
  import { apiGet } from '$lib/api';
  let ChartJS: any = null;

  type SleepBar = {
    date: string; // ISO date (YYYY-MM-DD)
    bed_time: string; // HH:MM:SS
    wake_time: string; // HH:MM:SS
    quality?: number | null;
    duration_min?: number | null;
  };

  let canvasEl: HTMLCanvasElement | null = null;
  let chart: any | null = null;

  let from = '';
  let to = '';
  let bars: SleepBar[] = [];
  let loading = false;
  let errorMsg: string | null = null;

  function iso(d: Date) {
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, '0');
    const dd = String(d.getDate()).padStart(2, '0');
    return `${yyyy}-${mm}-${dd}`;
  }

  function setDefaultRange(days = 14) {
    const end = new Date();
    const start = new Date();
    start.setDate(end.getDate() - (days - 1));
    from = iso(start);
    to = iso(end);
  }

  async function loadBars() {
    if (!from || !to) setDefaultRange();
    loading = true;
    errorMsg = null;
    try {
      const q = new URLSearchParams({ from, to }).toString();
      const data = await apiGet<SleepBar[]>(`/api/trends/sleep-bars?${q}`);
      bars = data;
      renderChart();
    } catch (e) {
      console.error(e);
      errorMsg = 'Failed to load trends';
    } finally {
      loading = false;
    }
  }

  async function renderChart() {
    if (!canvasEl) return;

    const labels = bars.map((b) => b.date);
    const durations = bars.map((b) => (b.duration_min ?? 0));
    const qualities = bars.map((b) => (b.quality ?? null));

    if (!ChartJS) {
      // typed dynamic import for chart.js to satisfy TS under bundler mode
      const mod = (await import('chart.js')) as typeof import('chart.js');
      ChartJS = mod.Chart;
      ChartJS.register(...mod.registerables);
    }

    // Destroy previous chart instance
    chart?.destroy();
    chart = new ChartJS(canvasEl, {
      type: 'bar',
      data: {
        labels,
        datasets: [
          {
            label: 'Duration (min)',
            data: durations,
            borderWidth: 1,
            backgroundColor: 'rgba(99, 102, 241, 0.5)', // indigo-500/50
            borderColor: 'rgb(99, 102, 241)'
          }
        ]
      },
      options: {
        responsive: true,
        scales: {
          y: {
            title: { display: true, text: 'Minutes' },
            beginAtZero: true
          },
          x: {
            title: { display: true, text: 'Wake Date' }
          }
        },
        plugins: {
          tooltip: {
            callbacks: {
              afterBody(ctx: any) {
                const i = ctx[0].dataIndex;
                const q = qualities[i];
                return q != null ? `Quality: ${q}` : '';
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
</script>

<section class="space-y-6">
  <header class="flex flex-wrap items-end justify-between gap-4">
    <div>
      <h2 class="text-2xl font-semibold text-slate-900">Trends</h2>
      <p class="text-sm text-slate-500">Review sleep duration and quality over time.</p>
    </div>
    <form class="flex flex-wrap items-end gap-2 rounded-xl border border-slate-200 bg-white px-3 py-2 shadow-sm" on:submit={refresh}>
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
        class="focus-ring touch-target inline-flex items-center rounded-full bg-indigo-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-700"
        disabled={loading}
      >
        {#if loading}Loading...{:else}Apply{/if}
      </button>
    </form>
  </header>

  {#if errorMsg}
    <div class="state-card state-card--error" role="alert" aria-live="polite">
      {errorMsg}
    </div>
  {/if}

  {#if loading}
    <div class="state-card state-card--loading" role="status" aria-live="polite">
      Loading trends...
    </div>
  {/if}

  <div class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm" aria-busy={loading}>
    <canvas bind:this={canvasEl} height="200"></canvas>
  </div>
</section>
