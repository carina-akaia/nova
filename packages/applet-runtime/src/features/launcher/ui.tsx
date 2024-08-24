import { Component, useContext } from "solid-js"
import { Button } from "@/common/ui/components"
import { render } from "solid-js/web"
import { appletProps, LauncherContext, setAppletProps } from "./model"

const AppletExample: Component = () => {
	const state = useContext(LauncherContext)

	return (
		<div
			w="full"
			h="full"
			style={{
				display: "flex",
				"flex-direction": "column",
				"justify-content": "center",
				"align-items": "center",
			}}
		>
			<h1>{"Loading" + ` ${state.account_id}'s ` + state.applet_id + ` ${state.route}...`}</h1>

			<Button variant="default">Ok</Button>
		</div>
	)
}

const tagName = "akaia-applet-launcher"

export type AppletLauncherParams = {
	account_id: string
	applet_id: string
	route: string
}

export class AkaiaAppletLauncher extends HTMLElement {
	static get observedAttributes() {
		return ["account_id", "applet_id", "route", "query"]
	}

	attributeChangedCallback(name: string, oldValue: string, newValue: string) {
		setAppletProps((props) => ({ ...props, [name]: newValue }))
	}

	connectedCallback() {
		render(
			() => {
				return (
					<LauncherContext.Provider value={appletProps}>
						<AppletExample />
					</LauncherContext.Provider>
				)
			},

			this.attachShadow({ mode: "closed" }),
		)
	}
}

declare global {
	namespace JSX {
		export interface IntrinsicElements {
			[tagName]: AppletLauncherParams
		}
	}
}

export const launcherInit = () => {
	if (customElements.get(tagName) === undefined) customElements.define(tagName, AkaiaAppletLauncher)
}
