<script lang="ts">
import { ModeWatcher } from "mode-watcher";
import "../app.css";
import { initializeStores } from "@skeletonlabs/skeleton";
let { children } = $props();
initializeStores();

let themes = [
	"skeleton",
	"modern",
	"wintry",
	"gold-nouveau",
	"modern",
	"rocket",
	"seafoam",
	"sahara",
	"crimson",
	"vintage",
	"hamlindigo",
];
let themecount = $state(0);
</script>

<div data-sveltekit-preload-data="hover" data-theme={themes[themecount]}>
    <button 
        class="btn variant-filled-primary"
        onclick={()=>{
        if(themecount > 9){
            themecount = 0;
        }else{
            themecount++
        }
    }}>
        {themes[themecount]}
    </button>
    <ModeWatcher />
    {@render children?.()}
</div>
