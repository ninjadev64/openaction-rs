use super::GenericInstancePayload;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppearEvent {
	pub event: String,
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: GenericInstancePayload,
}

#[derive(Deserialize)]
pub struct PropertyInspectorAppearEvent {
	pub event: String,
	pub action: String,
	pub context: String,
	pub device: String,
}
