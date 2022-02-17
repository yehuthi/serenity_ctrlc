use serenity::{
	client::{bridge::gateway::GatewayIntents, ClientBuilder, Context, RawEventHandler},
	model::event::Event,
};

use serenity_ctrlc::Ext;

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
		.ctrlc()?
		.start()
		.await?;
	Ok(())
}
