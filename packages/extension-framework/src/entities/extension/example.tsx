import type { Component } from "solid-js"

import { Button } from "@/common/ui/components"
import { useExtensionContext } from "@/entities/extension"
import { capitalize } from "remeda"

export const ExampleExtension: Component = () => {
	const ctx = useExtensionContext()

	return (
		<div
			style={{
				width: "100%",
				height: "100%",
				display: "flex",
				"flex-direction": "column",
				"justify-content": "center",
				"align-items": "center",
				gap: "2rem",
			}}
		>
			<h1>
				{`
					${ctx.account_id} ${ctx.app_id} ${capitalize(ctx.route_path ?? "dashboard")}
					${ctx.route_query !== null ? ` with ID ${ctx.route_query.id} ` : ""}
				`}
			</h1>

			<div style={{ display: "flex", "flex-direction": "column", "align-items": "center" }}>
				<h2>props</h2>
				<code>{ctx.props !== null && JSON.stringify(ctx.props, null, 2)}</code>
			</div>

			<Button variant="default">Ok</Button>
		</div>
	)
}
