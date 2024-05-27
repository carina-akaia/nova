use {
	crate::near_account::{get_balance, BalanceQueryResponse, NearAccountId},
	leptos::*,
	// leptos_query::QueryResult,
};

#[component]
pub fn PortfolioDemo(account_id: String) -> impl IntoView {
	let (account_id_signal, _set_account_id) = create_signal(NearAccountId(account_id));

	let data: Resource<NearAccountId, BalanceQueryResponse> =
		create_resource(account_id_signal, get_balance);

	// let QueryResult { data, .. } = balance_query().use_query(move ||
	// account_id.clone().into());

	view! {
		<Transition fallback={move || {
			view! { <div>"Loading..."</div> }
		}}>
			{move || {
				data.get()
					.map(|balance| {
						view! {
							<ul un-flex={"~ col"} un-gap={"2"} un-w={"full"}>
								<li>{balance.unwrap().unwrap().total}</li>
							</ul>
						}
					})
			}}

		</Transition>
	}
}
