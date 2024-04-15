use crate::SettingsValue;

use super::{ContextAndPayloadEvent, ContextEvent, OutboundEventManager, PayloadEvent, SimpleEvent};

use tokio_tungstenite::tungstenite::Error;

impl OutboundEventManager {
	pub async fn set_settings(&mut self, context: String, payload: SettingsValue) -> Result<(), Error> {
		self.send_event(ContextAndPayloadEvent {
			event: "setSettings",
			context,
			payload,
		})
		.await
	}

	pub async fn get_settings(&mut self, context: String) -> Result<(), Error> {
		self.send_event(ContextEvent {
			event: "getSettings",
			context,
		})
		.await
	}

	pub async fn set_global_settings(&mut self, payload: SettingsValue) -> Result<(), Error> {
		self.send_event(PayloadEvent {
			event: "setGlobalSettings",
			payload,
		})
		.await
	}

	pub async fn get_global_settings(&mut self) -> Result<(), Error> {
		self.send_event(SimpleEvent {
			event: "getGlobalSettings",
		})
		.await
	}
}
