import { defineWidget } from "@akaia/widgets-core"
import { SpaceScreen } from "./widget"

export type AkaiaWidget = {
	init: () => void
}

export const AnytypeWidget: AkaiaWidget = defineWidget({
	routes: [{ path: "/space", component: SpaceScreen }],
})
