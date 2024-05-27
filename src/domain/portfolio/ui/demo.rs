use {
	crate::account::{balance_query, AccountId},
	leptos::*,
	leptos_query::QueryResult,
};

#[island]
pub fn PortfolioDemo(account_id: AccountId) -> impl IntoView {
	let QueryResult { data, .. } = balance_query().use_query(move || account_id.clone().into());

	view! {
		<Transition fallback={move || {
			view! { <div>"Loading..."</div> }
		}}>
			{move || {
				data.get().unwrap().map(|balance| {
						view! {
							<ul un-flex={"~ col"} un-gap={"2"} un-w={"full"}>
								<li>{balance.total}</li>
							</ul>
						}
					})
			}}

		</Transition>
	}
}
