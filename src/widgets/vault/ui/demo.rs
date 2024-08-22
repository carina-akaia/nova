use {
	crate::near::{get_balance, BalanceQueryResponse, NearAccountBalance, NearAccountId},
	leptos::*,
};

#[component]
pub fn Vault(#[prop(into)] account_id_signal: RwSignal<NearAccountId>) -> impl IntoView {
	let balance_resource: Resource<NearAccountId, BalanceQueryResponse> =
		create_resource(account_id_signal, get_balance);

	view! {
		<Transition fallback={move || {
			view! { <div>"Loading..."</div> }
		}}>
			{move || {
				balance_resource
					.get()
					.map(|balance| {
						let NearAccountBalance { available, total, staked, state_staked } = balance
							.unwrap()
							.unwrap();

						view! {
							<div border={"2 solid rounded-lg"} w={"128"} flex={"~ col"}>
								<h1 class={"typography"} m={"0"} p={"4"} border-b={"2 solid"}>
									{format!("{}'s balance", account_id_signal.get().0)}
								</h1>

								<ul
									m={"0"}
									p={"4"}
									h={"100%"}
									flex={"~ col"}
									gap={"3"}
									list={"none"}
								>
									<li flex={"~"} items={"center"} gap={"2"}>
										<span
											class={"typography"}
											text={"lg"}
											font={"bold"}
											w={"64"}
										>
											{"Available:"}
										</span>

										<span class={"typography"} w={"100%"} text={"center"}>
											{format!(
												"{:.8} Ⓝ",
												available.parse::<f32>().unwrap_or(0.0)
													/ 1000000000000000000000000.0,
											)
												.to_string()}
										</span>
									</li>

									<li flex={"~"} items={"center"} gap={"2"}>
										<span
											class={"typography"}
											text={"lg"}
											font={"bold"}
											w={"64"}
										>
											{"Locked for storage:"}
										</span>

										<span class={"typography"} w={"100%"} text={"center"}>
											{format!(
												"{:.8} Ⓝ",
												state_staked.parse::<f32>().unwrap_or(0.0)
													/ 1000000000000000000000000.0,
											)
												.to_string()}
										</span>
									</li>

									<li flex={"~"} items={"center"} gap={"2"}>
										<span
											class={"typography"}
											text={"lg"}
											font={"bold"}
											w={"64"}
										>
											{"Staked:"}
										</span>

										<span class={"typography"} w={"100%"} text={"center"}>
											{format!(
												"{:.8} Ⓝ",
												staked.parse::<f32>().unwrap_or(0.0)
													/ 1000000000000000000000000.0,
											)
												.to_string()}
										</span>
									</li>

									<li flex={"~"} items={"center"} gap={"2"}>
										<span
											class={"typography"}
											text={"lg"}
											font={"bold"}
											w={"64"}
										>
											{"Total:"}
										</span>

										<span class={"typography"} w={"100%"} text={"center"}>
											{format!(
												"{:.8} Ⓝ",
												total.parse::<f32>().unwrap_or(0.0)
													/ 1000000000000000000000000.0,
											)
												.to_string()}
										</span>
									</li>
								</ul>
							</div>
						}
					})
			}}

		</Transition>
	}
}
