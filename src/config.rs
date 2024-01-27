use crate::fatal;

use {
    log::{error, info, warn},
    serde::{Deserialize, Serialize},
    std::{env, fs},
};

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    /// http or https
    /// CENTER_URL
    pub center_url: String,
    /// CLUSTER_IP
    pub host_ip: String,
    /// CLUSTER_PORT
    pub host_port: u32,
    /// CLUSTER_ID
    pub cluster_id: String,
    /// CLUSTER_SECRET
    pub cluster_secret: String,
    /// NO_DEMAON
    pub no_demaon: bool,
}

impl Config {
    pub fn new(
        center_url: Option<String>,
        host_ip: String,
        host_port: u32,
        cluster_id: String,
        cluster_secret: String,
        no_demaon: Option<bool>,
    ) -> Self {
        // https://openbmclapi.bangbang93.com
        Self {
            center_url: center_url.unwrap_or("https://openbmclapi.bangbang93.com".to_string()),
            host_ip,
            host_port,
            cluster_id,
            cluster_secret,
            no_demaon: no_demaon.unwrap_or(false),
        }
    }

    pub fn new_from_env() {
        // Load from env
        let center_url = env::var("CENTER_URL");
        let center_url = match center_url {
            Ok(url) => Some(url),
            Err(_) => {
                info!("center url not set, use default");
                None
            }
        };
        let host_ip: String = env::var("CLUSTER_IP").unwrap_or("0.0.0.0".to_string());
        let host_port = env::var("CLUSTER_PORT")
            .unwrap_or("8080".to_string())
            .parse::<u32>()
            .unwrap_or_else(|_| {
                fatal!("CLUSTER_PORT must be a number");
            });
        let no_demaon = env::var("NO_DAEMON").unwrap().parse::<bool>().ok();

        // Load from env
        let cluster_id = env::var("CLUSTER_ID").unwrap_or_else(|err| {
            fatal!("CLUSTER_ID must be set");
        });
        let cluster_secret = env::var("CLUSTER_SECRET").unwrap_or_else(|err| {
            fatal!("CLUSTER_SECRET must be set");
        });
        
        // Decrapated warning
        if env::var("CLUSTER_BYOC").is_ok() {
            warn!("CLUSTER_BYOC is deprecated, ignored");
        }
        if env::var("DISABLE_ACCESS_LOG").is_ok() {
            warn!("DISABLE_ACCESS_LOG is deprecated, ignored");
        }
        if env::var("FORCE_NOOPEN").is_ok() {
            warn!("FORCE_NOOPEN is deprecated, ignored");
        }
        if env::var("ENABLE_NGINX").is_ok() {
            warn!("ENABLE_NGINX is deprecated, ignored");
            // If you want to use Nginx, why would you choose this program?
        }

        // Create config
        let config = Config::new(
            center_url,
            host_ip,
            host_port,
            cluster_id,
            cluster_secret,
            no_demaon,
        );

        // Save config
        config.save();
    }
    pub fn save(&self) {
        if !fs::canonicalize(CONFIG_PATH).is_ok() {
            fs::File::create(CONFIG_PATH).unwrap_or_else(|err| {
                error!("Failed to create config file");
                panic!("{}", err);
            });
            //TODO: Trigger initialization
        }
        fs::write(CONFIG_PATH, toml::to_string(&self).unwrap()).unwrap_or_else(|err| {
            error!("Failed to save config");
            panic!("{}", err);
        });
    }

    // pub fn load() -> Result<Self> {
    //     todo!("Not implemented yet")
    // }

    pub fn join_center_url(&self, path: &str) -> String {
        format!("{}{}", self.center_url, path)
    }
}
