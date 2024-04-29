use super::GenericInstancePayload;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppearEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: GenericInstancePayload,
}

#[derive(Debug, Deserialize)]
pub struct PropertyInspectorAppearEvent {
	pub action: String,
	pub context: String,
	pub device: String,
}
