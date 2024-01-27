

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    // DISABLE_ACCESS_LOG
    // pub disable_access_log: bool,
    // FORCE_NOOPEN
    // pub force_noopen: bool,
    // ENABLE_NGINX
    // pub enable_nginx: bool,
    // NODE_UNIQUE_ID
    // pub node_unique_id: String,
}

impl Config {
    pub fn new(
        center_url: Option<String>,
        host_ip: String,
        host_port: u32,
        cluster_id: String,
        cluster_secret: String,
        no_demaon: bool,
    ) -> Self {
        // https://openbmclapi.bangbang93.com
        Self {
            center_url: center_url.unwrap_or("https://openbmclapi.bangbang93.com".to_string()),
            host_ip,
            host_port,
            cluster_id,
            cluster_secret,
            no_demaon,
        }
    }

    pub fn join_center_url(&self, path: &str) -> String {
        format!("{}{}", self.center_url, path)
    }
}
