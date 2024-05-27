use {
	leptos_query::*,
	near_api_lib::{
		accounts::get_account_balance, primitives::types::AccountId as NearAccountId,
		JsonRpcProvider,
	},
	serde::{Deserialize, Serialize},
	std::sync::Arc,
};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct AccountId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
	pub total:        String,
	pub state_staked: String,
	pub staked:       String,
	pub available:    String,
}

pub fn balance_query() -> QueryScope<AccountId, Option<AccountBalance>> {
	create_query(get_balance, QueryOptions::default())
}

async fn get_balance(account_id: AccountId) -> Option<AccountBalance> {
	let provider = Arc::new(JsonRpcProvider::new("https://rpc.mainnet.near.org"));

	match account_id.0.parse::<NearAccountId>() {
		| Ok(account_id) => {
			let result = get_account_balance(provider, account_id.clone()).await;

			match result {
				| Ok(balance) => Some(AccountBalance {
					total:        balance.total,
					state_staked: balance.state_staked,
					staked:       balance.staked,
					available:    balance.available,
				}),
				| Err(_) => None,
			}
		},
		| Err(_) => None,
	}
}
