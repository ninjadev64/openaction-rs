mod devices;
mod encoder;
mod keypad;
mod misc;
mod settings;
mod will_appear;

pub use devices::*;
pub use encoder::*;
pub use keypad::*;
pub use misc::*;
pub use settings::*;
pub use will_appear::*;

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
	pub settings: crate::SettingsValue,
	pub coordinates: Coordinates,
	pub controller: String,
	pub state: u16,
	pub is_in_multi_action: bool,
}

#[derive(Deserialize)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
enum InboundEventType {
	/* Action events */
	KeyDown(KeyEvent),
	KeyUp(KeyEvent),
	DialDown(DialPressEvent),
	DialUp(DialPressEvent),
	DialRotate(DialRotateEvent),
	DidReceiveSettings(DidReceiveSettingsEvent),
	WillAppear(AppearEvent),
	WillDisappear(AppearEvent),
	PropertyInspectorDidAppear(PropertyInspectorAppearEvent),
	PropertyInspectorDidDisappear(PropertyInspectorAppearEvent),
	/* Global events */
	DidReceiveGlobalSettings(DidReceiveGlobalSettingsEvent),
	DeviceDidConnect(DeviceDidConnectEvent),
	DeviceDidDisconnect(DeviceDidDisconnectEvent),
	SystemDidWakeUp(SystemDidWakeUpEvent),
}

/// The required return value for event handler functions. It is a ubiquitous Result type for convenience.
pub type EventHandlerResult = Result<(), anyhow::Error>;

/// A trait requiring methods for handling global events.
#[allow(unused_variables)]
pub trait GlobalEventHandler {
	fn did_receive_global_settings(
		&self,
		event: DidReceiveGlobalSettingsEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn device_did_connect(
		&self,
		event: DeviceDidConnectEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn device_did_disconnect(
		&self,
		event: DeviceDidDisconnectEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn system_did_wake_up(
		&self,
		event: SystemDidWakeUpEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}
}

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

	fn did_receive_settings(
		&self,
		event: DidReceiveSettingsEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn will_appear(
		&self,
		event: AppearEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn will_disappear(
		&self,
		event: AppearEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn property_inspector_did_appear(
		&self,
		event: PropertyInspectorAppearEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}

	fn property_inspector_did_disappear(
		&self,
		event: PropertyInspectorAppearEvent,
		outbound: &mut OutboundEventManager,
	) -> impl Future<Output = EventHandlerResult> + Send {
		async { Ok(()) }
	}
}

pub(crate) async fn process_incoming_messages(
	mut stream: SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
	global_event_handler: impl GlobalEventHandler,
	action_event_handler: impl ActionEventHandler,
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
				/* Action events */
				InboundEventType::KeyDown(event) => action_event_handler.key_down(event, outbound).await,
				InboundEventType::KeyUp(event) => action_event_handler.key_up(event, outbound).await,
				InboundEventType::DialDown(event) => action_event_handler.dial_down(event, outbound).await,
				InboundEventType::DialUp(event) => action_event_handler.dial_up(event, outbound).await,
				InboundEventType::DialRotate(event) => action_event_handler.dial_rotate(event, outbound).await,
				InboundEventType::DidReceiveSettings(event) => {
					action_event_handler.did_receive_settings(event, outbound).await
				}
				InboundEventType::WillAppear(event) => action_event_handler.will_appear(event, outbound).await,
				InboundEventType::WillDisappear(event) => action_event_handler.will_disappear(event, outbound).await,
				InboundEventType::PropertyInspectorDidAppear(event) => {
					action_event_handler
						.property_inspector_did_appear(event, outbound)
						.await
				}
				InboundEventType::PropertyInspectorDidDisappear(event) => {
					action_event_handler
						.property_inspector_did_disappear(event, outbound)
						.await
				}
				/* Global events */
				InboundEventType::DidReceiveGlobalSettings(event) => {
					global_event_handler.did_receive_global_settings(event, outbound).await
				}
				InboundEventType::DeviceDidConnect(event) => {
					global_event_handler.device_did_connect(event, outbound).await
				}
				InboundEventType::DeviceDidDisconnect(event) => {
					global_event_handler.device_did_disconnect(event, outbound).await
				}
				InboundEventType::SystemDidWakeUp(event) => {
					global_event_handler.system_did_wake_up(event, outbound).await
				}
			} {
				log::error!("Failed to process inbound event: {}", error.to_string())
			}
		}
	}
}
