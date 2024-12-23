<script lang="ts">
import { Menu, Submenu } from '@tauri-apps/api/menu';
import { ModeWatcher } from 'mode-watcher';
import '../app.css';
import { initializeStores } from '@skeletonlabs/skeleton';
import { onMount } from 'svelte';
import { getThemeOptions } from './layout.svelte';
import { setTheme, theme } from '$lib/stores/theme';

initializeStores();
let { children } = $props();

async function create(theme?: string) {
	const macOS = navigator.userAgent.includes('Macintosh');

	const fileMenu = await Submenu.new({
		text: 'Datei',
		items: [
			{
				text: `Neu`,
				enabled: true,
			},
			{
				text: `Speichern`,
				enabled: true,
			},
			{
				text: `Löschen`,
				enabled: true,
			},
			{
				text: `In Cloud Speichern`,
				enabled: false,
			},
		],
	});

	const editMenu = await Submenu.new({
		text: 'Bearbeiten',
		items: [
			{
				text: `Headline`,
				enabled: false,
			},
			{
				text: `Bold`,
				enabled: false,
			},
			{
				text: `Italic`,
				enabled: false,
			},
			{
				text: `Underline`,
				enabled: false,
			},
		],
	});

	const settingsMenu = await Submenu.new({
		text: 'Einstellungen',
		items: [
			{
				text: `Design ändern`,
				enabled: true,
				items: getThemeOptions(theme ?? 'skeleton'),
			},
			{
				text: `AI Model`,
				enabled: false,
			},
			{
				text: `AI Key`,
				enabled: false,
			},
			{
				text: `Cloud Daten`,
				enabled: false,
			},
		],
	});

	const menu = await Menu.new({
		items: [fileMenu, editMenu, settingsMenu],
	});
	await (macOS ? menu.setAsAppMenu() : menu.setAsWindowMenu());
}

theme.subscribe(async (newTheme) => await create(newTheme));

onMount(async () => {
	let storedtheme = localStorage.getItem('theme');
	setTheme(storedtheme ?? 'skeleton');
	setTheme($theme);
	await create($theme);
});
</script>


<div class="h-full scroll-auto">
    <ModeWatcher />
    {@render children?.()}
</div>
