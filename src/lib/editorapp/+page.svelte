<script lang="ts">
import "./styles.scss";
import { Editor } from "@tiptap/core";

let {
	editor = $bindable(),
	element = $bindable(),
	project = $bindable(),
} = $props<{
	element: Element | undefined;
	editor: Editor | null;
	project: string;
}>();
</script>

<div class="flex flex-col gap-4 prose text-white">
    <div class="textarea" bind:this={element}></div>

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
