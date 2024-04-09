mod misc;

use futures_util::{stream::SplitSink, SinkExt};
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Error, Message};

type Sink =
	SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>;

async fn send_event(socket: &mut Sink, event: impl Serialize) -> Result<(), Error> {
	socket.send(Message::Text(serde_json::to_string(&event).unwrap())).await
}

/// A struct with methods for sending events to the OpenAction server.
pub struct OutboundEventManager {
	sink: Sink,
}

impl OutboundEventManager {
	pub(crate) fn new(sink: Sink) -> Self {
		Self { sink }
	}
}

/// The outbound event manager available for access outside of event handlers.
pub static OUTBOUND_EVENT_MANAGER: Lazy<Mutex<Option<OutboundEventManager>>> = Lazy::new(|| Mutex::new(None));
