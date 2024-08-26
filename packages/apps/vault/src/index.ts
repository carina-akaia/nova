import { defineWidget } from "@akaia/widgets-core"
import { DemoScreen } from "./widget"

export type AkaiaWidget = {
	init: () => void
}

export const VaultWidget: AkaiaWidget = defineWidget({
	routes: [{ path: "/demo", component: DemoScreen }],
})
