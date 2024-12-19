<script lang="ts">
import { onDestroy, onMount } from "svelte";
import { Editor } from "@tiptap/core";
import Color from "@tiptap/extension-color";
import StarterKit from "@tiptap/starter-kit";
import TextStyle from "@tiptap/extension-text-style";
import ListItem from "@tiptap/extension-list-item";
import { Icon } from "svelte-icons-pack";
import { debounce } from "lodash-es";
import { FaSolidCheck, FaSolidRepeat } from "svelte-icons-pack/fa";
import "../lib/editorapp/styles.css";
import { HeadProps } from "$lib/editorapp/header";
import { FaSolidX } from "svelte-icons-pack/fa";
import type { Note, NoteWithTasks } from "$lib/types";
import Globalattr from "$lib/editorapp/globalattr";
import CustomTaskList from "$lib/editorapp/customtasklist";
import {
	getModalStore,
	Modal,
	type ModalSettings,
} from "@skeletonlabs/skeleton";
import Navbar from "./navbar.svelte";
import { addNote, createNote, getnotes } from "$lib/notes/functions";
import { invoke } from "@tauri-apps/api/core";
import TaskItem from "@tiptap/extension-task-item";

const initNote: Note = {
	id: -10,
	note: "",
	folder: "",
	title: "",
};

let notes = $state<Note[]>([]);
let tasks = $state<NoteWithTasks[]>([]);
let note = $state<Note>(initNote);

let editorEditable = $state<boolean>();

let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

async function setNote(id: number) {
	let n = notes.find((note) => note.id === id) ?? initNote;
	loadnote(n);
}

async function save(note: Note, editor: Editor | null) {
	if (editor) await addNote(note, editor);
	await gettask();
}

function loadnote(value: Note) {
	note = value;
	note.title = value.title;
	// if (note.id && note.id >= 0) {
	editor?.setEditable(true);
	// }
	editor?.commands.setContent("");
	editor?.commands.setContent(value.note);
	editor?.commands.focus();
}

async function runcommand(query: string) {
	if (query.startsWith("/")) {
		try {
			//projects = await invoke("searchtext", { query });
		} catch (error) {
			console.error("Invoke error:", error);
		}
	} else if (query.startsWith(":")) {
		if (query.substring(1).trim() === "save") {
			await save(note, editor);
		}
		// } else if (query.startsWith("")) {
		// } else if (query.startsWith("")) {
	}
}

const modalStore = getModalStore();

const modal: ModalSettings = {
	type: "prompt",
	value: "",
	valueAttr: { type: "text", minlength: 3, maxlength: 10, required: false },
	response: (r: string) => runcommand(r),
};

async function gettask() {
	tasks = await invoke("gettask");
}

onMount(async () => {
	if (!editor && notes.length == 0) {
		notes = await getnotes(notes, "");
		gettask();
		// Create the editor
		if (!editor) setEditor();
	}
	// Save on Ctrl+S
	window.addEventListener("keypress", async (key) => {
		if (key.ctrlKey && key.code === "KeyF") {
			modalStore.trigger(modal);
		} else if (key.shiftKey && key.code === "Tab") {
			key.preventDefault();
		}
		if (key.ctrlKey && key.code === "KeyN") {
			notes = await createNote();
		}
		if (key.ctrlKey && key.code === "KeyS") {
			await save(note, editor);
		}
	});
});

onDestroy(async () => {
	if (note.id != 0) {
		await save(note, editor);
	}
});

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
		onTransaction: () => {
			editor = editor;
		},
		onUpdate() {
			if (note.id !== 0) {
				debouncedSave();
			}
		},
		editorProps: {
			attributes: {
				class: "w-full",
			},
		},
	});
}

// Save after 2 seconds of not typing
const debouncedSave = debounce(async () => {
	try {
		await save(note, editor);
	} catch (error) {
		console.error("Auto-save failed:", error);
	}
}, 200);
</script>

<div class="flex h-screen overflow-auto" >
    <Navbar bind:notes bind:note bind:editor />
        <div class="container h-screen overflow-hidden w-100 flex flex-col gap-4 prose text-white h-screen overflow-auto">
            <label class="label mb-2">
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
            <div class="textarea h-4/5" bind:this={element}></div>
            <label class="label mb-2">
                <span>Project</span>
                <input
                    readonly={note.id && note.id > 0 ? true : false}
                    type="text"
                    id="folder"
                    class="input px-2"
                    placeholder="Note"
                    bind:value={note.folder}
                />
            </label>
        </div>

    <nav class="list-nav">
        <ul>
            <button class="btn w-full truncate self-end" onclick={gettask}>
                <Icon src={FaSolidRepeat}/>
            </button>
            {#each tasks as note}
                <p>{note.title}</p> 
                {#each note.tasks as p}
                    <li>
                        <button class={`btn variant-filled-secondary w-full truncate self-end text-black ${p.checked ? "border-green-200" : "border-red-500"}`} onclick={()=>setNote(note.note_id)}>
                            <div class="w-full flex justify-between items-center" >
                                <span class="flex gap-2 truncate">
                                    {#if p.checked}
                                        <Icon src={FaSolidCheck} color="green"/>
                                        {:else}
                                        <Icon src={FaSolidX} color="red"/>
                                    {/if}
                                    {p.label}
                                </span>
                            </div>
                        </button>
                    </li>
                {/each}
            {/each}
        </ul>
    </nav>
</div>
<Modal />

