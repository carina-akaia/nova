use {
	crate::near_account::{get_balance, BalanceQueryResponse, NearAccountId},
	leptos::*,
};

#[component]
pub fn PortfolioDemo(#[prop(into)] account_id: String) -> impl IntoView {
	let account_id_signal = create_rw_signal(account_id);

	let balance_resource: Resource<String, BalanceQueryResponse> =
		create_resource(account_id_signal, |arg0: std::string::String| {
			get_balance(NearAccountId(arg0))
		});

	view! {
		<Transition fallback={move || {
			view! { <div>"Loading..."</div> }
		}}>
			{move || {
				balance_resource
					.get()
					.map(|balance| {
						view! {
							<div border={"2 solid rounded-lg"} w={"128"} flex={"~ col"}>
								<h1 class={"typography"} m={"0"} p={"4"} border-b={"2 solid"}>
									{format!("{}'s balance", account_id_signal.get().to_string())}
								</h1>

								<ul
									m={"0"}
									p={"4"}
									h={"100%"}
									flex={"~ col"}
									gap={"3"}
									list={"none"}
								>
									<li flex={"~"} items={"center"} gap={"1"}>
										<span class={"typography"} text={"lg"} font={"bold"}>
											{"Total:"}
										</span>

										<span class={"typography"} w={"100%"} text={"center"}>
											{balance.unwrap().unwrap().total}
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
