import TaskItem from "@tiptap/extension-task-item";
import { formatDate } from "$lib/helpers/formater";
import { mergeAttributes, Node, wrappingInputRule } from "@tiptap/core";
import { Node as ProseMirrorNode } from "@tiptap/pm/model";

const CustomTaskItem = TaskItem.extend({
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
