<script lang="ts">
import { Menu, Submenu } from "@tauri-apps/api/menu";
import { ModeWatcher } from "mode-watcher";
import "../app.css";
import { getToastStore, initializeStores, Toast } from "@skeletonlabs/skeleton";
import { onMount } from "svelte";

initializeStores();
const toastStore = getToastStore();
let { children } = $props();

async function create() {
	const macOS = navigator.userAgent.includes("Macintosh");

	const submenu = await Submenu.new({
		text: "Settings",
		items: [
			{
				text: `Switch theme`,
				enabled: true,
				items: [
					{
						text: `Skeleton`,
						enabled: true,
						action() {
							setTheme("skeleton");
						},
					},
					{
						text: `modern`,
						enabled: true,
						action() {
							setTheme("modern");
						},
					},
					{
						text: `wintry`,
						enabled: true,
						action() {
							setTheme("wintry");
						},
					},
					{
						text: `gold-nouveau`,
						enabled: true,
						action() {
							setTheme("gold-nouveau");
						},
					},
					{
						text: `rocket`,
						enabled: true,
						action() {
							setTheme("rocket");
						},
					},
					{
						text: `seafoam`,
						enabled: true,
						action() {
							setTheme("seafoam");
						},
					},
					{
						text: `sahara`,
						enabled: true,
						action() {
							setTheme("sahara");
						},
					},
					{
						text: `crimson`,
						enabled: true,
						action() {
							setTheme("crimson");
						},
					},
					{
						text: `vintage`,
						enabled: true,
						action() {
							setTheme("vintage");
						},
					},
					{
						text: `hamlindigo`,
						enabled: true,
						action() {
							setTheme("hamlindigo");
						},
					},
					{
						text: `Skeleton`,
						enabled: true,
						action() {
							setTheme("skeleton");
						},
					},
				],
			},
		],
	});

	const menu = await Menu.new({
		items: [submenu],
	});
	await (macOS ? menu.setAsAppMenu() : menu.setAsWindowMenu());
}

let theme = $state("skeleton");

function setTheme(selectedTheme: string) {
	theme = selectedTheme;
	const savedMessage = { message: `Changed theme to: ${theme}` };
	toastStore.trigger(savedMessage);

	localStorage.setItem("theme", theme);
	document.body.setAttribute("data-theme", theme);
}

onMount(() => {
	create();
	let storedtheme = localStorage.getItem("theme");
	theme = storedtheme ?? "";
});

$effect(() => {
	document.body.setAttribute("data-theme", theme);
});
</script>


<div class="h-full">
    <ModeWatcher />
    {@render children?.()}
</div>
