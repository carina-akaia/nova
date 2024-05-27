use serde::{Deserialize, Serialize};

mod balance;

pub use balance::*;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct AccountId(pub String);
