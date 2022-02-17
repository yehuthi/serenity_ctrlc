# serenity_ctrlc [<img src="https://img.shields.io/crates/v/serenity_ctrlc" align="right" />](https://crates.io/crates/serenity_ctrlc)

Graceful Ctrl+C handler for serenity bots.

## Example

```rust
use serenity_ctrlc::Ext;

ClientBuilder::new(token)
	// ...
	.await?
	.ctrlc()? // ←
	.start()
	.await?;
```
