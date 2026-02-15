<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import StatusDisplay from "$lib/components/StatusDisplay.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
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

    <Card.Root class="max-w-md mx-auto">
      <Card.Header>
        <Card.Title>Greeting</Card.Title>
        <Card.Description>Enter your name to receive a personalized greeting</Card.Description>
      </Card.Header>
      <Card.Content>
        <form onsubmit={greet} class="flex gap-2">
          <Input 
            placeholder="Enter a name..." 
            bind:value={name}
            class="flex-1"
          />
          <Button type="submit">Greet</Button>
        </form>
        {#if greetMsg}
          <p class="mt-4 text-sm text-muted-foreground">{greetMsg}</p>
        {/if}
      </Card.Content>
    </Card.Root>

    <StatusDisplay />

    <div class="flex justify-center">
      <Button href="/second" variant="outline">
        Go to Second Page →
      </Button>
    </div>
  </div>
</main>
