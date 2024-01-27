use {
    log::{error, warn},
    serde::{Deserialize, Serialize},
    serde_json::Result,
    std::{env, fs, io::Error},
};

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    /// http or https
    /// CLUSTER_BYOC + CENTER_URL
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
    // DISABLE_ACCESS_LOG [DECRAPEATED]
    // pub disable_access_log: bool,
    // FORCE_NOOPEN [DECRAPEATED]
    // pub force_noopen: bool,
    // ENABLE_NGINX [DECRAPEATED]
    // pub enable_nginx: bool,
}

impl Config {
    pub fn new(
        center_url: String,
        host_ip: String,
        host_port: u32,
        cluster_id: String,
        cluster_secret: String,
        no_demaon: bool,
    ) -> Self {
        // https://openbmclapi.bangbang93.com
        Self {
            center_url,
            host_ip,
            host_port,
            cluster_id,
            cluster_secret,
            no_demaon,
        }
    }

    pub fn convert_from_env() {
        // Load from env
        let center_url = env::var("CENTER_URL").unwrap_or("openbmclapi.bangbang93.com".to_string());
        let host_ip: String = env::var("CLUSTER_IP").unwrap_or("0.0.0.0".to_string());
        let host_port = env::var("CLUSTER_PORT")
            .unwrap_or("8080".to_string())
            .parse::<u32>()
            .unwrap_or_else(|_| {
                error!("CLUSTER_PORT must be a number");
                panic!();
            });
        let cluster_id = env::var("CLUSTER_ID").unwrap_or_else(|_| {
            error!("CLUSTER_ID must be set");
            panic!();
        });
        let cluster_secret = env::var("CLUSTER_SECRET").unwrap_or_else(|_| {
            error!("CLUSTER_SECRET must be set");
            panic!();
        });
        let no_demaon = env::var("NO_DAEMON")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or_else(|_| {
                error!("NO_DAEMON must be true or false");
                panic!();
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
            fs::File::create(CONFIG_PATH).unwrap_or_else(|_| {
                error!("Failed to create config file");
                panic!();
            });
        }
        fs::write(CONFIG_PATH, toml::to_string(&self).unwrap()).unwrap_or_else(|_| {
            error!("Failed to save config");
            panic!();
        });
    }

    pub fn load() -> Result<Self> {
        todo!("Not implemented yet")
    }

    pub fn join_center_url(&self, path: &str) -> String {
        format!("{}{}", self.center_url, path)
    }
}
