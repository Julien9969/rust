<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import * as Chart from "$lib/components/ui/chart/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { PieChart, Text } from "layerchart";

  type GroupedEntry = { name: string; totalMs: number };
  type ChartItem = { app: string; duration: number; color: string };

  const CHART_COLORS = [
    "var(--chart-1)",
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
  ];

  let chartData: ChartItem[] = $state([]);
  let chartConfig: Chart.ChartConfig = $state({ duration: { label: "Minutes" } });
  let totalMinutes = $state(0);
  let isLoading = $state(true);
  let error = $state('');
  let todayLabel = $state('');
  let intervalId: number | null = null;

  async function updateChart() {
    const now = new Date();
    const startOfDay = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const endOfDay = new Date(startOfDay.getTime() + 24 * 60 * 60 * 1000);
    todayLabel = startOfDay.toLocaleDateString(undefined, { month: 'long', day: 'numeric', year: 'numeric' });

    try {
      const entries = await invoke<GroupedEntry[]>('get_grouped_data', {
        groupBy: 'app_name',
        startTime: startOfDay.getTime(),
        endTime: endOfDay.getTime(),
      });

      console.log('Grouped entries:', entries);

      // Top 9 + "Other" bucket
      const top = entries.slice(0, 9);
      const otherMs = entries.slice(9).reduce((sum, e) => sum + e.totalMs, 0);
      if (otherMs > 0) top.push({ name: 'Other', totalMs: otherMs });

      chartData = top.map((e, i) => ({
        app: e.name,
        duration: Math.round(e.totalMs / 60000),
        color: CHART_COLORS[i % CHART_COLORS.length],
      }));

      chartConfig = {
        duration: { label: 'Minutes' },
        ...Object.fromEntries(chartData.map(({ app, color }) => [app, { label: app, color }])),
      };

      totalMinutes = chartData.reduce((acc, d) => acc + d.duration, 0);
    } catch (e) {
      error = String(e);
    } finally {
      isLoading = false;
    }
  }

  onMount(async () => {
    await updateChart();
    intervalId = window.setInterval(updateChart, 60 * 1000); // Refresh every 1 minutes
  });

  onDestroy(() => {
    if (intervalId !== null) {
      clearInterval(intervalId);
    }
  });

  const totalDisplay = $derived(
    totalMinutes >= 60
      ? `${Math.floor(totalMinutes / 60)}h ${totalMinutes % 60}m`
      : `${totalMinutes}m`
  );
</script>

<Card.Root class="flex flex-col" style="padding: 0.8rem 0rem;">
  <Card.Header class="items-center">
    <Card.Title>App Usage Today</Card.Title>
    <Card.Description>{todayLabel}</Card.Description>
  </Card.Header>
  <Card.Content class="flex-1">
    {#if isLoading}
      <div class="flex items-center justify-center" style="height: 200px;">
        <span class="text-muted-foreground text-sm">Loading…</span>
      </div>
    {:else if error}
      <div class="flex items-center justify-center" style="height: 200px;">
        <span class="text-destructive text-sm">{error}</span>
      </div>
    {:else if chartData.length === 0}
      <div class="flex items-center justify-center" style="height: 200px;">
        <span class="text-muted-foreground text-sm">No activity recorded today.</span>
      </div>
    {:else}
      <Chart.Container config={chartConfig} class="mx-auto aspect-square" style="max-height: 200px;">
        <PieChart
          data={chartData}
          key="app"
          value="duration"
          c="color"
          innerRadius={70}
          padding={6}
          props={{ pie: { motion: "tween" } }}
        >
          {#snippet aboveMarks()}
            <Text
              value={totalDisplay}
              textAnchor="middle"
              verticalAnchor="middle"
              class="fill-foreground text-3xl! font-bold"
              dy={3}
            />
            <Text
              value="Today"
              textAnchor="middle"
              verticalAnchor="middle"
              class="fill-muted-foreground! text-muted-foreground"
              dy={22}
            />
          {/snippet}
          {#snippet tooltip()}
            <Chart.Tooltip hideLabel />
          {/snippet}
        </PieChart>
      </Chart.Container>
    {/if}
  </Card.Content>
  <Card.Footer class="flex-col gap-1 text-sm">
    <div class="text-muted-foreground leading-none">
      Showing active app time for today
    </div>
  </Card.Footer>
</Card.Root>
