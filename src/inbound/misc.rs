use serde::Deserialize;

#[derive(Deserialize)]
pub struct SystemDidWakeUpEvent {
	pub event: String,
}
