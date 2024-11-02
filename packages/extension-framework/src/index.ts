import { install } from "@/entities/extension-v1/index"

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
