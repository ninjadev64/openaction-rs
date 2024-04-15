use crate::SettingsValue;

use super::Coordinates;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DidReceiveSettingsPayload {
	pub settings: SettingsValue,
	pub coordinates: Coordinates,
}

#[derive(Deserialize)]
pub struct DidReceiveSettingsEvent {
	pub event: String,
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DidReceiveSettingsPayload,
}

#[derive(Deserialize)]
pub struct DidReceiveGlobalSettingsPayload {
	pub settings: SettingsValue,
}

#[derive(Deserialize)]
pub struct DidReceiveGlobalSettingsEvent {
	pub event: String,
	pub payload: DidReceiveGlobalSettingsPayload,
}
