import { Component } from "solid-js"
import { Button } from "@/common/ui/components"
import { customElement } from "solid-element"
import { render } from "solid-js/web"

export type AppletLauncherProps = {
	account_id: string
	applet_id: string
	route: string
}

const _AkaiaAppletLauncher: Component<AppletLauncherProps> = (props) => {
	const applet_id = () => props.applet_id
	console.log(applet_id())

	return (
		<div un-w="full">
			<h1>{`Loading ${props.account_id}'s ${applet_id()} ${props.route}...`}</h1>
			<Button variant="default">Ok</Button>
		</div>
	)
}

const launcherDefaultProps: AppletLauncherProps = {
	account_id: "",
	applet_id: "",
	route: "",
}

const tagName = "akaia-applet-launcher"

export class AkaiaAppletLauncher extends HTMLElement {
	static get observedAttributes() {
		return ["account_id", "applet_id", "route"]
	}

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    console.log(`Attribute ${name} has changed to ${newValue}.`);
  }

	connectedCallback() {
		render(
			() => (
				<div un-w="full">
					<h1>
						{"Loading" +
							` ${this.attributes.getNamedItem("account_id")?.value}'s ` +
							this.attributes.getNamedItem("applet_id")?.value +
							` ${this.attributes.getNamedItem("route")?.value}...`}
					</h1>

					<Button variant="default">Ok</Button>
				</div>
			),

			this.attachShadow({ mode: "closed" }),
		)
	}
}

declare global {
	namespace JSX {
		export interface IntrinsicElements {
			[tagName]: AppletLauncherProps
		}
	}
}

export const launcherInit = () => {
	// customElement("akaia-applet-launcher", launcherDefaultProps, AkaiaAppletLauncher);
	if (customElements.get(tagName) === undefined) customElements.define(tagName, AkaiaAppletLauncher)
}
