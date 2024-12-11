import { Node, textblockTypeInputRule } from "@tiptap/core";

export const inputRegex = /^!-\s$/;

export const HeadProps = Node.create({
	name: "HeadProps",
	group: "block",
	content: "inline*",
	defining: true,

	addOptions() {
		return {
			ghostText: "This is ghost text", // Default ghost text
		};
	},

	addAttributes() {
		return {
			ghostText: {
				default: null, // Allows setting ghost text per node instance
			},
		};
	},

	parseHTML() {
		return [
			{
				tag: "prop",
				preserveWhitespace: "full",
			},
		];
	},

	renderHTML() {
		return [
			"prop",
			["tag", { "data-type": "property", class: "text-slate-500" }, 0],
		];
	},

	addInputRules() {
		return [
			textblockTypeInputRule({
				find: inputRegex,
				type: this.type,
			}),
		];
	},
});
