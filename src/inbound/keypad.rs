use super::GenericInstancePayload;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct KeyEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: GenericInstancePayload,
}
