import { defineExtension } from "@akaia/framework"
import { DemoScreen } from "./screens/demo"

export type AkaiaExtension = {
	init: () => void
}

export const VaultExtension: AkaiaExtension = defineExtension({
	routes: [{ path: "/demo", component: DemoScreen }],
})
