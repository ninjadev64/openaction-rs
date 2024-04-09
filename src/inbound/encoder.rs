use super::Coordinates;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DialRotatePayload {
	pub settings: serde_json::Value,
	pub coordinates: Coordinates,
	pub ticks: i16,
	pub pressed: bool,
}

#[derive(Deserialize)]
pub struct DialRotateEvent {
	pub event: String,
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DialRotatePayload,
}

#[derive(Deserialize)]
pub struct DialPressPayload {
	pub controller: String,
	pub settings: serde_json::Value,
	pub coordinates: Coordinates,
}

#[derive(Deserialize)]
pub struct DialPressEvent {
	pub event: String,
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DialPressPayload,
}
