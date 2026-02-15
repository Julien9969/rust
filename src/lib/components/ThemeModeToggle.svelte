<script lang="ts">
 import { Sun, Moon } from "@lucide/svelte";
 
 import { toggleMode, mode } from "mode-watcher";
 import { Button } from "$lib/components/ui/button/index.js";
</script>

<Button onclick={toggleMode} variant="outline" size="icon">
 {#if mode.current === 'light'}
  <Sun class="h-[1.2rem] w-[1.2rem]" />
 {:else}
  <Moon class="h-[1.2rem] w-[1.2rem]" />
 {/if}
 <span class="sr-only">Toggle theme</span>
</Button>