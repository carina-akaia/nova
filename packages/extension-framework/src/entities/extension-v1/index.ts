// @ts-ignore-next-line
import { importSvelteBundle } from "svelte-browser-import"
import { createRxDatabase } from "rxdb"
import { getRxStorageDexie } from "rxdb/plugins/storage-dexie"

import {
	type ExtensionInputs,
	extensionContextState,
	ExtensionContext,
	setExtensionContextState,
} from "./model"

export { useExtensionContext } from "./model"

const tagName = "akaia-app"

const db = await createRxDatabase({
	name: tagName,
	storage: getRxStorageDexie(),
})

export const install = async () => {
	const res = await importSvelteBundle({
		urls: ["./example.svelte" /* "./Nested.svelte" */],
		// files: [ // only one of urls or files can be provided
		//     {
		//         name: 'App',
		//         type: 'svelte',
		//         content: '...',
		//         modified: true,
		//     }
		// ],
		packagesUrl: "https://unpkg.com",
		svelteUrl: "https://unpkg.com/svelte",
		injectedJS: "",
		injectedCSS: "",

		onstatus: (val) => {
			console.log(val)
		},
	})

	console.log(res)

	const ExtensionInstance = new res.render()

	if (customElements.get(tagName) === undefined) {
		customElements.define(
			tagName,

			class AkaiaExtension extends HTMLElement {
				static get observedAttributes(): (keyof ExtensionInputs)[] {
					return ["account_id", "app_id", "route_path", "route_query", "props"]
				}

				attributeChangedCallback(name: keyof ExtensionInputs, oldValue: string, newValue: string) {
					if (oldValue !== newValue) {
						setExtensionContextState((props) => ({
							...props,
							[name]: name === "route_query" || name === "props" ? JSON.parse(newValue) : newValue,
						}))
					}
				}

				connectedCallback() {
					const _extension = new ExtensionInstance({
						target: this.attachShadow({ mode: "closed" }),
					})
				}
			},
		)
	}
}

declare global {
	namespace JSX {
		export interface IntrinsicElements {
			[tagName]: ExtensionInputs
		}
	}
}
