<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import type { Editor } from '@tiptap/core';

let {
	editor = $bindable(),
	element = $bindable(),
	note = $bindable(),
}: {
	editor: Editor | null;
	element: Element | undefined;
	note: string;
} = $props();

let prompt = $state<string>('');
let response = $state<string>('');

let models = [
	'gpt-4o-mini',
	'neural-chat:latest',
	'llama3.3',
	'claude-3-haiku-20240307',
];

let model = $state('gpt-4o-mini');

async function promptai() {
	response = await invoke('promptai', {
		input: prompt,
		note: note,
		model: model ?? models[0],
	});
	editor?.commands.setContent(note + '<br/>' + response);
}
</script>

<div class="textarea p-4 h-4/5" bind:this={element}></div>
<form onsubmit={promptai} class="flex">
    <label class="label w-4/5">
        <span>Frag die AI</span>
        {#each models as modelname} 
            <span>{modelname}, </span> 
        {/each} 
        <input
            type="text"
            id="prompt"
            class="input px-2"
            placeholder="Fass mir das hier zusammen"
            bind:value={prompt}
        />
        <!-- <input -->
        <!--     type="text"  -->
        <!--     id="model"  -->
        <!--     class="input px-2"  -->
        <!--     placeholder={models[0]}  -->
        <!--     bind:value={model}  -->
        <!-- />  -->
        <label class="label">
            <span>Model</span>
            <select class="select">
                {#each models as model }
                    <option value={model}>{model}</option>
                {/each}
            </select>
        </label>
    </label>
    <button type="submit" class="btn truncate self-end">
        Send
    </button>
</form>
