import { defineExtension } from "@akaia/framework"
import { SpaceScreen } from "./extension"

export type AkaiaExtension = {
	init: () => void
}

export const AnytypeExtension: AkaiaExtension = defineExtension({
	routes: [{ path: "/space", component: SpaceScreen }],
})
