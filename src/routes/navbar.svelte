<script lang="ts">
import type { Note } from '$lib/types';
import { Editor } from '@tiptap/core';
import { Icon } from 'svelte-icons-pack';
import { FaSolidNoteSticky, FaSolidTrash } from 'svelte-icons-pack/fa';
import { addNote, deleteNote } from '$lib/notes/functions';

let {
	note = $bindable(),
	notes = $bindable(),
	editor = $bindable(),
}: { note: Note; notes: Note[]; editor: Editor | null } = $props();

async function save(note: Note, editor: Editor | null) {
	if (editor) await addNote(note, editor);
}

function loadnote(value: Note) {
	save(note, editor);
	note = value;
	note.title = value.title;
	// if (note.id && note.id >= 0) {
	editor?.setEditable(true);
	// }
	editor?.commands.setContent('');
	editor?.commands.setContent(value.note);
	editor?.commands.focus();
}
</script>

{#if notes}
    <nav class="flex flex-col list-nav justify-between  w-1/5 max-w-1/5 min-w-1/5">
        <ul>
            {#each notes as note}
                <li class="w-full flex justify-between items-center">
                        <div class="w-full flex justify-between items-center" >
                            <button class="btn truncate" onclick={()=>loadnote(note)}>
                                <span class="badge variant-filled-surface">
                                    <Icon src={FaSolidNoteSticky}/>
                                </span>
                                <span class="flex-auto truncate">{note.title}</span>
                            </button>
                            <button class="btn-icon-s variant-filled-error" onclick={async()=>notes = await deleteNote(note.id ?? 0)}>
                                <Icon src={FaSolidTrash}/>
                            </button>
                        </div>
                </li>
            {/each}
        </ul>
    </nav>
{/if}
