<script lang="ts">
import type { Editor } from '@tiptap/core';
import { Search, ArrowUp, ArrowDown, X } from 'lucide-svelte';
import { onMount } from 'svelte';

export let editor: Editor;

let searchTerm = '';
let caseSensitive = false;

$: resultCount = editor?.storage?.search?.searchResults?.length || 0;
$: currentResult = editor?.storage?.search?.currentResult || -1;

function updateSearch(value: string) {
	searchTerm = value;
	editor.commands.setSearchTerm(value);
}

function toggleCaseSensitive() {
	caseSensitive = !caseSensitive;
	editor.commands.setCaseSensitive(caseSensitive);
}

function clearSearch() {
	searchTerm = '';
	editor.commands.clearSearch();
}

function navigateResults(direction: 'prev' | 'next') {
	const results = editor.storage.search.searchResults;
	if (!results.length) return;

	let newIndex = currentResult;
	if (direction === 'next') {
		newIndex = currentResult + 1 >= results.length ? 0 : currentResult + 1;
	} else {
		newIndex = currentResult - 1 < 0 ? results.length - 1 : currentResult - 1;
	}

	editor.storage.search.currentResult = newIndex;
	const pos = results[newIndex];
	editor.commands.setTextSelection(pos);
	editor.view.dispatch(editor.state.tr);
}

onMount(() => {
	document.getElementById('searchField')?.focus();
});
</script>

<div class="flex absolute z-10 w-50% right-5 top-5 items-center gap-2 p-2 h-min backdrop-blur shadow-sm">
  <div class="relative flex-1">
    <div class="absolute left-2 top-1/2 -translate-y-1/2 text-gray-400">
      <Search size={18} />
    </div>
    <input
      type="text"
      id="searchField"
      bind:value={searchTerm}
      on:input={(e) => updateSearch(e.currentTarget.value)}
      placeholder="Search in text..."
      class="w-full pl-8 pr-4 py-1 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>
</div>

<style>
  :global(.search-highlight) {
    background-color: rgba(255, 213, 0, 0.2);
  }
</style>
