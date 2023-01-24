// Setup utils for the crate
use anyhow::Result;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// Set up the tracing filter level using the env value, or else set it here. Reads RUST_LOG.
/// TRACE < DEBUG < INFO < WARN < ERROR
pub(crate) fn setup() {
  let filter = LevelFilter::INFO.into();
  EnvFilter::builder().with_default_directive(filter).from_env_lossy();
}
