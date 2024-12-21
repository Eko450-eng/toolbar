<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onDestroy, onMount } from "svelte";
import { Editor } from "@tiptap/core";
import Color from "@tiptap/extension-color";
import StarterKit from "@tiptap/starter-kit";
import TextStyle from "@tiptap/extension-text-style";
import ListItem from "@tiptap/extension-list-item";
import { Icon } from "svelte-icons-pack";
import { debounce } from "lodash-es";
import {
	FaSolidFloppyDisk,
	FaSolidMoon,
	FaSolidNoteSticky,
	FaSolidPlus,
	FaSolidTrash,
} from "svelte-icons-pack/fa";
import "../lib/editorapp/styles.css";
import TaskItem from "@tiptap/extension-task-item";
import { HeadProps } from "$lib/editorapp/header";
import { FaSolidX } from "svelte-icons-pack/fa";
import { toggleMode } from "mode-watcher";
import { type Note } from "$lib/types";
import Globalattr from "$lib/editorapp/globalattr";
import CustomTaskList from "$lib/editorapp/customtasklist";
import {
	getModalStore,
	getToastStore,
	Modal,
	Toast,
	type ModalSettings,
	type ToastStore,
} from "@skeletonlabs/skeleton";
import Navbar from "./navbar.svelte";
import { addNote, createNote, getnotes } from "$lib/notes/functions";
import { invoke } from "@tauri-apps/api/core";
import TaskItem from "@tiptap/extension-task-item";

let note = $state<Note>({
	id: 0,
	note: "",
	folder: "",
	title: "",
});

let projects = $state<string[]>([]);
let project = $state<string>();
let editorEditable = $state<boolean>();

let notes = $state<Note[]>([]);
let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

const modalStore = getModalStore();
const toastStore = getToastStore();

const t = {
	message: "Saved",
};

const modal: ModalSettings = {
	type: "prompt",
	value: "",
	valueAttr: { type: "text", minlength: 3, maxlength: 10, required: false },
	response: (r: string) => runcommand(r),
};

async function setNote(id: number) {
	let n = notes.find((note) => note.id === id) ?? initNote;
	loadnote(n);
}

async function save(note: Note, editor: Editor | null, noToast?: boolean) {
	if (editor) await addNote(note, editor);
	if (!noToast) toastStore.trigger(t);
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

async function gettask() {
	tasks = await invoke("gettask");
}

onMount(async () => {
	// Create the editor
	if (!editor) setEditor();

	// Save on Ctrl+S
	window.addEventListener("keypress", (key) => {
		if (key.ctrlKey && key.code === "KeyS") {
			addNote();
		}
	});

	await invoke("createdb", {});
	getnotes();
});

onDestroy(async () => {
	if (note.id != 0) {
		await addNote();
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
			// force re-render so `editor.isActive` works as expected
			editor = editor;
		},
		onUpdate() {
			if (note.id != 0) {
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

const debouncedSave = debounce(async () => {
	try {
		await save(note, editor, true);
	} catch (error) {
		console.error("Auto-save failed:", error);
	}
}, 2000);

async function getnotes() {
	if (notes.length <= 0) notes = await invoke("getnote", { project });
	notes = await invoke("getnote", { project });
	await getprojects();
}

async function getprojects() {
	try {
		projects = await invoke("getprojects", { note });
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

async function createNote() {
	try {
		await invoke("createnote", {});
		notes = await invoke("getnote", {});
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

async function addNote() {
	try {
		note.note = editor?.getHTML() ?? "";
		await invoke("addnote", { note });
		notes = await invoke("getnote", {});
		getprojects();
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

async function deleteNote(id: number) {
	try {
		await invoke("deletenote", { id });
		notes = await invoke("getnote", {});
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

function loadnote(value: Note) {
	editorEditable = note.id && note.id > 0 ? true : false;
	note = value;
	if (note.id && note.id > 0) {
		editor?.setEditable(true);
	}
	editor?.commands.setContent(value.note);
	editor?.commands.focus();
}
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
        <div class="textarea h-4/5" bind:this={element}></div>
        <label class="label">
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

    <nav class="list-nav" >
        <ul>
            <button class="btn w-full truncate self-end bg-red-500" onclick={() => { 
                project = ""; 
                getnotes(); 
            }}>
                <Icon src={FaSolidX}/>
            </button>
            {#each projects as p}
                <li>
                    <div class="w-full flex justify-between items-center" >
                        <button class="btn truncate" onclick={() => {
                            project = p;
                            getnotes();
                        }}>
                            <span class="badge bg-primary-500 ">
                                <Icon src={FaSolidNoteSticky}/>
                            </span>
                            <span class="flex-auto truncate">{p}</span>
                        </button>
                    </div>
                </li>
            {/each}
        </ul>
    </nav>
</div>
<Modal />
<Toast position="br" />
