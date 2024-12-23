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
import { addNote, createNote, getnotes } from '$lib/notes/functions';
import { invoke } from '@tauri-apps/api/core';
import TaskItem from '@tiptap/extension-task-item';
import EditorView from './editorView.svelte';
import Searchbar from './searchbar.svelte';

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

onMount(async () => {
	if (!editor && notes.length == 0) {
		notes = await getnotes(notes, '');
		gettask();
		if (!editor) setEditor();
	}
	// Save on Ctrl+S
	window.addEventListener('keypress', async (key) => {
		if (key.ctrlKey && key.code === 'KeyF') {
			modalStore.trigger(modal);
		} else if (key.shiftKey && key.code === 'Tab') {
			key.preventDefault();
		}
		if (key.ctrlKey && key.code === 'KeyN') {
			notes = await createNote();
		}
		if (key.ctrlKey && key.code === 'KeyS') {
			await save(note, editor);
		}
	});
});
</script>

<div class="flex h-dvh" >
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
