<script lang="ts">
import { onMount } from 'svelte';
import { Editor } from '@tiptap/core';
import Color from '@tiptap/extension-color';
import StarterKit from '@tiptap/starter-kit';
import TextStyle from '@tiptap/extension-text-style';
import ListItem from '@tiptap/extension-list-item';
import { debounce } from 'lodash-es';
import '../lib/editorapp/styles.css';
import { HeadProps } from '$lib/editorapp/header';
import { initNote, type Note, type NoteWithTasks } from '$lib/types';
import Globalattr from '$lib/editorapp/globalattr';
import CustomTaskList from '$lib/editorapp/customtasklist';
import {
	getModalStore,
	getToastStore,
	Modal,
	Toast,
	type ModalSettings,
} from '@skeletonlabs/skeleton';
import Navbar from './navbar.svelte';
import {
	addNote,
	createNote,
	deleteNote,
	getnotes,
} from '$lib/notes/functions';
import { invoke } from '@tauri-apps/api/core';
import TaskItem from '@tiptap/extension-task-item';
import EditorView from './editorView.svelte';
import Searchbar from './searchbar.svelte';
import {
	triggerDelete,
	triggerEditor,
	triggerNewNote,
	triggerSave,
} from '$lib/stores/triggers';
import { SearchExtension } from '$lib/editorapp/searchExtension';
import SearchControl from '$lib/editorapp/searchControl.svelte';

let notes = $state<Note[]>([]);
let tasks = $state<NoteWithTasks[]>([]);
let note = $state<Note>(initNote);

let editorEditable = $state<boolean>();
let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

const modalStore = getModalStore();
const toastStore = getToastStore();

const savedMessage = { message: 'Saved' };

const modal: ModalSettings = {
	type: 'prompt',
	value: '',
	valueAttr: { type: 'text', minlength: 3, maxlength: 10, required: false },
	response: (r: string) => runcommand(r),
};

triggerSave.subscribe(async () => {
	if ($triggerSave < 1) return;
	await save(note, editor, false);
	triggerSave.set(0);
});

triggerDelete.subscribe(async () => {
	if ($triggerDelete < 1 || !note.id) return;
	notes = await deleteNote(note.id);
	triggerDelete.set(0);
});

triggerNewNote.subscribe(async () => {
	if ($triggerNewNote < 1) return;
	notes = await createNote();
	triggerNewNote.set(0);
});

triggerEditor.subscribe(() => {
	if ($triggerEditor === 'bold') editor?.commands.setBold();
	if ($triggerEditor === 'headline1') editor?.commands.setHeading({ level: 1 });
	if ($triggerEditor === 'headline2') editor?.commands.setHeading({ level: 2 });
	if ($triggerEditor === 'headline3') editor?.commands.setHeading({ level: 3 });
	if ($triggerEditor === 'headline4') editor?.commands.setHeading({ level: 4 });
	if ($triggerEditor === 'italic') editor?.commands.setItalic();
	if ($triggerEditor === 'strike') editor?.commands.setStrike();
	triggerEditor.set('');
});

async function save(note: Note, editor: Editor | null, noToast?: boolean) {
	if (editor) await addNote(note, editor);
	if (!noToast) toastStore.trigger(savedMessage);
	await gettask();
}

async function runcommand(query: string) {
	if (query.startsWith('/')) {
		try {
		} catch (error) {
			console.error('Invoke error:', error);
		}
	} else if (query.startsWith(':')) {
		if (query.substring(1).trim() === 'save') {
			await save(note, editor);
		}
	}
}

async function gettask() {
	tasks = await invoke('gettask');
}

function setEditor() {
	editor = null;
	editor = new Editor({
		editable: false,
		element: element,
		extensions: [
			SearchExtension,
			Globalattr,
			CustomTaskList,
			TaskItem,
			HeadProps,
			Color.configure({ types: [TextStyle.name, ListItem.name] }),
			StarterKit,
		],
		content: note.note,
		onTransaction: () => (editor = editor),
		onUpdate() {
			if (note.id !== 0) debouncedSave();
		},
		editorProps: {
			attributes: {
				class: 'w-full',
			},
		},
	});
}

const debouncedSave = debounce(async () => {
	try {
		await save(note, editor, true);
	} catch (error) {
		console.error('Auto-save failed:', error);
	}
}, 200);

let search = $state(false);

onMount(async () => {
	if (!editor && notes.length == 0) {
		notes = await getnotes(notes, '');
		gettask();
		if (!editor) setEditor();
	}
	// Save on Ctrl+S
	window.addEventListener('keypress', async (key) => {
		if (key.ctrlKey && key.code === 'KeyF') {
			search = true;
		} else if (key.ctrlKey && key.code === 'KeyP') {
			modalStore.trigger(modal);
		} else if (key.code === 'Escape') {
			search = false;
			if (!editor) return;
			editor.commands.clearSearch();
		} else if (key.shiftKey && key.code === 'Tab') {
			key.preventDefault();
		} else if (key.ctrlKey && key.code === 'KeyN') {
			notes = await createNote();
		} else if (key.ctrlKey && key.code === 'KeyS') {
			await save(note, editor);
		}
	});
});
</script>

<div class="flex h-dvh" >
    {#if editor && search}
        <SearchControl {editor} />
    {/if}
    <Navbar bind:notes bind:note bind:editor />
    <div
        class="container w-100 flex flex-col gap-4 prose prose-invert text-white overflow-auto"
    >
        <label class="label">
        <span>Title</span>
            <input
                readonly={editorEditable}
                type="text"
                id="title"
                class="input px-2"
                placeholder="Note"
                bind:value={note.title}
            />
        </label>
        <EditorView bind:editor bind:element bind:note={note.note} />
    </div>

    <Searchbar bind:editor bind:note bind:tasks bind:notes />

</div>
<Modal />
<Toast position="br" />
