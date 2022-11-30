//! Simple abstraction for a TOML based Eludris configuration file
mod effis_ratelimits;
mod oprish_ratelimits;

use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use anyhow::anyhow;
#[cfg(feature = "http")]
use rocket::data::ByteUnit;

#[cfg(feature = "logic")]
use anyhow::{bail, Context};
#[cfg(feature = "logic")]
use std::{env, fs, path};
#[cfg(feature = "logic")]
use url::Url;

pub use effis_ratelimits::*;
pub use oprish_ratelimits::*;

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
    #[serde(default = "message_limit_default")]
    pub message_limit: usize,
    pub url: Option<String>,
    #[serde(default)]
    pub ratelimits: OprishRatelimits,
}

impl Default for OprishConf {
    fn default() -> Self {
        Self {
            url: None,
            message_limit: message_limit_default(),
            ratelimits: OprishRatelimits::default(),
        }
    }
}

fn message_limit_default() -> usize {
    2048
}

/// Pandemonium config.
#[derive(Debug, Serialize, Deserialize)]
pub struct PandemoniumConf {
    pub url: Option<String>,
    #[serde(default = "pandemonium_ratelimit_default")]
    pub ratelimit: RatelimitConf,
}

impl Default for PandemoniumConf {
    fn default() -> Self {
        Self {
            url: None,
            ratelimit: pandemonium_ratelimit_default(),
        }
    }
}

fn pandemonium_ratelimit_default() -> RatelimitConf {
    RatelimitConf {
        reset_after: 10,
        limit: 5,
    }
}

/// Effis config.
#[derive(Debug, Serialize, Deserialize)]
pub struct EffisConf {
    #[serde(default = "file_size_default")]
    pub file_size: String,
    pub url: Option<String>,
    #[serde(default = "attachment_file_size_default")]
    pub attachment_file_size: String,
    #[serde(default)]
    pub ratelimits: EffisRatelimits,
}

fn file_size_default() -> String {
    "20MB".to_string()
}

fn attachment_file_size_default() -> String {
    "100MB".to_string()
}

impl Default for EffisConf {
    fn default() -> Self {
        Self {
            file_size: file_size_default(),
            url: None,
            attachment_file_size: attachment_file_size_default(),
            ratelimits: EffisRatelimits::default(),
        }
    }
}

/// Ratelimit config data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatelimitConf {
    pub reset_after: u32,
    pub limit: u32,
}

#[cfg(feature = "logic")]
macro_rules! validate_ratelimit_limits {
    ($ratelimits:expr, $($bucket_name:ident),+) => {
        if $(
            $ratelimits.$bucket_name.limit == 0
            )||+ {
            bail!("Ratelimit limit can't be 0");
        }
    };
}

#[cfg(feature = "http")]
macro_rules! validate_file_sizes {
    ($($size:expr),+) => {
       $(
            let size: ByteUnit = $size
            .parse()
            .map_err(|err| anyhow!("{}", err))
            .with_context(|| format!("Invalid file size limit {}", $size))?;
            if size.as_u128() == 0 {
                bail!("File size cannot be 0: {}", $size);
            }
       )+
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
                bail!("Invalid description length, must be between 1 and 2048 characters long");
            }
        }
        if self.oprish.message_limit < 1024 {
            bail!("Message limit can not be less than 1024 characters");
        }
        validate_ratelimit_limits!(self.oprish.ratelimits, info, message_create, ratelimits);
        validate_ratelimit_limits!(self.pandemonium, ratelimit);
        validate_ratelimit_limits!(self.effis.ratelimits, assets, attachments, fetch_file);

        if let Some(url) = &self.oprish.url {
            Url::parse(url).with_context(|| format!("Invalid oprish url {}", url))?;
        }
        if let Some(url) = &self.pandemonium.url {
            Url::parse(url).with_context(|| format!("Invalid pandemonium url {}", url))?;
        }
        if let Some(url) = &self.effis.url {
            Url::parse(url).with_context(|| format!("Invalid effis url {}", url))?;
        }

        #[cfg(feature = "http")]
        validate_file_sizes!(
            self.effis.file_size,
            self.effis.attachment_file_size,
            self.effis.ratelimits.assets.file_size_limit,
            self.effis.ratelimits.attachments.file_size_limit
        );

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
            url = "wss://foo.bar"
            ratelimit = { reset_after = 20, limit = 10}

            [effis]
            file_size = "100MB"

            [effis.ratelimits]
            attachments = { reset_after = 600, limit = 20, file_size_limit = "500MB"}
            "#;

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf {
            instance_name: "WooChat".to_string(),
            description: Some("The poggest place to chat".to_string()),
            oprish: OprishConf {
                ratelimits: OprishRatelimits {
                    info: RatelimitConf {
                        reset_after: 10,
                        limit: 2,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            pandemonium: PandemoniumConf {
                ratelimit: RatelimitConf {
                    reset_after: 20,
                    limit: 10,
                },
                url: Some("wss://foo.bar".to_string()),
            },
            effis: EffisConf {
                file_size: "100MB".to_string(),
                ratelimits: EffisRatelimits {
                    attachments: EffisRatelimitConf {
                        reset_after: 600,
                        limit: 20,
                        file_size_limit: "500MB".to_string(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        };

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }

    #[test]
    fn default_conf() {
        let conf_str = "instance_name = \"TestInstance\"";

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf::from_name("TestInstance".to_string()).unwrap();

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

    macro_rules! test_urls {
        ($conf:expr, $($service:ident),+) => {
            $(
                assert!($conf.validate().is_ok());
                $conf.$service.url = Some("notavalidurl".to_string());
                assert!($conf.validate().is_err());
                $conf.$service.url = Some("http://avalid.url".to_string());
                assert!($conf.validate().is_ok());
            )+
        };
    }

    #[cfg(feature = "http")]
    macro_rules! test_file_sizes {
        ($conf:expr, $($size:expr),+) => {
            $(
                $size = "not a valid size".to_string();
                assert!($conf.validate().is_err());
                $size = "0MB".to_string();
                assert!($conf.validate().is_err());
                $size = "10MB".to_string();
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
            conf.effis.ratelimits.assets,
            conf.effis.ratelimits.attachments,
            conf.effis.ratelimits.fetch_file,
            conf.oprish.ratelimits.info,
            conf.oprish.ratelimits.message_create,
            conf.oprish.ratelimits.ratelimits
        );

        test_urls!(conf, oprish, pandemonium, effis);

        #[cfg(feature = "http")]
        test_file_sizes!(
            conf,
            conf.effis.file_size,
            conf.effis.attachment_file_size,
            conf.effis.ratelimits.assets.file_size_limit,
            conf.effis.ratelimits.attachments.file_size_limit
        );
    }
}
