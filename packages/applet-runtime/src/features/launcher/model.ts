import { createContext } from "solid-js"
import { createStore } from "solid-js/store"

export type AppletLauncherState = {
	account_id: null | string
	applet_id: null | string
	route: null | string
	query: null | string
}

export const initialLauncherParams: AppletLauncherState = {
	account_id: null,
	applet_id: null,
	route: null,
	query: null,
}

export const LauncherContext = createContext(initialLauncherParams)

export const [appletProps, setAppletProps] = createStore(initialLauncherParams)
