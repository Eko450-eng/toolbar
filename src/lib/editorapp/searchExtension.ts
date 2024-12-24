import { Extension } from '@tiptap/core';
import { Plugin, PluginKey } from 'prosemirror-state';
import { Decoration, DecorationSet } from 'prosemirror-view';

interface SearchOptions {
	searchTerm: string;
	caseSensitive: boolean;
	highlightClass: string;
}

declare module '@tiptap/core' {
	interface Commands<ReturnType> {
		search: {
			setSearchTerm: (searchTerm: string) => ReturnType;
			setCaseSensitive: (caseSensitive: boolean) => ReturnType;
			clearSearch: () => ReturnType;
		};
	}
}

export const SearchExtension = Extension.create<SearchOptions>({
	name: 'search',

	addOptions() {
		return {
			searchTerm: '',
			caseSensitive: false,
			highlightClass: 'search-highlight',
		};
	},

	addStorage() {
		return {
			searchResults: [],
			currentResult: -1,
		};
	},

	addCommands() {
		return {
			setSearchTerm:
				(searchTerm: string) =>
				({ editor }) => {
					editor.storage.search.searchResults = [];
					editor.storage.search.currentResult = -1;
					this.options.searchTerm = searchTerm;
					editor.view.dispatch(editor.state.tr);
					return true;
				},

			setCaseSensitive:
				(caseSensitive: boolean) =>
				({ editor }) => {
					this.options.caseSensitive = caseSensitive;
					editor.view.dispatch(editor.state.tr);
					return true;
				},

			clearSearch:
				() =>
				({ editor }) => {
					this.options.searchTerm = '';
					editor.storage.search.searchResults = [];
					editor.storage.search.currentResult = -1;
					editor.view.dispatch(editor.state.tr);
					return true;
				},
		};
	},

	addProseMirrorPlugins() {
		const key = new PluginKey('search');

		return [
			new Plugin({
				key,
				state: {
					init() {
						return DecorationSet.empty;
					},
					apply: (tr, oldState) => {
						const searchTerm = this.options.searchTerm;
						if (!searchTerm) return DecorationSet.empty;

						const { doc } = tr;
						const decorations: Decoration[] = [];
						const searchResults: number[] = [];

						const search = (text: string, pos: number) => {
							const searchString = this.options.caseSensitive
								? searchTerm
								: searchTerm.toLowerCase();
							const content = this.options.caseSensitive
								? text
								: text.toLowerCase();
							let index = 0;
							while (index !== -1) {
								index = content.indexOf(searchString, index);
								if (index !== -1) {
									const from = pos + index;
									const to = from + searchTerm.length;
									const decoration = Decoration.inline(from, to, {
										class: this.options.highlightClass,
									});
									decorations.push(decoration);
									searchResults.push(from);
									index += 1;
								}
							}
						};

						doc.descendants((node, pos) => {
							if (node.isText) {
								search(node.text || '', pos);
							}
						});

						// Update storage with search results
						this.editor.storage.search.searchResults = searchResults;

						return DecorationSet.create(doc, decorations);
					},
				},
				props: {
					decorations(state) {
						return this.getState(state);
					},
				},
			}),
		];
	},
});
