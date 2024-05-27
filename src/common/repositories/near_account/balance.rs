use {
	super::NearAccountId,
	leptos::*,
	// leptos_query::*,
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearAccountBalance {
	pub total:        String,
	pub state_staked: String,
	pub staked:       String,
	pub available:    String,
}

pub type BalanceQueryResponse = Result<Option<NearAccountBalance>, ServerFnError>;

// pub fn balance_query() -> QueryScope<NearAccountId, BalanceQueryResponse> {
// 	create_query(get_balance, QueryOptions::default())
// }

// #[cfg(feature = "ssr")]
#[server(name=GetBalance, prefix="/api", endpoint="near_account/balance")]
pub async fn get_balance(
	account_id: NearAccountId,
) -> Result<Option<NearAccountBalance>, ServerFnError> {
	use {
		near_api_lib::{
			accounts::get_account_balance, primitives::types::AccountId, JsonRpcProvider,
		},
		std::sync::Arc,
	};

	let provider = Arc::new(JsonRpcProvider::new("https://rpc.mainnet.near.org"));

	match account_id.0.parse::<AccountId>() {
		| Ok(parsed_account_id) => {
			let result = get_account_balance(provider, parsed_account_id.clone()).await;

			match result {
				| Ok(balance) => Ok(Some(NearAccountBalance {
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
