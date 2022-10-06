mod config;
pub mod mikan;
pub mod notifier;

pub use config::*;
pub use notifier::*;

pub type Result<T> = anyhow::Result<T>;
