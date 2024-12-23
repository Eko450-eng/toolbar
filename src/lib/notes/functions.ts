import { type Note } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { type Editor } from '@tiptap/core';

export async function getnotes(
	notes: Note[],
	project: string,
): Promise<Note[]> {
	if (notes.length <= 0) notes = await invoke('getnote', {});
	let note: Note[] = await invoke('getnote', {});
	return note;
	// await getprojects();
}

export async function getprojects(note: string) {
	try {
		return await invoke('getprojects', { note });
	} catch (error) {
		console.error('Invoke error:', error);
	}
}

export async function createNote(): Promise<Note[]> {
	await invoke('createnote', {});
	return await invoke('getnote');
}

export async function addNote(note: Note, editor: Editor): Promise<Note[]> {
	let newNote = note;
	newNote.note = editor?.getHTML() ?? '';
	await invoke('addnote', { note: newNote });
	return await invoke('getnote');
}

export async function deleteNote(id: number): Promise<Note[]> {
	await invoke('deletenote', { id });
	return await invoke('getnote');
}
