use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    pub instance_name: String,
    #[serde(default)]
    pub oprish: OprishConf,
    #[serde(default)]
    pub pandemonium: PandemoniumConf,
    #[serde(default)]
    pub effis: EffisConf,
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            instance_name: "EludrisInstance".to_string(),
            oprish: OprishConf::default(),
            pandemonium: PandemoniumConf::default(),
            effis: EffisConf::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OprishConf {
    #[serde(default)]
    pub ratelimits: OprishRatelimits,
}

impl Default for OprishConf {
    fn default() -> Self {
        Self {
            ratelimits: OprishRatelimits::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OprishRatelimits {
    #[serde(default = "info_ratelimit_default")]
    info: RatelimitData,
    #[serde(default = "message_create_ratelimit_default")]
    message_create: RatelimitData,
}

impl Default for OprishRatelimits {
    fn default() -> Self {
        Self {
            info: info_ratelimit_default(),
            message_create: message_create_ratelimit_default(),
        }
    }
}

fn info_ratelimit_default() -> RatelimitData {
    RatelimitData {
        reset_after: 5,
        limit: 2,
    }
}

fn message_create_ratelimit_default() -> RatelimitData {
    RatelimitData {
        reset_after: 5,
        limit: 10,
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RatelimitData {
    pub reset_after: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Conf {
    /// Create a new [`Conf`]
    ///
    /// # Panics
    ///
    /// This function is *intended* to panic if a suitable config is not found.
    ///
    /// That also includes the config file's data failing to deserialise.
    pub fn new(path: &str) -> Conf {
        let data = fs::read_to_string(path).unwrap();
        toml::from_str(&data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::conf::*;

    #[test]
    fn try_deserialize() {
        let conf_str = r#"
instance_name = "WooChat"

[oprish.ratelimits]
info = { reset_after = 10, limit = 2}

[pandemonium]
ratelimit = { reset_after = 20, limit = 10}
            "#;

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf {
            instance_name: "WooChat".to_string(),
            oprish: OprishConf {
                ratelimits: OprishRatelimits {
                    info: RatelimitData {
                        reset_after: 10,
                        limit: 2,
                    },
                    ..Default::default()
                },
            },
            pandemonium: PandemoniumConf {
                ratelimit: RatelimitData {
                    reset_after: 20,
                    limit: 10,
                },
            },
            ..Default::default()
        };

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }

    #[test]
    fn default_conf() {
        let conf_str = r#"
instance_name = "EludrisInstance"
            "#;

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf::default();

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }
}
