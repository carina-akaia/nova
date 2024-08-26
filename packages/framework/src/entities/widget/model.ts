import { createContext, useContext } from "solid-js"
import { createStore } from "solid-js/store"

export type WidgetInputs = {
	account_id: string
	widget_id: string
	route_path: string
	route_query: string
	props: string
}

export type WidgetContextState = {
	account_id: null | WidgetInputs["account_id"]
	widget_id: null | WidgetInputs["widget_id"]
	route_path: null | string
	route_query: null | Record<string, undefined | null | boolean | string | object>
	props: null | object
}

const initialLauncherParams: WidgetContextState = {
	account_id: null,
	widget_id: null,
	route_path: null,
	route_query: null,
	props: null,
}

export const WidgetContext = createContext(initialLauncherParams)

export const [widgetContextState, setWidgetContextState] = createStore(initialLauncherParams)

export const useWidgetContext = () => useContext(WidgetContext)
