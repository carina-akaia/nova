use {
	super::AccountId,
	leptos::*,
	leptos_query::*,
	serde::{Deserialize, Serialize},
	std::sync::Arc,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
	pub total:        String,
	pub state_staked: String,
	pub staked:       String,
	pub available:    String,
}

type BalanceResponse = Result<Option<AccountBalance>, ServerFnError>;

pub fn balance_query() -> QueryScope<AccountId, BalanceResponse> {
	create_query(get_balance, QueryOptions::default())
}

#[server(GetBalance, "/near_account/balance")]
async fn get_balance(account_id: AccountId) -> Result<Option<AccountBalance>, ServerFnError> {
	use near_api_lib::{
		accounts::get_account_balance, primitives::types::AccountId as NearAccountId,
		JsonRpcProvider,
	};

	let provider = Arc::new(JsonRpcProvider::new("https://rpc.mainnet.near.org"));

	match account_id.0.parse::<NearAccountId>() {
		| Ok(parsed_account_id) => {
			let result = get_account_balance(provider, parsed_account_id.clone()).await;

			match result {
				| Ok(balance) => Ok(Some(AccountBalance {
					total:        balance.total,
					state_staked: balance.state_staked,
					staked:       balance.staked,
					available:    balance.available,
				})),
				| Err(_) => Ok(None),
			}
		},
		| Err(_) => Ok(None),
	}
}
