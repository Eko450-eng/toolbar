import { Extension } from "@tiptap/core";

const GlobalAttr = Extension.create({
	name: "globalAttr",
	addGlobalAttributes() {
		return [
			{
				types: [
					"paragraph",
					"taskItem",
					"HeadProps",
					"blockquote",
					"codeBlock",
					"doc",
					"heading",
				],
				attributes: {
					timeStamp: {
						default: null,
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
