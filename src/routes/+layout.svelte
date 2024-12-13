<script lang="ts">
import { ModeWatcher } from "mode-watcher";
import "../app.css";
import { initializeStores } from "@skeletonlabs/skeleton";
let { children } = $props();
initializeStores();
</script>

<ModeWatcher />
{@render children?.()}
