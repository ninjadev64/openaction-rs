mod inbound;
mod outbound;

use tokio_tungstenite::connect_async;

use futures_util::StreamExt;

pub use inbound::*;
pub use outbound::OutboundEventManager;

struct CliArgs {
	_command: String,
	port: String,
	uuid: String,
	event: String,
}

/// Initialise the plugin and register it with the OpenAction server.
pub async fn init_plugin(event_handler: impl inbound::ActionEventHandler) -> Result<(), anyhow::Error> {
	let mut args = std::env::args();
	let args = CliArgs {
		_command: args.next().unwrap(),
		port: args.nth(1).unwrap(),
		uuid: args.nth(1).unwrap(),
		event: args.nth(1).unwrap(),
	};

	let socket = connect_async(format!("ws://localhost:{}", args.port)).await?.0;
	let (write, read) = socket.split();

	let mut outbound = OutboundEventManager::new(write);
	outbound.register(args.event, args.uuid).await?;
	*outbound::OUTBOUND_EVENT_MANAGER.lock().await = Some(outbound);

	inbound::process_incoming_messages(read, event_handler).await;

	Ok(())
}
