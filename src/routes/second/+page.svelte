<script lang="ts">
  import StatusDisplay from "$lib/components/ActivityEntry.svelte";
  import { Button } from "$lib/components/ui/button";
</script>

<main class="min-h-screen bg-background p-8">
  <div class="max-w-4xl mx-auto space-y-8">
    <div class="text-center space-y-2">
      <h1 class="text-4xl font-bold tracking-tight">Second Page</h1>
      <p class="text-muted-foreground">
        This is the second page of the application with real-time activity tracking.
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
