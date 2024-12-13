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
	Modal,
	type ModalSettings,
} from "@skeletonlabs/skeleton";

let note = $state<Note>({
	id: 0,
	note: "",
	folder: "",
	title: "",
});

let projects = $state<Note[]>([]);
let project = $state<string>();
let editorEditable = $state<boolean>();

let notes = $state<Note[]>([]);
let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

async function searchtext(query: string) {
	try {
		projects = await invoke("searchtext", { query });
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

const modalStore = getModalStore();

const modal: ModalSettings = {
	type: "prompt",
	// Data
	// Populates the input value and attributes
	value: "",
	valueAttr: { type: "text", minlength: 3, maxlength: 10, required: false },
	// Returns the updated response value
	response: (r: string) => searchtext(r),
};

onMount(async () => {
	// Create the editor
	if (!editor) setEditor();

	// Save on Ctrl+S
	window.addEventListener("keypress", (key) => {
		window.addEventListener("keypress", (key) => {
			if (key.ctrlKey && key.code === "KeyF") {
				modalStore.trigger(modal);
			}
		});
		if (key.shiftKey && key.code === "Tab") {
			console.log("none");
			key.preventDefault();
		}
		if (key.ctrlKey && key.code === "KeyN") {
			createNote();
		}
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

// Save after 2 seconds of not typing
const debouncedSave = debounce(async () => {
	try {
		await addNote();
	} catch (error) {
		console.error("Auto-save failed:", error);
	}
}, 2000);

async function getnotes() {
	if (notes.length <= 0) notes = await invoke("getnote", { project });
	notes = await invoke("getnote", { project });
	// await getprojects();
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
		// getprojects();
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

<div class="flex h-screen overflow-auto">
    {#if notes}
        <nav class="flex flex-col list-nav justify-between  w-1/5 max-w-1/5 min-w-1/5">
            <ul>
                {#each notes as note}
                    <li class="w-full flex justify-between items-center">
                            <div class="w-full flex justify-between items-center" >
                                <button class="btn truncate" onclick={()=>loadnote(note)}>
                                    <span class="badge bg-primary-500 ">
                                        <Icon src={FaSolidNoteSticky}/>
                                    </span>
                                    <span class="flex-auto truncate">{note.title}</span>
                                </button>
                                <button class="btn-icon-s bg-red-500" onclick={()=>deleteNote(note.id ?? 0)}>
                                    <Icon src={FaSolidTrash}/>
                                </button>
                            </div>
                    </li>
                {/each}
            </ul>
            <div class="flex flex-col">
                <div class="flex justify-evenly mb-10">
                    <button type="button" class="btn-icon variant-filled" onclick={createNote}>
                        <Icon src={FaSolidPlus} />
                    </button>
                    <button type="button" class="btn-icon variant-filled" onclick={addNote}>
                        <Icon src={FaSolidFloppyDisk} />
                    </button>
                    <button type="button" class="btn-icon variant-filled" onclick={toggleMode}>
                        <Icon src={FaSolidMoon} />
                    </button>
                </div>
            </div>
        </nav>
    {/if}

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
                getnotes(); 
            }}>
                <Icon src={FaSolidX}/>
            </button>
            {#each projects as p}
                <li>
                    <div class="w-full flex justify-between items-center" >
                        <button class="btn truncate" onclick={() => {
                            project = p.title;
                            getnotes();
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

