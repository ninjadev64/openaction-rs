use super::{send_event, OutboundEventManager};

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
struct OpenUrlEvent {
	event: &'static str,
	payload: OpenUrlPayload,
}

impl OutboundEventManager {
	pub(crate) async fn register(&mut self, event: String, uuid: String) -> Result<(), Error> {
		send_event(&mut self.sink, RegisterEvent { event, uuid }).await
	}

	pub async fn open_url(&mut self, url: String) -> Result<(), Error> {
		send_event(
			&mut self.sink,
			OpenUrlEvent {
				event: "openUrl",
				payload: OpenUrlPayload { url },
			},
		)
		.await
	}
}
