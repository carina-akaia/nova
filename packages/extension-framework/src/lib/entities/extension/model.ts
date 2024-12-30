import { writable, type Writable } from "svelte/store"

export type ExtensionInputs = {
	account_id: string
	app_id: string
	route_path: string
	route_query: string
	props: string
}

export type ExtensionContextState = {
	account_id: null | ExtensionInputs["account_id"]
	app_id: null | ExtensionInputs["app_id"]
	route_path: null | string
	route_query: null | Record<string, undefined | null | boolean | string | object>
	props: null | object
}

const initialExtensionContext: ExtensionContextState = {
	account_id: null,
	app_id: null,
	route_path: null,
	route_query: null,
	props: null,
}

export const extensionContextStore: Writable<ExtensionContextState> =
	writable(initialExtensionContext)

export const setExtensionContextState = (
	updater: (state: ExtensionContextState) => Partial<ExtensionContextState>,
) => {
	extensionContextStore.update((state: ExtensionContextState) => ({
		...state,
		...updater(state),
	}))
}
