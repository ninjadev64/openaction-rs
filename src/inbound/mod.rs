mod encoder;
mod keypad;

pub use encoder::*;
pub use keypad::*;

use crate::outbound::OutboundEventManager;

use std::future::Future;

use futures_util::{stream::SplitStream, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

/// A representation of the coordinates of an action instance.
#[derive(Deserialize)]
pub struct Coordinates {
	pub row: u8,
	pub column: u8,
}

/// A representation of the payload data that accompanies events related to actions.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericInstancePayload {
	pub settings: serde_json::Value,
	pub coordinates: Coordinates,
	pub controller: String,
	pub state: u16,
	pub is_in_multi_action: bool,
}

#[derive(Deserialize)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
enum InboundEventType {
	KeyDown(KeyEvent),
	KeyUp(KeyEvent),
	DialDown(DialPressEvent),
	DialUp(DialPressEvent),
	DialRotate(DialRotateEvent),
}

/// The required return value for event handler functions. It is a ubiquitous Result type for convenience.
pub type EventHandlerResult = Result<(), anyhow::Error>;

/// A trait requiring methods for handling events related to an action.
#[allow(unused_variables)]
pub trait ActionEventHandler {
	fn key_down(
		&self,
		event: KeyEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn key_up(
		&self,
		event: KeyEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn dial_down(
		&self,
		event: DialPressEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn dial_up(
		&self,
		event: DialPressEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn dial_rotate(
		&self,
		event: DialRotateEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}
}

pub(crate) async fn process_incoming_messages(
	mut stream: SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
	event_handler: impl ActionEventHandler,
) {
	while let Some(message) = stream.next().await {
		let Ok(data) = message else {
			continue;
		};

		if let Message::Text(text) = data {
			let decoded: InboundEventType = match serde_json::from_str(&text) {
				Ok(event) => event,
				Err(_) => {
					log::warn!(
						"Unknown event received: {}",
						serde_json::from_str::<serde_json::Value>(&text)
							.unwrap()
							.as_object()
							.unwrap()
							.get("event")
							.unwrap()
					);
					continue;
				}
			};

			let mut lock = crate::outbound::OUTBOUND_EVENT_MANAGER.lock().await;
			let outbound = lock.as_mut().unwrap();

			if let Err(error) = match decoded {
				InboundEventType::KeyDown(event) => event_handler.key_down(event, outbound).await,
				InboundEventType::KeyUp(event) => event_handler.key_up(event, outbound).await,
				InboundEventType::DialDown(event) => event_handler.dial_down(event, outbound).await,
				InboundEventType::DialUp(event) => event_handler.dial_up(event, outbound).await,
				InboundEventType::DialRotate(event) => event_handler.dial_rotate(event, outbound).await,
			} {
				log::error!("Failed to process inbound event: {}", error.to_string())
			}
		}
	}
}
