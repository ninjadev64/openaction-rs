use crate::SettingsValue;

use super::Coordinates;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DidReceiveSettingsPayload {
	pub settings: SettingsValue,
	pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize)]
pub struct DidReceiveSettingsEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: DidReceiveSettingsPayload,
}

#[derive(Debug, Deserialize)]
pub struct DidReceiveGlobalSettingsPayload {
	pub settings: SettingsValue,
}

#[derive(Debug, Deserialize)]
pub struct DidReceiveGlobalSettingsEvent {
	pub payload: DidReceiveGlobalSettingsPayload,
}
