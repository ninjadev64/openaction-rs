use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceSizeInfo {
	pub rows: u8,
	pub columns: u8,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
	pub id: String,
	pub name: String,
	pub size: DeviceSizeInfo,
	pub r#type: u8,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceDidConnectEvent {
	pub device: String,
	pub deviceInfo: DeviceInfo,
}

#[derive(Debug, Deserialize)]
pub struct DeviceDidDisconnectEvent {
	pub device: String,
}
