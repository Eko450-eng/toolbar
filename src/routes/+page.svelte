<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import EditorApp from "$lib/editorapp/+page.svelte";
import { onMount } from "svelte";
import { Editor } from "@tiptap/core";
import Color from "@tiptap/extension-color";
import StarterKit from "@tiptap/starter-kit";
import TextStyle from "@tiptap/extension-text-style";
import ListItem from "@tiptap/extension-list-item";
import { Icon } from "svelte-icons-pack";
import { FaSolidFloppyDisk } from "svelte-icons-pack/fa";

let content = $state("");
let project = $state("");
let t = $state([""]);
let id = $state("");

let element: Element | undefined = $state(undefined);
let editor: Editor | null = $state(null);

function setEditor() {
	editor = null;
	editor = new Editor({
		element: element,
		extensions: [
			Color.configure({ types: [TextStyle.name, ListItem.name] }),
			StarterKit,
		],
		content: content,
		onTransaction: () => {
			// force re-render so `editor.isActive` works as expected
			editor = editor;
		},
		editorProps: {
			attributes: {
				class: "w-full",
			},
		},
	});
}

onMount(() => {
	if (!editor) {
		setEditor();
	}
});

async function getNote(event: Event) {
	event.preventDefault();
	t = await invoke("getnote", { id });
	editor?.commands.setContent(t[0]);
	project = t[1];
}

async function createDB(event: Event) {
	event.preventDefault();
	await invoke("createdb", {});
}

async function addNote(event: Event) {
	event.preventDefault();
	try {
		const result = await invoke("addnote", {
			note: editor?.getHTML(),
			project,
		});
		console.log(result);
	} catch (error) {
		console.error("Invoke error:", error);
	}
}
</script>

<div class="container w-100 mt-4">
    <p>Title</p>
    <EditorApp bind:element bind:editor bind:project  />

    <div class="flex flex-col">
        <div class="flex justify-evenly mt-4">
            <input
              type="text"
              id="project"
              class="input"
              placeholder=""
              bind:value={id}
            />
            <button type="button" class="btn-icon variant-filled" onclick={addNote}>
                <Icon src={FaSolidFloppyDisk} />
            </button>
            <button class="btn variant-filled" onclick={getNote}>Get Note</button>
            <button class="btn variant-filled" onclick={setEditor}>Set Editor</button>
            <button class="btn variant-filled" onclick={createDB}>Create DB</button>
        </div>
    </div>
</div>
