<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";
import { Editor } from "@tiptap/core";
import Color from "@tiptap/extension-color";
import StarterKit from "@tiptap/starter-kit";
import TextStyle from "@tiptap/extension-text-style";
import ListItem from "@tiptap/extension-list-item";
import { Icon } from "svelte-icons-pack";
import { debounce } from "lodash-es";
import {
	FaSolidFloppyDisk,
	FaSolidNoteSticky,
	FaSolidPlus,
	FaSolidRepeat,
	FaSolidTrash,
} from "svelte-icons-pack/fa";
import { all, createLowlight } from "lowlight";
import CodeBlockLowlight from "@tiptap/extension-code-block-lowlight";
import "../lib/editorapp/styles.css";
import TaskList from "@tiptap/extension-task-list";
import TaskItem from "@tiptap/extension-task-item";
import { HeadProps } from "$lib/editorapp/header";

type Note = {
	id?: number;
	created_at?: Date;
	title: string;
	note: string;
	project: string;
};

let note = $state<Note>({
	id: 0,
	note: "",
	project: "",
	title: "",
});

const CustomTaskList = TaskList.extend({
	addKeyboardShortcuts() {
		return {
			"Mod-shift-t": () => this.editor.commands.toggleTaskList(),
		};
	},
});

let notes = $state<Note[]>([]);
let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

function setEditor() {
	const lowlight = createLowlight(all);

	editor = null;
	editor = new Editor({
		element: element,
		extensions: [
			CustomTaskList,
			TaskItem,
			HeadProps,
			CodeBlockLowlight.configure({
				lowlight,
			}),
			Color.configure({ types: [TextStyle.name, ListItem.name] }),
			StarterKit.configure({
				codeBlock: false,
			}),
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
	notes = await invoke("getnote", {});
});

async function createNote() {
	try {
		const result = await invoke("createnote", { note });
		notes = await invoke("getnote", {});
		console.log(result);
	} catch (error) {
		console.error("Invoke error:", error);
	}
}

async function addNote() {
	try {
		note.note = editor?.getHTML() ?? "";
		await invoke("addnote", { note });
		notes = await invoke("getnote", {});
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
                    <button type="button" class="btn-icon variant-filled" onclick={setEditor}>
                        <Icon src={FaSolidRepeat} />
                    </button>
                </div>
            </div>
        </nav>
    {/if}

        <div class="container h-screen overflow-hidden w-100 flex flex-col gap-4 prose text-white h-screen overflow-auto">
        <label class="label mb-2">
            <span>Title</span>
            <input
              type="text"
              id="title"
              class="input px-2"
              placeholder=""
              bind:value={note.title}
            />
        </label>
            <div class="textarea h-4/5" bind:this={element}></div>
            <label class="label">
                <span>Project</span>
                <input
                  type="text"
                  id="project"
                  class="input px-2"
                  placeholder=""
                  bind:value={note.project}
                />
            </label>
        </div>
</div>

