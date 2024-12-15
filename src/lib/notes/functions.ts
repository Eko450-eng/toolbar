import { type Note } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { type Editor } from "@tiptap/core";

export async function getnotes(
	notes: Note[],
	project: string,
): Promise<Note[]> {
	if (notes.length <= 0) notes = await invoke("getnote", { project });
	let note = await invoke("getnote", { project });
	return note;
	// await getprojects();
}

export async function getprojects(note: string) {
	try {
		return await invoke("getprojects", { note });
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

export async function createNote() {
	try {
		await invoke("createnote", {});
		return await invoke("getnote", {});
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

export async function addNote(note: Note, editor: Editor) {
	let newNote = note;
	try {
		newNote.note = editor?.getHTML() ?? "";
		await invoke("addnote", { note: newNote });
		// getprojects();
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

export async function deleteNote(id: number): Note {
	try {
		await invoke("deletenote", { id });
	} catch (error) {
		console.error("Invoke error:", error);
	}
}
