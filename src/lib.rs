//! Ctrl+C handling for [`serenity`](https://crates.io/crates/serenity) bots.

#![deny(missing_docs)]

use std::sync::Arc;

use serenity::Client;

pub use ctrlc::Error;

/// Register a Ctrl+C handler that will disconnect the bot.
///
/// # Warning
/// This must be called from the bot's Tokio runtime instance.
pub fn ctrlc(client: &Client) -> Result<(), Error> {
	let rt = tokio::runtime::Handle::current();
	let shard_manager = Arc::clone(&client.shard_manager);
	ctrlc::set_handler(move || {
		let shard_manager = Arc::clone(&shard_manager);
		rt.spawn(async move {
			shard_manager.lock().await.shutdown_all().await;
		});
	})
}

/// Provides a [`ctrlc`](ctrlc()) extension method for [`Client`](Client).
pub trait Ext: private::Sealed + Sized {
	/// Extension method for [`ctrlc`](ctrlc()).
	fn ctrlc(self) -> Result<Self, Error>;
}

impl Ext for Client {
	fn ctrlc(self) -> Result<Self, Error> {
		ctrlc(&self)?;
		Ok(self)
	}
}

mod private {
	use serenity::Client;
	pub trait Sealed {}
	impl Sealed for Client {}
}
