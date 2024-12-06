<script lang="ts">
import { ModeWatcher } from "mode-watcher";
import "../app.scss";
let { children } = $props();
</script>
 
<ModeWatcher />
{@render children?.()}
