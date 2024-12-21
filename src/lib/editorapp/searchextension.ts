import { Extension } from "@tiptap/core";

const SearchExtension = Extension.create({
	addOptions() {
		return {
			searchClass: "search-result",
			activeSearchClass: "search-result-active",
		};
	},

	addCommands() {
		return {
			setSearch:
				(decorations) =>
				({ tr, dispatch }) => {
					if (dispatch) {
						tr.setMeta("search", { decorations });
					}
					return true;
				},
			unsetSearch:
				() =>
				({ tr, dispatch }) => {
					if (dispatch) {
						tr.setMeta("search", { decorations: [] });
					}
					return true;
				},
		};
	},

	addProseMirrorPlugins() {
		const plugin = new Plugin({
			state: {
				init() {
					return DecorationSet.empty;
				},
				apply(tr, old) {
					const meta = tr.getMeta("search");
					if (meta) {
						const decorations = meta.decorations.map((d) =>
							Decoration.inline(d.from, d.to, { class: d.class }),
						);
						return DecorationSet.create(tr.doc, decorations);
					}
					return old.map(tr.mapping, tr.doc);
				},
			},
			props: {
				decorations(state) {
					return this.getState(state);
				},
			},
		});

		return [plugin];
	},
});

export default SearchExtension;
