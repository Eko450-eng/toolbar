import { Extension } from "@tiptap/core";

const GlobalAttr = Extension.create({
	name: "globalAttr",
	addGlobalAttributes() {
		return [
			{
				types: [
					"paragraph",
					"taskList",
					"taskItem",
					"HeadProps",
					"blockquote",
					"bulletList",
					"codeBlock",
					"doc",
					"hardBreak",
					"heading",
					"horizontalRule",
					"listItem",
					"orderedList",
				],
				attributes: {
					timeStamp: {
						default: null, // Default value
						parseHTML: (element) => element.getAttribute("timeStamp"),
						renderHTML: (attributes) => {
							if (!attributes.timeStamp) {
								return {};
							}
							return {
								timeStamp: attributes.timeStamp,
							};
						},
					},
				},
			},
		];
	},
});

export default GlobalAttr;
