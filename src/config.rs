use {
    core::panic,
    log::{info, warn},
    serde::{Deserialize, Serialize},
    std::{
        env, fs,
        path::{Path, PathBuf},
    },
};

const CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// TODO: 将除了 cluster_id, cluster_secret 之外的配置项可选化
pub struct Config {
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
    /// cache dir
    pub cache_dir: PathBuf,
}

impl Config {
    pub fn new(
        center_url: Option<String>,
        host_ip: Option<String>,
        host_port: Option<u32>,
        cluster_id: String,
        cluster_secret: String,
        no_demaon: Option<bool>,
        cache_dir: Option<PathBuf>,
    ) -> Self {
        // cache dir 默认: cwd + 'cache'
        let cache_dir = if let Some(cache_dir) = cache_dir {
            cache_dir
        } else {
            env::current_dir()
                .unwrap_or(PathBuf::from("."))
                .join("cache")
        };
        Self {
            center_url: center_url.unwrap_or("https://openbmclapi.bangbang93.com".to_string()),
            host_ip: host_ip.unwrap_or("0.0.0.0".to_string()),
            host_port: host_port.unwrap_or(8080),
            cluster_id,
            cluster_secret,
            no_demaon: no_demaon.unwrap_or(false),
            cache_dir,
        }
    }

    pub fn convert_from_env() {
        // Load from env
        let center_url = env::var("CENTER_URL").ok();
        let host_ip = env::var("CLUSTER_IP").ok();
        let host_port = env::var("CLUSTER_PORT").unwrap().parse::<u32>().ok();
        let no_demaon = env::var("NO_DAEMON").unwrap().parse::<bool>().ok();
        let cache_dir = env::var("CACHE_DIR").ok().map(|x| PathBuf::from(x));

        let cluster_id = env::var("CLUSTER_ID").unwrap_or_else(|err| {
            todo!("Not implemented yet");
        });

        let cluster_secret = env::var("CLUSTER_SECRET").unwrap_or_else(|err| {
            todo!("Not implemented yet");
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
            cache_dir,
        );

        // Save config
        config.save();
    }

    pub fn save(&self) {
        if !fs::canonicalize(CONFIG_PATH).is_ok() {
            fs::File::create(CONFIG_PATH).unwrap_or_else(|err| {
                todo!("Not implemented yet");
            });
            //TODO: Trigger initialization
        }
        fs::write(CONFIG_PATH, toml::to_string(&self).unwrap()).unwrap_or_else(|err| {
            todo!("Not implemented yet");
        });
    }

    pub fn load(&mut self) {
        if Path::new(CONFIG_PATH).exists() {
            let raw_data: Config = toml::from_str(&fs::read_to_string(CONFIG_PATH).unwrap())
                .unwrap_or_else(|err| {
                    panic!("Failed to load config: {}", err);
                    // TODO: Replace this with fatal!()
                });
            self.center_url = raw_data.center_url;
            self.host_ip = raw_data.host_ip;
            self.host_port = raw_data.host_port;
            self.cluster_id = raw_data.cluster_id;
            self.cluster_secret = raw_data.cluster_secret;
            self.no_demaon = raw_data.no_demaon;
            self.cache_dir = raw_data.cache_dir;
            info!("Config loaded");
        } else {
            panic!("Failed to find config file");
            // TODO: Replace this with fatal!()
        }
    }

    pub fn join_center_url(&self, path: &str) -> String {
        format!("{}{}", self.center_url, path)
    }
}

#[test]
fn test_save_and_load_config() {
    let config: Config = Config::new(Some("https://example.com".to_string()), Some("0.0.0.0".to_string()), Some(23333), "0066ccff".to_string(), "123456789".to_string(), Some(true), Some(PathBuf::from("cache")));
    let mut test_config: Config = Config::new(None, None, None, "111".to_string(), "222".to_string(), None, None);
    config.save();
    test_config.load();
    assert_eq!(test_config.center_url, "https://example.com");
    assert_eq!(test_config.host_ip, "0.0.0.0");
    assert_eq!(test_config.host_port, 23333);
    assert_eq!(test_config.cluster_id, "0066ccff");
    assert_eq!(test_config.cluster_secret, "123456789");
    assert_eq!(test_config.no_demaon, true);
    assert_eq!(test_config.cache_dir, PathBuf::from("cache"));

    // Clean up the temporary config file
    fs::remove_file(CONFIG_PATH).unwrap();
}
