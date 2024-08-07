use crate::SettingsValue;

use super::Coordinates;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DialRotatePayload {
	pub settings: SettingsValue,
	pub coordinates: Coordinates,
	pub ticks: i16,
	pub pressed: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialRotateEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DialRotatePayload,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialPressPayload {
	pub controller: String,
	pub settings: SettingsValue,
	pub coordinates: Coordinates,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialPressEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DialPressPayload,
}
