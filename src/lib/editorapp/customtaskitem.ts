import TaskItem from "@tiptap/extension-task-item";
import { formatDate } from "$lib/helpers/formater";
import { mergeAttributes, Node, wrappingInputRule } from "@tiptap/core";
import { Node as ProseMirrorNode } from "@tiptap/pm/model";

const CustomTaskItem = TaskItem.extend({
	addKeyboardShortcuts() {
		return {
			"Mod-t": () => {
				// Get the current position
				const { $from } = this.editor.state.selection;

				// Find the task item node by walking up the tree
				const taskItem = $from.node($from.depth);
				console.log("hier", taskItem);

				if (taskItem?.type.name === "taskItem") {
					this.editor
						.chain()
						.focus()
						.command(({ tr }) => {
							tr.setNodeMarkup($from.before($from.depth), null, {
								checked: !taskItem.attrs.checked,
							});
							return true;
						})
						.run();
					return true;
				}
			},
		};
	},
	renderHTML({ node, HTMLAttributes }) {
		return [
			"li",
			mergeAttributes(this.options.HTMLAttributes, HTMLAttributes, {
				"data-type": this.name,
			}),
			[
				"label",
				[
					"input",
					{
						type: "checkbox",
						checked: node.attrs.checked ? "checked" : null,
					},
				],
			],
			["div", 0],
		];
	},
});

export default CustomTaskItem;
