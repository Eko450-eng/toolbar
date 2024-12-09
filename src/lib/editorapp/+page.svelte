<script lang="ts">
import { Editor } from "@tiptap/core";

let { element = $bindable(), project = $bindable() } = $props<{
	element: Element | undefined;
	editor: Editor | null;
	project: string;
}>();
</script>

<div class="flex flex-col gap-4 prose text-white h-screen">
    <div class="textarea h-3/5 overflow-auto" bind:this={element}></div>

    <label class="label">
        <span>Project</span>
        <input
          type="text"
          id="project"
          class="input"
          placeholder=""
          bind:value={project}
        />
    </label>
</div>
