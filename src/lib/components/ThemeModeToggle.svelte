<script lang="ts">
 import { Sun, Moon } from "@lucide/svelte"; 
 import { toggleMode, mode } from "mode-watcher";
 import { Button } from "$lib/components/ui/button/index.js";

 let { class: className = "" } = $props();
</script>

<Button onclick={toggleMode} class={`hover:bg-transparent hover:text-current ${className}`} variant="ghost" size="icon">
 {#if mode.current === 'light'}
    <Sun class="h-[1.1rem] w-[1.1rem]" />
 {:else}
  <Moon class="h-[1.1rem] w-[1.1rem]" />
 {/if}
 <span class="sr-only">Toggle theme</span>
</Button>