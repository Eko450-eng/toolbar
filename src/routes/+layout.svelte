<script lang="ts">
import { ModeWatcher } from "mode-watcher";
import "../app.css";
import { initializeStores } from "@skeletonlabs/skeleton";
import { onMount } from "svelte";
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

onMount(() => {
	let theme = localStorage.getItem("theme");
	if (theme) {
		themecount = parseInt(theme);
		console.log(theme);
	} else {
		localStorage.setItem("theme", "0");
	}
});

$effect(() => {
	let s = themes[themecount];
	localStorage.setItem("theme", themecount.toString());
	console.log(document.body.setAttribute("data-theme", s));
});
</script>

<div class="h-full">
    <button 
        class="btn variant-filled-primary absolute"
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
