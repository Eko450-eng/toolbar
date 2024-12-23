import { writable, type Writable, get } from 'svelte/store';

export const theme = writable('skeleton');

export function setThemeStore(newTheme: string) {
	theme.set(newTheme);
}

export function setTheme(selectedTheme: string) {
	setThemeStore(selectedTheme);
	localStorage.setItem('theme', get(theme));
	document.body.setAttribute('data-theme', get(theme));
}

export function resetTheme() {
	localStorage.setItem('theme', get(theme));
	document.body.setAttribute('data-theme', get(theme));
}
