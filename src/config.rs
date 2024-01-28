use {
    crate::fatal,
    log::{error, info, warn},
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
        host_ip: Option<String>,
        host_port: Option<u32>,
        cluster_id: String,
        cluster_secret: String,
        no_demaon: Option<bool>,
    ) -> Self {
        Self {
            center_url: center_url.unwrap_or("https://openbmclapi.bangbang93.com".to_string()),
            host_ip: host_ip.unwrap_or("0.0.0.0".to_string()),
            host_port: host_port.unwrap_or(8080),
            cluster_id,
            cluster_secret,
            no_demaon: no_demaon.unwrap_or(false),
        }
    }

    pub fn convert_from_env() {
        // Load from env
        let center_url = env::var("CENTER_URL").ok();
        let host_ip = env::var("CLUSTER_IP").ok();
        let host_port = env::var("CLUSTER_PORT").unwrap().parse::<u32>().ok();
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
                fatal!("Failed to create config file");
            });
            //TODO: Trigger initialization
        }
        fs::write(CONFIG_PATH, toml::to_string(&self).unwrap()).unwrap_or_else(|err| {
            fatal!("Failed to save config file");
        });
    }

    pub fn load() -> Result<Self> {
        toml::from_str(&self.load_raw()?).map_err(|err| {
            fatal!("Failed to parse config file");
        })
    }

    pub fn join_center_url(&self, path: &str) -> String {
        format!("{}{}", self.center_url, path)
    }
}
