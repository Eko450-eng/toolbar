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
import { FaSolidNoteSticky } from "svelte-icons-pack/fa";
import "../lib/editorapp/styles.css";
import TaskItem from "@tiptap/extension-task-item";
import { HeadProps } from "$lib/editorapp/header";
import { FaSolidX } from "svelte-icons-pack/fa";
import type { Note } from "$lib/types";
import Globalattr from "$lib/editorapp/globalattr";
import CustomTaskList from "$lib/editorapp/customtasklist";
import {
	getModalStore,
	Modal,
	type ModalSettings,
} from "@skeletonlabs/skeleton";
import Navbar from "./navbar.svelte";
import { addNote, createNote } from "$lib/notes/functions";

let note = $state<Note>({
	id: 0,
	note: "",
	folder: "",
	title: "",
});

let projects = $state<Note[]>([]);
let project = $state<string>();
let editorEditable = $state<boolean>();

let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

async function runcommand(query: string) {
	if (query.startsWith("/")) {
		try {
			//projects = await invoke("searchtext", { query });
		} catch (error) {
			console.error("Invoke error:", error);
		}
	} else if (query.startsWith(":")) {
		if (query.substring(1).trim() === "save") {
			if (editor) await addNote(note, editor);
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

onMount(async () => {
	// Create the editor
	if (!editor) setEditor();

	// Save on Ctrl+S
	window.addEventListener("keypress", async (key) => {
		if (key.ctrlKey && key.code === "KeyF") {
			modalStore.trigger(modal);
		} else if (key.shiftKey && key.code === "Tab") {
			key.preventDefault();
		}
		if (key.ctrlKey && key.code === "KeyN") {
			createNote();
		}
		if (key.ctrlKey && key.code === "KeyS") {
			if (editor) await addNote(note, editor);
		}
	});

	await invoke("createdb", {});
});

onDestroy(async () => {
	if (note.id != 0) {
		if (editor) await addNote(note, editor);
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
		if (editor) await addNote(note, editor);
	} catch (error) {
		console.error("Auto-save failed:", error);
	}
}, 2000);
</script>

<div class="flex h-screen overflow-auto">
    <Navbar bind:note bind:editor />
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
            <button class="btn w-full truncate self-end bg-red-500" onclick={() => { 
                project = ""; 
            }}>
                <Icon src={FaSolidX}/>
            </button>
            {#each projects as p}
                <li>
                    <div class="w-full flex justify-between items-center" >
                        <button class="btn truncate" onclick={() => {
                            project = p.title;
                        }}>
                            <span class="badge bg-primary-500 ">
                                <Icon src={FaSolidNoteSticky}/>
                            </span>
                            <span class="flex-auto truncate">{p.title}</span>
                        </button>
                    </div>
                </li>
            {/each}
        </ul>
    </nav>
</div>
<Modal />

