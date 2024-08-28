use tracing_subscriber::{
  layer::SubscriberExt, reload::Handle, util::SubscriberInitExt, EnvFilter,
  Registry,
};

// TODO: remove hardcorded strings
// TODO: error handling

use crate::config::Values;

pub(super) struct Log {
  reload_handle: Handle<EnvFilter, Registry>,
}

pub(super) fn new() -> anyhow::Result<Log> {
  Log::new()
}

impl Log {
  pub(super) fn new() -> anyhow::Result<Log> {
    let format_layer = tracing_subscriber::fmt::layer();
    let (filter_layer, filter_handle) = tracing_subscriber::reload::Layer::new(
      Self::build_tracing_filter("info")?,
    );
    tracing_subscriber::registry()
      .with(filter_layer)
      .with(format_layer)
      .try_init()?;

    Ok(Log {
      reload_handle: filter_handle,
    })
  }

  pub(super) fn reload(&self, config: Values) -> anyhow::Result<()> {
    if let Some(log_level) = config.log_level {
      self.reload_handle.modify(move |filter| {
        let new_filter = Self::build_tracing_filter(log_level.as_str());
        if let Ok(new_filter) = new_filter {
          *filter = new_filter;
        }
      })?;
    }

    Ok(())
  }

  fn build_tracing_filter(level: &str) -> anyhow::Result<EnvFilter> {
    Ok(
      tracing_subscriber::EnvFilter::builder()
        .with_default_directive(
          tracing::level_filters::LevelFilter::WARN.into(),
        )
        .with_env_var("BLOOPER_LOG")
        .from_env()?
        .add_directive(format!("blooper={level}").parse()?),
    )
  }
}
