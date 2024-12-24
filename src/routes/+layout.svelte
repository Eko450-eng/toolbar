<script lang="ts">
import { Menu, Submenu } from '@tauri-apps/api/menu';
import { ModeWatcher } from 'mode-watcher';
import '../app.css';
import { initializeStores } from '@skeletonlabs/skeleton';
import { onMount } from 'svelte';
import { getThemeOptions } from './layout.svelte';
import { setTheme, theme } from '$lib/stores/theme';
import { toggleMode } from 'mode-watcher';
import {
	triggerDelete,
	triggerEditor,
	triggerNewNote,
	triggerSave,
} from '$lib/stores/triggers';

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
				action() {
					$triggerNewNote += 1;
				},
			},
			{
				text: `Speichern`,
				enabled: true,
				action() {
					$triggerSave += 1;
				},
			},
			{
				text: `Löschen`,
				enabled: true,
				action() {
					$triggerDelete += 1;
				},
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
				enabled: true,
				items: [
					{
						text: '1',
						action() {
							triggerEditor.set('headline1');
						},
					},
					{
						text: '2',
						action() {
							triggerEditor.set('headline2');
						},
					},
					{
						text: '3',
						action() {
							triggerEditor.set('headline3');
						},
					},
					{
						text: '4',
						action() {
							triggerEditor.set('headline4');
						},
					},
				],
			},
			{
				text: `Bold`,
				enabled: true,
				action() {
					triggerEditor.set('bold');
				},
			},
			{
				text: `Italic`,
				enabled: true,
				action() {
					triggerEditor.set('italic');
				},
			},
			{
				text: `strike`,
				enabled: true,
				action() {
					triggerEditor.set('strike');
				},
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
				text: `Dark Mode - DIE APP IST FÜR DARKMODE Ausgelegt!!!!!`,
				enabled: true,
				action() {
					toggleMode();
				},
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
