import { render } from "solid-js/web"
import {
	type WidgetInputs,
	widgetContextState,
	WidgetContext,
	setWidgetContextState,
} from "./model"
import { ExampleWidget } from "./example"

export { useWidgetContext } from "./model"

const tagName = "akaia-widget"

export const widgetLauncherInit = () => {
	if (customElements.get(tagName) === undefined) {
		customElements.define(
			tagName,

			class AkaiaWidget extends HTMLElement {
				static get observedAttributes(): (keyof WidgetInputs)[] {
					return ["account_id", "widget_id", "route_path", "route_query", "props"]
				}

				attributeChangedCallback(name: keyof WidgetInputs, oldValue: string, newValue: string) {
					if (oldValue !== newValue) {
						setWidgetContextState((props) => ({
							...props,
							[name]: name === "route_query" || name === "props" ? JSON.parse(newValue) : newValue,
						}))
					}
				}

				connectedCallback() {
					render(
						() => {
							return (
								<WidgetContext.Provider value={widgetContextState}>
									<link rel="stylesheet" href="/app/akaia_nova.css" />
									<ExampleWidget />
								</WidgetContext.Provider>
							)
						},

						this.attachShadow({ mode: "closed" }),
					)
				}
			},
		)
	}
}

declare global {
	namespace JSX {
		export interface IntrinsicElements {
			[tagName]: WidgetInputs
		}
	}
}
