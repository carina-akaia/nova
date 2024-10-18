use {leptos::{html::Div, *},
     leptos_use::{core::Position, use_draggable_with_options, use_window, UseDraggableOptions,
                  UseDraggableReturn}};

#[component]
pub fn Tile(children: Children) -> impl IntoView {
	let root_element = create_node_ref::<Div>();

	let inner_width = use_window().as_ref()
	                              .map(|window| window.inner_width().unwrap().as_f64().unwrap())
	                              .unwrap_or(0.0);

	let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
		root_element,
		UseDraggableOptions::default()
			.initial_value(Position {
				x: inner_width / 3.0,
				y: 80.0,
			})
			.prevent_default(true),
	);

	view! {
		<div
			node_ref={root_element}
			cursor={"move"}
			fixed={"~"}
			style={move || format!("touch-action: none; {}", style())}
			w={"640px"}
			h={"480px"}
			rounded={"xl"}
			bg={"rgba(255, 255, 255, 0.9)"}
			backdrop-blur={"24px"}
			select={"none"}
			shadow={"md"}
			z={"24"}
		>
			<div flex={"~"} gap={"2"} p={"4"} h={"24px"}>
				<span>"🔴"</span>
				<span>"🟠"</span>
				<span>"🟣"</span>
			</div>

			<div flex={"~ col"} gap={"4"} p={"8"} h={"full"}>
				<div text={"sm"} class={"opacity-50"}>
					x =
					{move || x().round()}
					y =
					{move || y().round()}
				</div>

				{children()}
			</div>
		</div>
	}
}
