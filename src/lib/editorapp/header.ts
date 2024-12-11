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

	// addNodeView() {
	// 	return ({ node }) => {
	// 		const container = document.createElement("div");
	// 		container.setAttribute("data-type", "ghost-text-node");
	// 		container.style.position = "relative";
	//
	// 		// Actual content
	// 		const content = document.createElement("div");
	// 		content.contentEditable = "true";
	// 		content.style.minHeight = "1em"; // Ensure space for content
	// 		container.appendChild(content);
	//
	// 		// Ghost text
	//
	// 		// Update visibility of ghost text based on content
	// 		const updateGhostVisibility = () => {};
	//
	// 		// Initial visibility check
	// 		updateGhostVisibility();
	//
	// 		// Listen to changes in the node's content
	// 		const observer = new MutationObserver(updateGhostVisibility);
	// 		observer.observe(content, {
	// 			childList: true,
	// 			characterData: true,
	// 			subtree: true,
	// 		});
	//
	// 		// Cleanup when node is removed
	// 		return {
	// 			dom: container,
	// 			contentDOM: content,
	// 			destroy() {
	// 				observer.disconnect();
	// 			},
	// 		};
	// 	};
	// },

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

	// addKeyboardShortcuts() {
	// 	return {
	// 		Enter: ({ editor }) => {
	// 			const { commands } = editor;
	// 			let now = new Date(Date.now()).toLocaleString();
	// 			const ghostText = document.createElement("span");
	// 			ghostText.textContent = now;
	// 			ghostText.style.pointerEvents = "none"; // Prevent interaction
	// 			ghostText.style.color = "#aaa"; // Style as desired
	// 			ghostText.style.opacity = "0.5";
	// 			ghostText.style.position = "absolute";
	// 			ghostText.style.zIndex = "1";
	// 			ghostText.style.display = "inline-block";
	// 			console.log(ghostText);
	//
	//
	//
	// 			// commands.insertContent(ghostText.getHTML()); // Switch to normal typing mode
	// 			// commands.insertContent(
	// 			// 	`<span style="pointer-events: none; color: rgb(170, 170, 170); opacity: 0.5; position: absolute; z-index: 1; display: inline-block;">${now}</span>`,
	// 			// );
	// 			// commands.insertContent("<br>"); // Switch to normal typing mode
	// 			return false;
	// 		},
	// 	};
	// },
});
