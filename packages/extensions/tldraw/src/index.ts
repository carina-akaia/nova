import { defineExtension } from "@akaia/framework"
import { TLDrawCanvas } from "./views/canvas"

export type AkaiaExtension = {
	init: () => void
}

export const TLDrawExtension: AkaiaExtension = defineExtension({
	routes: [{ path: "/demo", component: TLDrawCanvas }],
})
