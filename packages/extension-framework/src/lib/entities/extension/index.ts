// @ts-ignore-next-line
import { importSvelteBundle } from "svelte-browser-import"
import { createRxDatabase } from "rxdb"
import { getRxStorageDexie } from "rxdb/plugins/storage-dexie"

import { type ExtensionInputs, setExtensionContextState } from "./model"

const tagName = "akaia-app"

const db = await createRxDatabase({
	name: tagName,
	storage: getRxStorageDexie(),
})

export const install = async () => {
	const extensionBundle = await importSvelteBundle({
		// files: [
		// 	// only one of urls or files can be provided
		// 	{
		// 		name: "App",
		// 		type: "svelte",
		// 		modified: true,

		// 		content: `
		// 			<script>
		// 				import { onMount } from "svelte"

		// 				onMount(() => {
		// 					console.log("test")
		// 				})
		// 				</script>

		// 				<p>Example CommLink extension written in Svelte</p>
		// 		`,
		// 	},
		// ],

		urls: ["/static/packages/extensions/haptic/routes/+layout.svelte" /* "./Nested.svelte" */],
		packagesUrl: "https://ga.jspm.io",
		svelteUrl: "https://ga.jspm.io/npm:svelte@5.1.3",
		injectedJS: "",
		injectedCSS: "",

		onstatus: (val: unknown) => {
			console.log("onStatus", val)
		},
	})

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
					console.log("connected")

					const Extension = new extensionBundle.render()

					console.log("Extension", Extension)

					console.log("attachShadow result:", this.attachShadow({ mode: "open" }))

					const extensionInstance = new Extension({
						target: this.attachShadow({ mode: "open" }),
					})

					console.log("extensionInstance", extensionInstance)
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
