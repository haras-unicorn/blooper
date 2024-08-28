#![deny(
  unsafe_code,
  // reason = "Let's just not do it"
)]
#![deny(
  clippy::unwrap_used,
  clippy::expect_used,
  clippy::panic,
  clippy::unreachable,
  clippy::arithmetic_side_effects
  // reason = "We have to handle errors properly"
)]
#![deny(
  clippy::dbg_macro,
  // reason = "Use tracing instead"
)]

mod app;
mod config;
mod log;

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
  let log = log::new()?;
  let config = config::new().await;

  app::run(app::Flags {})?;

  Ok(())
}
