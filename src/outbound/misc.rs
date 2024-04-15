use crate::SettingsValue;

use super::{ContextAndPayloadEvent, OutboundEventManager, PayloadEvent};

use serde::Serialize;
use tokio_tungstenite::tungstenite::Error;

#[derive(Serialize)]
struct RegisterEvent {
	event: String,
	uuid: String,
}

#[derive(Serialize)]
struct OpenUrlPayload {
	url: String,
}

#[derive(Serialize)]
struct LogMessagePayload {
	message: String,
}

impl OutboundEventManager {
	pub(crate) async fn register(&mut self, event: String, uuid: String) -> Result<(), Error> {
		self.send_event(RegisterEvent { event, uuid }).await
	}

	pub async fn open_url(&mut self, url: String) -> Result<(), Error> {
		self.send_event(PayloadEvent {
			event: "openUrl",
			payload: OpenUrlPayload { url },
		})
		.await
	}

	pub async fn log_message(&mut self, message: String) -> Result<(), Error> {
		self.send_event(PayloadEvent {
			event: "logMessage",
			payload: LogMessagePayload { message },
		})
		.await
	}

	pub async fn send_to_property_inspector(&mut self, context: String, payload: SettingsValue) -> Result<(), Error> {
		self.send_event(ContextAndPayloadEvent {
			event: "sendToPropertyInspector",
			context,
			payload,
		})
		.await
	}
}
