import TaskList from "@tiptap/extension-task-list";
import { formatDate } from "$lib/helpers/formater";

const CustomTaskList = TaskList.extend({
	addKeyboardShortcuts() {
		return {
			"Shift-Tab": () => false,
			Tab: () => false,
			"Mod-shift-t": () => this.editor.commands.toggleTaskList(),
			Enter: () => {
				const { state, view } = this.editor;
				const { tr, doc } = state;
				const currentPos = state.selection.$from.pos;

				doc.nodesBetween(currentPos, currentPos, (node, pos) => {
					if (node.type.name !== "text" && node.content.size > 0) {
						console.log(node.content);
						if (node.content.size > 0) {
							tr.setNodeMarkup(pos, null, {
								...node.attrs,
								timeStamp: formatDate(new Date()),
							});
						} else {
							tr.setNodeMarkup(pos, null, {
								...node.attrs,
								timeStamp: null,
							});
						}
					}
				});

				view.dispatch(tr);
				return false;
			},
		};
	},
});

export default CustomTaskList;
