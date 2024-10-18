import { install } from "@/entities/extension"

import type { AttributifyAttributes } from "@unocss/preset-attributify"

declare module "solid-js" {
	namespace JSX {
		interface HTMLAttributes<T> extends AttributifyAttributes {}
	}
}

export * from "@/common/ui/components"

export const init = () => {
	install()
}

init()
