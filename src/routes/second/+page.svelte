<script lang="ts">
  import StatusDisplay from "$lib/components/ActivityEntry.svelte";
  import { Button } from "$lib/components/ui/button";
</script>

<main class="min-h-screen bg-background p-8">
  <div class="max-w-4xl mx-auto space-y-8">
    <div class="text-center space-y-4">
      <h1 class="text-4xl font-bold tracking-tight">Welcome to Tauri + Svelte</h1>
      
      <div class="flex justify-center gap-6 py-4">
        <a href="https://vitejs.dev" target="_blank" class="transition-transform hover:scale-110">
          <img src="/vite.svg" class="h-20" alt="Vite Logo" />
        </a>
        <a href="https://tauri.app" target="_blank" class="transition-transform hover:scale-110">
          <img src="/tauri.svg" class="h-20" alt="Tauri Logo" />
        </a>
        <a href="https://kit.svelte.dev" target="_blank" class="transition-transform hover:scale-110">
          <img src="/svelte.svg" class="h-20" alt="SvelteKit Logo" />
        </a>
      </div>
      
      <p class="text-muted-foreground">
        Click on the Tauri, Vite, and SvelteKit logos to learn more.
      </p>
    </div>
    
    <StatusDisplay />

    <div class="flex justify-center">
      <Button href="/" variant="outline">
        ← Back to Home
      </Button>
    </div>
  </div>
</main>
