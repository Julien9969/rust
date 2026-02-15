<script lang="ts">
	import '../app.css';
	import './layout.css';
 	import { ModeWatcher } from "mode-watcher";
	const { children } = $props();
</script>

<ModeWatcher />
{@render children()}
