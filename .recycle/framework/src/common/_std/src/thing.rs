use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize},
               json_types::Base64VecU8,
               serde::{Deserialize, Serialize},
               Timestamp};

type ThingId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ThingMetadata {
	pub id:             ThingId,
	pub name:           String,
	pub description:    String,
	pub icon:           Base64VecU8,
	#[serde(with = "u64_dec_format")]
	pub created_at:     Timestamp,
	/// JSON Schema
	pub payload_schema: String,
}

/// A non-fungible token for distributed relational graph database that
/// describes anything
pub struct Thing {
	pub metadata: ThingMetadata,
	/// Data serialized as JSON
	pub payload:  String,
}

pub trait Consistency {
	fn validate(&self) {
		// TODO: Validate `&self.payload` against `&self.payload_schema`
	}
}

mod u128_dec_format {
	use near_sdk::serde::{de, Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		serializer.serialize_str(&num.to_string())
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
		where D: Deserializer<'de> {
		String::deserialize(deserializer)?.parse()
		                                  .map_err(de::Error::custom)
	}
}

mod u64_dec_format {
	use near_sdk::serde::{de, Deserialize, Deserializer, Serializer};

	pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
		serializer.serialize_str(&num.to_string())
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
		where D: Deserializer<'de> {
		String::deserialize(deserializer)?.parse()
		                                  .map_err(de::Error::custom)
	}
}
