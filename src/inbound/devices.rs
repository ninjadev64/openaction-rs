use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeviceSizeInfo {
	pub rows: u8,
	pub columns: u8,
}

#[derive(Deserialize)]
pub struct DeviceInfo {
	pub id: String,
	pub name: String,
	pub size: DeviceSizeInfo,
	pub r#type: u8,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceDidConnectEvent {
	pub event: String,
	pub device: String,
	pub deviceInfo: DeviceInfo,
}

#[derive(Deserialize)]
pub struct DeviceDidDisconnectEvent {
	pub event: String,
	pub device: String,
}
