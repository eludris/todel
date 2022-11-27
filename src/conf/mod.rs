//! Simple abstraction for a TOML based Eludris configuration file
mod oprish_ratelimits;

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
#[cfg(feature = "logic")]
use std::{env, fs, path};

pub use oprish_ratelimits::OprishRatelimits;

/// Eludris config.
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    pub instance_name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub oprish: OprishConf,
    #[serde(default)]
    pub pandemonium: PandemoniumConf,
    #[serde(default)]
    pub effis: EffisConf,
}

/// Oprish config.
#[derive(Debug, Serialize, Deserialize)]
pub struct OprishConf {
    #[serde(default)]
    pub ratelimits: OprishRatelimits,
    #[serde(default = "message_limit_default")]
    pub message_limit: usize,
}

impl Default for OprishConf {
    fn default() -> Self {
        Self {
            ratelimits: OprishRatelimits::default(),
            message_limit: message_limit_default(),
        }
    }
}

fn message_limit_default() -> usize {
    2048
}

/// Pandemonium config.
#[derive(Debug, Serialize, Deserialize)]
pub struct PandemoniumConf {
    #[serde(default = "pandemonium_ratelimit_default")]
    pub ratelimit: RatelimitData,
}

impl Default for PandemoniumConf {
    fn default() -> Self {
        Self {
            ratelimit: pandemonium_ratelimit_default(),
        }
    }
}

fn pandemonium_ratelimit_default() -> RatelimitData {
    RatelimitData {
        reset_after: 10,
        limit: 5,
    }
}

/// Effis config.
#[derive(Debug, Serialize, Deserialize)]
pub struct EffisConf {
    pub file_size: String,
    #[serde(default)]
    pub ratelimit: EffisRatelimitData,
}

impl Default for EffisConf {
    fn default() -> Self {
        Self {
            file_size: file_size_default(),
            ratelimit: EffisRatelimitData::default(),
        }
    }
}

fn file_size_default() -> String {
    "100MB".to_string()
}

/// Ratelimit config data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatelimitData {
    pub reset_after: u32,
    pub limit: u32,
}

/// Effis ratelimit data config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffisRatelimitData {
    pub reset_after: u32,
    pub limit: u32,
    pub file_size_limit: String,
}

impl Default for EffisRatelimitData {
    fn default() -> Self {
        Self {
            reset_after: 10,
            limit: 25,
            file_size_limit: "200MB".to_string(),
        }
    }
}

#[cfg(feature = "logic")]
macro_rules! validate_ratelimit_limits {
    ($ratelimits:expr, $($bucket_name:ident),+) => {
        if $(
            $ratelimits.$bucket_name.limit == 0
            )||+ {
            Err(anyhow!("Ratelimit limit can't be 0"))?;
        }
    };
}

#[cfg(feature = "logic")]
impl Conf {
    /// Create a new [`Conf`].
    ///
    /// # Panics
    ///
    /// This function is *intended* to panic if a suitable config is not found.
    ///
    /// That also includes the config file's data failing to deserialise.
    pub fn new<T: AsRef<path::Path>>(path: T) -> anyhow::Result<Self> {
        let data = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file {}", path.as_ref().display()))?;
        let data: Self = toml::from_str(&data).with_context(|| {
            format!("Could not parse {} as valid toml", path.as_ref().display())
        })?;
        data.validate()?;
        Ok(data)
    }

    /// Create a new [`Conf`] by determining it's path based on the "ELUDRIS_CONF" environment
    /// variable or falling back to "Eludris.toml" if it is not found.
    ///
    /// # Panics
    ///
    /// This function is *intended* to panic if a suitable config is not found.
    ///
    /// That also includes the config file's data failing to deserialise.
    pub fn new_from_env() -> anyhow::Result<Self> {
        Self::new(env::var("ELUDRIS_CONF").unwrap_or_else(|_| "Eludris.toml".to_string()))
    }

    /// Create a new [`Conf`] with default config from the provided instance name.
    pub fn from_name(instance_name: String) -> anyhow::Result<Self> {
        let conf = Self {
            instance_name,
            description: None,
            oprish: OprishConf::default(),
            pandemonium: PandemoniumConf::default(),
            effis: EffisConf::default(),
        };
        conf.validate()?;
        Ok(conf)
    }

    fn validate(&self) -> Result<(), anyhow::Error> {
        if let Some(description) = &self.description {
            if description.is_empty() || description.len() > 2048 {
                Err(anyhow!(
                    "Invalid description length, must be between 1 and 2048 characters long"
                ))?;
            }
        }
        if self.oprish.message_limit < 1024 {
            Err(anyhow!(
                "Message limit can not be less than 1024 characters"
            ))?;
        }
        if self.pandemonium.ratelimit.limit == 0 || self.effis.ratelimit.limit == 0 {
            Err(anyhow!("Ratelimit limit can't be 0"))?;
        }
        validate_ratelimit_limits!(self.oprish.ratelimits, info, message_create, ratelimits);
        if self.effis.file_size.starts_with('0') {
            Err(anyhow!("Effis max file size cant be 0 or start with 0"))?;
        }

        Ok(())
    }
}

#[cfg(feature = "logic")]
#[cfg(test)]
mod tests {
    use crate::conf::*;

    #[test]
    fn try_deserialize() {
        // This is yucky since there is leading space but TOML thankfully doesn't mind it
        let conf_str = r#"
            instance_name = "WooChat"
            description = "The poggest place to chat"

            [oprish.ratelimits]
            info = { reset_after = 10, limit = 2}

            [pandemonium]
            ratelimit = { reset_after = 20, limit = 10}

            [effis]
            file_size = "100MB"
            ratelimit = { reset_after = 10, limit = 2, file_size_limit = "500MB"}
            "#;

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf {
            instance_name: "WooChat".to_string(),
            description: Some("The poggest place to chat".to_string()),
            oprish: OprishConf {
                ratelimits: OprishRatelimits {
                    info: RatelimitData {
                        reset_after: 10,
                        limit: 2,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            pandemonium: PandemoniumConf {
                ratelimit: RatelimitData {
                    reset_after: 20,
                    limit: 10,
                },
            },
            effis: EffisConf {
                file_size: "100MB".to_string(),
                ratelimit: EffisRatelimitData {
                    reset_after: 10,
                    limit: 2,
                    file_size_limit: "500MB".to_string(),
                },
            },
        };

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }

    #[test]
    fn default_conf() {
        let conf_str = "instance_name = \"TestInstance\"";

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf::from_name("TestInstance".to_string());

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }

    macro_rules! test_limit {
        ($conf:expr, $($limit:expr),+) => {
            $(
                $limit.limit = 0;
                assert!($conf.validate().is_err());
                $limit.limit = 1;
                assert!($conf.validate().is_ok());
            )+
        };
    }

    #[test]
    fn validate() {
        let mut conf = Conf::from_name("WooChat".to_string()).unwrap();

        assert!(conf.validate().is_ok());
        conf.description = Some("".to_string());

        assert!(conf.validate().is_err());
        conf.description = Some("h".repeat(2049));
        assert!(conf.validate().is_err());
        conf.description = Some("very cool".to_string());
        assert!(conf.validate().is_ok());

        conf.oprish.message_limit = 2;
        assert!(conf.validate().is_err());
        conf.oprish.message_limit = 1024;
        assert!(conf.validate().is_ok());

        test_limit!(
            conf,
            conf.pandemonium.ratelimit,
            conf.effis.ratelimit,
            conf.oprish.ratelimits.info,
            conf.oprish.ratelimits.message_create,
            conf.oprish.ratelimits.ratelimits
        );
    }
}
