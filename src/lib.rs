use std::sync::Arc;

use serenity::Client;

pub use ctrlc::Error;

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

pub trait Ext: private::Sealed + Sized {
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
