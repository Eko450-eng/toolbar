import { writable } from 'svelte/store';

export const triggerSave = writable(0);
export const triggerDelete = writable(0);
export const triggerNewNote = writable(0);
export const triggerEditor = writable('');
