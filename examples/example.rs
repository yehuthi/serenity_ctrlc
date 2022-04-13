use std::time::Duration;

use serenity::{
	client::{bridge::gateway::GatewayIntents, ClientBuilder, Context, RawEventHandler},
	model::event::Event,
};

use serenity_ctrlc::{Disconnector, Ext};

struct Handler;

#[serenity::async_trait]
impl RawEventHandler for Handler {
	async fn raw_event(&self, _: Context, event: Event) {
		if let Event::Ready(_) = event {
			println!("Bot is ready.");
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let token = std::env::args()
		.nth(1)
		.expect("Token not found in first argument.");
	ClientBuilder::new(token)
		.intents(GatewayIntents::empty())
		.raw_event_handler(Handler)
		.await?
		.ctrlc_with(|dc| async {
			println!("Disconnecting in 3 seconds..");
			tokio::time::sleep(Duration::from_secs(3)).await;
			println!("Disconnecting..");
			Disconnector::disconnect_some(dc).await;
			println!("Disconnected!");
		})?
		.start()
		.await?;
	Ok(())
}
