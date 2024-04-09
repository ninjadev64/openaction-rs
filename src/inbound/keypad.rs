use super::GenericInstancePayload;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeyEvent {
	pub event: String,
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: GenericInstancePayload,
}
