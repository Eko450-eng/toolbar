import TaskList from "@tiptap/extension-task-list";
import { formatDate } from "$lib/helpers/formater";

const CustomTaskList = TaskList.extend({
	addKeyboardShortcuts() {
		return {
			"Mod-shift-t": () => this.editor.commands.toggleTaskList(),
			Enter: () => {
				const { state, view } = this.editor;
				const { tr, doc } = state;
				const currentPos = state.selection.$from.pos;

				// Find the node at the cursor position
				doc.nodesBetween(currentPos, currentPos, (node, pos) => {
					if (node.type.name !== "text" && node.content.size > 0) {
						console.log(node.content);
						// Ensure it's not a text node
						if (node.content.size > 0) {
							tr.setNodeMarkup(pos, null, {
								...node.attrs,
								timeStamp: formatDate(new Date()), // Set the timeStamp
							});
						} else {
							tr.setNodeMarkup(pos, null, {
								...node.attrs,
								timeStamp: null, // Set the timeStamp
							});
						}
					}
				});

				// Dispatch the transaction
				view.dispatch(tr);
				return false;
			},
		};
	},
});

export default CustomTaskList;
