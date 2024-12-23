<script lang="ts">
import { initNote, type Note, type NoteWithTasks } from '$lib/types';
import type { Editor } from '@tiptap/core';
import { Icon } from 'svelte-icons-pack';
import { FaSolidCheck, FaSolidX } from 'svelte-icons-pack/fa';

let {
	editor = $bindable(),
	tasks = $bindable(),
	notes = $bindable(),
	note = $bindable(),
}: {
	editor: Editor | null;
	tasks: NoteWithTasks[];
	notes: Note[];
	note: Note;
} = $props();

function setNote(id: number) {
	let value = notes.find((note) => note.id === id) ?? initNote;
	note = value;
	note.title = value.title;
	// if (note.id && note.id >= 0) {
	editor?.setEditable(true);
	// }
	editor?.commands.setContent('');
	editor?.commands.setContent(value.note);
	editor?.commands.focus();
}

let taskFilter = $state<boolean>(false);
</script>

<nav class="list-nav h-full overflow-y-scroll w-1/6" >
    <ul>
        <button class={`btn w-full truncate gap-2 self-end ${taskFilter ? "variant-filled" : "variant-ghost"}`} onclick={()=>taskFilter = !taskFilter}>
            Nur offene Anzeigen
            <Icon src={FaSolidCheck} color={taskFilter ? "green" : "red"}/>
        </button>
        {#each tasks as note}
            <p>{note.title}</p> 
            {#each note.tasks as p}
                {#if (taskFilter && !p.checked) || !taskFilter }
                    <li>
                        <button class={`btn w-full truncate self-end ${p.checked ? "border-green-200" : "border-red-500"}`} onclick={()=>setNote(note.note_id)}>
                            <div class="w-full flex gap-2 justify-between items-center" >
                                {#if p.checked}
                                    <Icon src={FaSolidCheck} size="16" color="green"/>
                                    {:else}
                                    <Icon src={FaSolidX} size="16" color="red"/>
                                {/if}
                                <span class="truncate">
                                    {p.label}
                                </span>
                            </div>
                        </button>
                    </li>
                {/if}
            {/each}
        {/each}
    </ul>
</nav>
