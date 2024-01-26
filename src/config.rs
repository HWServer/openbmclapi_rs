#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClusterByoc {
    http,
    https,
}

impl ClusterByoc {
    pub fn to_string(&self) -> String {
        match self {
            ClusterByoc::http => "http".to_string(),
            ClusterByoc::https => "https".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Config {
    pub center_url: String,
    pub cluster_port: u32,
    pub cluster_id: String,
    pub cluster_secret: String,
    /// CLUSTER_BYOC
    pub cluster_byoc: ClusterByoc,
    pub no_demaon: bool,
    /// DISABLE_ACCESS_LOG
    pub disable_access_log: bool,
    /// FORCE_NOOPEN
    pub force_noopen: bool,
    /// ENABLE_NGINX
    pub enable_nginx: bool,
}

impl Config {
    pub fn new(
        center_url: String,
        cluster_port: u32,
        cluster_id: String,
        cluster_secret: String,
        cluster_byoc: ClusterByoc,
        no_demaon: bool,
        disable_access_log: bool,
        force_noopen: bool,
        enable_nginx: bool,
    ) -> Self {
        Self {
            center_url,
            cluster_port,
            cluster_id,
            cluster_secret,
            cluster_byoc,
            no_demaon,
            disable_access_log,
            force_noopen,
            enable_nginx,
        }
    }
    pub fn new_from_env() -> Self {
        let center_url =
            std::env::var("CENTER_URL").unwrap_or("openbmclapi.bangbang93.com".to_string());
        let cluster_port = std::env::var("CLUSTER_PORT")
            .unwrap_or("8080".to_string())
            .parse::<u32>()
            .expect("CLUSTER_PORT must be a number");
        let cluster_id = std::env::var("CLUSTER_ID").unwrap_or("1".to_string());
        let cluster_secret = std::env::var("CLUSTER_SECRET").expect("CLUSTER_SECRET must be set");
        let cluster_byoc = match std::env::var("CLUSTER_BYOC")
            .unwrap_or("http".to_string())
            .as_str()
        {
            "http" => ClusterByoc::http,
            "https" => ClusterByoc::https,
            _ => panic!("CLUSTER_BYOC must be http or https"),
        };
        let no_demaon = std::env::var("NO_DAEMON")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("NO_DAEMON must be true or false");
        let disable_access_log = std::env::var("DISABLE_ACCESS_LOG")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("DISABLE_ACCESS_LOG must be true or false");
        let force_noopen = std::env::var("FORCE_NOOPEN")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("FORCE_NOOPEN must be true or false");
        let enable_nginx = std::env::var("ENABLE_NGINX")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("ENABLE_NGINX must be true or false");
        Self {
            center_url,
            cluster_port,
            cluster_id,
            cluster_secret,
            cluster_byoc,
            no_demaon,
            disable_access_log,
            force_noopen,
            enable_nginx,
        }
    }
}
