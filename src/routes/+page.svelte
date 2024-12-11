<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onDestroy, onMount } from "svelte";
import { Editor, Extension } from "@tiptap/core";
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
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import { HeadProps } from "$lib/editorapp/header";
import { FaSolidX } from "svelte-icons-pack/fa";
import { toggleMode } from "mode-watcher";

type Note = {
	id?: number;
	created_at?: Date;
	title: string;
	note: string;
	folder: string;
};

let note = $state<Note>({
	id: 0,
	note: "",
	folder: "",
	title: "",
});

let projects = $state<string[]>([]);
let project = $state<string>();
let editorEditable = $state<boolean>(note.id && note.id > 0 ? true : false);

function formatDate(rawDate: Date) {
	const date = new Date(rawDate);
	const formattedDate = new Intl.DateTimeFormat("de-DE", {
		year: "numeric",
		month: "long",
		day: "numeric",
		hour: "numeric",
		minute: "numeric",
		second: "numeric",
	}).format(date);
	return formattedDate;
}

const CustomTaskList = TaskList.extend({
	addKeyboardShortcuts() {
		return {
			"Mod-shift-t": () => this.editor.commands.toggleTaskList(),
			Enter: () => {
				const { state, view } = this.editor;
				const { tr, doc } = state;
				const currentPos = state.selection.$from.pos;

				// Find the node at the cursor position
				doc.nodesBetween(currentPos, currentPos, (node, pos) => {
					if (node.type.name !== "text") {
						// Ensure it's not a text node
						tr.setNodeMarkup(pos, null, {
							...node.attrs,
							timeStamp: formatDate(new Date()), // Set the timeStamp
						});
					}
				});

				// Dispatch the transaction
				view.dispatch(tr);
				return false;
			},
		};
	},
});

let notes = $state<Note[]>([]);
let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

onDestroy(async () => {
	await addNote();
});

const GlobalAttr = Extension.create({
	name: "globalAttr",
	addGlobalAttributes() {
		return [
			{
				types: [
					"paragraph",
					"taskList",
					"taskItem",
					"HeadProps",
					"blockquote",
					"bulletList",
					"codeBlock",
					"doc",
					"hardBreak",
					"heading",
					"horizontalRule",
					"listItem",
					"orderedList",
				],
				attributes: {
					timeStamp: {
						default: "", // Default value
						parseHTML: (element) => element.getAttribute("timeStamp"),
						renderHTML: (attributes) => {
							if (!attributes.timeStamp) {
								return {};
							}
							return {
								timeStamp: attributes.timeStamp,
							};
						},
					},
				},
			},
		];
	},
});

function setEditor() {
	editor = null;
	editor = new Editor({
		editable: false,
		element: element,
		extensions: [
			GlobalAttr,
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
		await addNote();
	} catch (error) {
		console.error("Auto-save failed:", error);
	}
}, 2000); // 2 seconds after user stops typing

onMount(async () => {
	if (!editor) {
		setEditor();
	}
	await invoke("createdb", {});
	getnotes();
});

async function getnotes() {
	if (notes.length <= 0) notes = await invoke("getnote", { project });
	notes = await invoke("getnote", { project });
	await getprojects();
}

async function createNote() {
	try {
		await invoke("createnote", { note });
		notes = await invoke("getnote", {});
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

async function getprojects() {
	try {
		projects = await invoke("getprojects", { note });
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

async function addNote() {
	console.log(note.note);
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

