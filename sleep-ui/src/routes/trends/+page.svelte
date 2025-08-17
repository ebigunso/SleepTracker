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
    return d.toISOString().slice(0, 10);
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
      // @ts-ignore - suppress type resolution issues for chart.js in TS bundler mode
      const mod: any = await import('chart.js');
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

<section class="space-y-4">
  <header class="flex items-end justify-between gap-4">
    <h2 class="text-xl font-semibold text-gray-900">Trends</h2>
    <form class="flex items-end gap-2" on:submit={refresh}>
      <div>
        <label for="from-date" class="block text-xs text-gray-600">From</label>
        <input
          id="from-date"
          type="date"
          bind:value={from}
          class="rounded-md border-gray-300 text-sm focus:border-indigo-500 focus:ring-indigo-500"
        />
      </div>
      <div>
        <label for="to-date" class="block text-xs text-gray-600">To</label>
        <input
          id="to-date"
          type="date"
          bind:value={to}
          class="rounded-md border-gray-300 text-sm focus:border-indigo-500 focus:ring-indigo-500"
        />
      </div>
      <button
        type="submit"
        class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-medium text-white hover:bg-indigo-700"
        disabled={loading}
      >
        {#if loading}Loading...{:else}Apply{/if}
      </button>
    </form>
  </header>

  {#if errorMsg}
    <div class="rounded border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">
      {errorMsg}
    </div>
  {/if}

  <div class="rounded-lg border bg-white p-3">
    <canvas bind:this={canvasEl} height="200"></canvas>
  </div>
</section>
