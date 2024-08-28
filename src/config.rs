// TODO: remove hardcoded extensions
// TODO: remove hardcoded env prefix
// TODO: merge overwrite none
// TODO: check log level

pub(super) async fn new() -> Config {
  Config::new().await
}

pub(crate) struct Config {
  values: std::sync::Arc<tokio::sync::Mutex<Values>>,
}

impl Config {
  pub(super) async fn new() -> Self {
    let values = Values::new().await;
    Self {
      values: std::sync::Arc::new(tokio::sync::Mutex::new(values)),
    }
  }
}

#[derive(
  Debug,
  Clone,
  Eq,
  PartialEq,
  Default,
  merge:: Merge,
  serde::Serialize,
  serde::Deserialize,
  clap::Parser,
)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Values {
  #[merge(skip)]
  pub(crate) config_path: Option<String>,

  #[merge(skip)]
  pub(crate) log_level: Option<String>,
}

impl Values {
  async fn new() -> Self {
    let args = Self::parse_args().await;
    let mut values = args;
    if let Ok(env) = Self::parse_env().await {
      merge::Merge::merge(&mut values, env);
    }

    if let Some(file) = Self::find_config_file(values.config_path.clone()).await
    {
      if let Some(extension) = file.clone().extension().and_then(|x| x.to_str())
      {
        if let Ok(raw) = tokio::fs::read_to_string(file).await {
          if let Ok(file) = Self::parse_file(raw.as_str(), extension) {
            merge::Merge::merge(&mut values, file);
          }
        }
      }
    }

    values
  }

  async fn reload(
    &mut self,
    path: Option<std::path::PathBuf>,
  ) -> anyhow::Result<()> {
    let path = if let Some(path) = path {
      path
    } else {
      if let Some(path) = Self::find_config_file(self.config_path.clone()).await
      {
        path
      } else {
        return Ok(());
      }
    };

    if let Some(extension) = path.clone().extension().and_then(|x| x.to_str()) {
      if let Ok(raw) = tokio::fs::read_to_string(path).await {
        if let Ok(file) = Self::parse_file(raw.as_str(), extension) {
          merge::Merge::merge(self, file);
        }
      }
    }

    Ok(())
  }

  async fn parse_args() -> Self {
    clap::Parser::parse()
  }

  async fn parse_env() -> anyhow::Result<Self> {
    Ok(envy::prefixed("BLOOPER_").from_env::<Self>()?)
  }

  fn parse_file(raw: &str, extension: &str) -> anyhow::Result<Self> {
    let values = match extension {
      "toml" => toml::from_str::<Self>(raw)?,
      "yaml" | "yml" => serde_yaml::from_str::<Self>(raw)?,
      "json" => serde_json::from_str::<Self>(raw)?,
      _ => return Err(anyhow::anyhow!("Invalid config file extension")),
    };

    Ok(values)
  }

  async fn find_config_file(
    location: Option<String>,
  ) -> Option<std::path::PathBuf> {
    if let Some(location) = location {
      if tokio::fs::try_exists(location.clone())
        .await
        .is_ok_and(|x| x)
      {
        return Some(std::path::PathBuf::from(location));
      }
    }

    let config_dir = if let Some(config_dir) = Self::find_config_dir() {
      if tokio::fs::try_exists(&config_dir).await.is_ok_and(|x| x) {
        config_dir
      } else {
        return None;
      }
    } else {
      return None;
    };

    Self::find_config_file_in_config_dir(config_dir).await
  }

  async fn find_config_file_in_config_dir(
    config_dir: std::path::PathBuf,
  ) -> Option<std::path::PathBuf> {
    let mut possible_config_files = if let Ok(possible_config_files) =
      tokio::fs::read_dir(config_dir).await
    {
      possible_config_files
    } else {
      return None;
    };

    while let Ok(Some(possible_config_file)) =
      possible_config_files.next_entry().await
    {
      let possible_config_file: std::path::PathBuf =
        possible_config_file.file_name().into();
      if Self::is_config_file(possible_config_file.clone()) {
        return Some(possible_config_file);
      }
    }

    None
  }

  fn is_config_file(path: std::path::PathBuf) -> bool {
    let file_name = if let Some(file_name) = path.to_str() {
      file_name
    } else {
      return false;
    };

    let mut split = file_name.split(".");
    let basename = split.next();
    let extension = split.next();

    if let Some(basename) = basename {
      if let Some(extension) = extension {
        if basename == "config"
          && (extension == "yaml"
            || extension == "yml"
            || extension == "json"
            || extension == "toml")
        {
          return true;
        }
      }
    }

    false
  }

  fn find_config_dir() -> Option<std::path::PathBuf> {
    let project_dirs = if let Some(project_dirs) =
      directories::ProjectDirs::from("xyz", "haras-unicorn", "blooper")
    {
      project_dirs
    } else {
      return None;
    };

    let config_dir = project_dirs.config_dir().to_path_buf();

    Some(config_dir)
  }
}
