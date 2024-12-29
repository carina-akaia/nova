import { install } from "@/lib/entities/extension/index"

import type { AttributifyAttributes } from "@unocss/preset-attributify"

declare module "solid-js" {
	namespace JSX {
		interface HTMLAttributes<T> extends AttributifyAttributes {}
	}
}

export const init = () => {
	install()
}

init()
