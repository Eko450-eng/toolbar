import {
	Node,
	textblockTypeInputRule,
	type KeyboardShortcutCommand,
} from "@tiptap/core";

export const inputRegex = /^!-\s$/;

export const HeadProps = Node.create({
	name: "HeadProps",
	group: "block",
	content: "inline*",
	defining: true,

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

	addKeyboardShortcuts() {
		return {
			Enter: ({ editor }) => {
				const { state, commands } = editor;
				const { $from, from, to } = state.selection;

				// Check if the current node is empty
				const isEmpty =
					$from.parent.content.lastChild &&
					$from.parent.content.lastChild.text === undefined;

				console.log(isEmpty);
				if (isEmpty) {
					commands.insertContent("<p></p>"); // Switch to normal typing mode
					return true;
				} else {
					commands.insertContent("<br>");
					return true;
				}
			},
		};
	},
});
