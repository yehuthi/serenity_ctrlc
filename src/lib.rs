//! Ctrl+C handling for [`serenity`](https://crates.io/crates/serenity) bots.

#![deny(missing_docs)]

use std::{
	future::Future,
	sync::{Arc, Weak},
};

use serenity::{client::bridge::gateway::ShardManager, prelude::Mutex, Client};

pub use ctrlc::Error;

/// A procedure that [disconnects](Self::disconnect) the bot.
#[derive(Debug)]
#[repr(transparent)]
pub struct Disconnector {
	shard_manager: Arc<Mutex<ShardManager>>,
}

impl Disconnector {
	/// Creates a [`Disconnector`] [`Option`] from a [`Weak`] [`ShardManager`].
	///
	/// Returns [`None`] if the [`ShardManager`] has been already dropped.
	fn from_weak_shard_manager(shard_manager: &Weak<Mutex<ShardManager>>) -> Option<Self> {
		Some(Self {
			shard_manager: shard_manager.upgrade()?,
		})
	}

	/// Disconnects the bot.
	pub async fn disconnect(self) {
		self.shard_manager.lock().await.shutdown_all().await;
	}

	/// Disconnects the bot when there is [`Some`] [`Disconnector`].
	pub async fn disconnect_some(disconnector: Option<Self>) {
		if let Some(disconnector) = disconnector {
			disconnector.disconnect().await;
		}
	}
}

/// Register a Ctrl+C handler that can disconnect the bot.
///
/// If you just want to disconnect the bot if it's running use [`ctrlc`](crate::ctrlc) instead.
/// This function is useful if you want the Ctrl+C handler to do more than just disconnect the bot.
///
/// This function takes an [`FnMut`] that will be the Ctrl+C handler:
/// it takes an [`Option`] which will have [`Some`] [`Disconnector`] if the bot is still running.
///
/// # Warning
/// This must be called from the bot's Tokio runtime instance.
pub fn ctrlc_with<F: Future + Send>(
	client: &Client,
	mut f: impl (FnMut(Option<Disconnector>) -> F) + Send + 'static,
) -> Result<(), Error> {
	let rt = tokio::runtime::Handle::current();
	let shard_manager = Arc::downgrade(&client.shard_manager);
	ctrlc::set_handler(move || {
		let disconnect = Disconnector::from_weak_shard_manager(&shard_manager);
		rt.block_on(async {
			f(disconnect).await;
		});
	})
}

/// Register a Ctrl+C handler that will disconnect the bot.
///
/// If you want to customize the handler see [`ctrlc_with`](crate::ctrlc_with).
///
/// # Warning
/// This must be called from the bot's Tokio runtime instance.
pub fn ctrlc(client: &Client) -> Result<(), Error> {
	ctrlc_with(client, Disconnector::disconnect_some)
}

/// Provides extension methods for [`Client`](Client).
pub trait Ext: private::Sealed + Sized {
	/// Extension method for [`ctrlc`](crate::ctrlc).
	fn ctrlc(self) -> Result<Self, Error>;

	/// Extension method for [`ctrlc_with`](crate::ctrlc_with).
	fn ctrlc_with<F: Future + Send>(
		self,
		f: impl (FnMut(Option<Disconnector>) -> F) + Send + 'static,
	) -> Result<Self, Error>;
}

impl Ext for Client {
	fn ctrlc(self) -> Result<Self, Error> {
		ctrlc(&self)?;
		Ok(self)
	}

	fn ctrlc_with<F: Future + Send>(
		self,
		f: impl (FnMut(Option<Disconnector>) -> F) + Send + 'static,
	) -> Result<Self, Error> {
		ctrlc_with(&self, f)?;
		Ok(self)
	}
}

mod private {
	use serenity::Client;
	pub trait Sealed {}
	impl Sealed for Client {}
}
