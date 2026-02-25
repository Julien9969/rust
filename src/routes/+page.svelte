<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import StatusDisplay from "$lib/components/ActivityEntry.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import MenuBar from "$lib/components/MenuBar.svelte";
  import DonutChart from "$lib/components/DonutChart.svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="min-h-screen bg-background p-2">
  <MenuBar/>
  <div class="mx-auto space-y-4 scale-95">
    <div class="flex items-start gap-4 ">

      <Card.Root class="max-w-md w-[40%]">
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
      <div class="w-[60%]">
        <DonutChart />
      </div>
    </div>
    <StatusDisplay />
    <div class="flex justify-center">
      <Button href="/second" variant="outline">
        Go to Second Page →
      </Button>
    </div>
  </div>
</main>
