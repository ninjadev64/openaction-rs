use super::Coordinates;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DialRotatePayload {
	pub settings: serde_json::Value,
	pub coordinates: Coordinates,
	pub ticks: i16,
	pub pressed: bool,
}

#[derive(Debug, Deserialize)]
pub struct DialRotateEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DialRotatePayload,
}

#[derive(Debug, Deserialize)]
pub struct DialPressPayload {
	pub controller: String,
	pub settings: serde_json::Value,
	pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct DialPressEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DialPressPayload,
}
