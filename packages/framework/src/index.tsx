import { widgetLauncherInit } from "@/entities/widget"

import type { AttributifyAttributes } from "@unocss/preset-attributify"

declare module "solid-js" {
	namespace JSX {
		interface HTMLAttributes<T> extends AttributifyAttributes {}
	}
}

export const init = () => {
	widgetLauncherInit()
}

init()
