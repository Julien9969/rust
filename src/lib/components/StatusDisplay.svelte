<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import * as Card from "$lib/components/ui/card";

  type StatusUpdate = {
    appName: string;
    idleTime: number;
  };

  let statusUpdate = $state<StatusUpdate | null>(null);
  let lastUpdated = $state<string>("");

  onMount(() => {
    const unlisten = listen<StatusUpdate>('status-update', (event) => {
      console.log(
        `status update: ${event.payload.appName} has been idle for ${event.payload.idleTime}ms`
      );
      statusUpdate = event.payload;
      lastUpdated = new Date().toLocaleTimeString();
    });

    return () => {
      unlisten.then(fn => fn());
    };
  });

  function formatIdleTime(ms: number): string {
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);

    if (hours > 0) {
      return `${hours}h ${minutes % 60}m`;
    } else if (minutes > 0) {
      return `${minutes}m ${seconds % 60}s`;
    } else {
      return `${seconds}s`;
    }
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>Activity Status</Card.Title>
    <Card.Description>Real-time tracking of your active window and idle time</Card.Description>
  </Card.Header>
  <Card.Content>
    {#if statusUpdate}
      <div class="space-y-3">
        <div class="flex justify-between items-center p-3 bg-muted rounded-lg">
          <span class="text-sm font-medium text-muted-foreground">Active Application</span>
          <span class="text-sm font-semibold">{statusUpdate.appName}</span>
        </div>
        
        <div class="flex justify-between items-center p-3 bg-muted rounded-lg">
          <span class="text-sm font-medium text-muted-foreground">Idle Time</span>
          <span class="text-sm font-semibold font-mono text-primary">{formatIdleTime(statusUpdate.idleTime)}</span>
        </div>
        
        <div class="flex justify-between items-center p-3 bg-muted rounded-lg">
          <span class="text-sm font-medium text-muted-foreground">Last Updated</span>
          <span class="text-sm text-muted-foreground">{lastUpdated}</span>
        </div>
      </div>
    {:else}
      <p class="text-center text-muted-foreground italic py-8">
        Waiting for status updates...
      </p>
    {/if}
  </Card.Content>
</Card.Root>
