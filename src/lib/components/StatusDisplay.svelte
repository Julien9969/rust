<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

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

<div class="status-card">
  <h2>Activity Status</h2>
  
  {#if statusUpdate}
    <div class="status-content">
      <div class="status-item">
        <span class="label">Active Application:</span>
        <span class="value">{statusUpdate.appName}</span>
      </div>
      
      <div class="status-item">
        <span class="label">Idle Time:</span>
        <span class="value idle-time">{formatIdleTime(statusUpdate.idleTime)}</span>
      </div>
      
      <div class="status-item">
        <span class="label">Last Updated:</span>
        <span class="value time">{lastUpdated}</span>
      </div>
    </div>
  {:else}
    <p class="waiting">Waiting for status updates...</p>
  {/if}
</div>

<style>
  .status-card {
    background-color: #ffffff;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    margin: 0 auto;
    max-width: 600px;
    border: 1px solid #e0e0e0;
  }

  h2 {
    margin-top: 0;
    margin-bottom: 1.5rem;
    color: #646cff;
    font-size: 1.5rem;
    border-bottom: 2px solid #646cff;
    padding-bottom: 0.5rem;
  }

  .status-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .status-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background-color: #f8f8f8;
    border-radius: 6px;
  }

  .label {
    font-weight: 500;
    color: #666;
  }

  .value {
    font-weight: 600;
    color: #0f0f0f;
  }

  .idle-time {
    font-family: 'Courier New', monospace;
    color: #646cff;
  }

  .time {
    font-size: 0.9em;
    color: #888;
  }

  .waiting {
    text-align: center;
    color: #999;
    font-style: italic;
    padding: 2rem;
  }

  @media (prefers-color-scheme: dark) {
    .status-card {
      background-color: #1a1a1a;
      border-color: #333;
    }

    h2 {
      color: #24c8db;
      border-bottom-color: #24c8db;
    }

    .status-item {
      background-color: #2a2a2a;
    }

    .label {
      color: #aaa;
    }

    .value {
      color: #f6f6f6;
    }

    .idle-time {
      color: #24c8db;
    }

    .time {
      color: #888;
    }

    .waiting {
      color: #666;
    }
  }
</style>
