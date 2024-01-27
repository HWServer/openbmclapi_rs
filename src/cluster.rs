use crate::config::{ClusterByoc, Config};
use crate::utils::avro_data_to_file_list;
use crate::PROTOCOL_VERSION;

use log::{info, warn};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use zstd::stream::decode_all;

#[derive(Deserialize, Debug, Clone)]
pub struct SyncFile {
    pub path: String,
    pub hash: String,
    pub size: i64,
}

pub struct Cluster {
    pub config: Config,
    pub ua: String,
}

impl Cluster {
    pub fn new(config: Config) -> Self {
        let ua = format!("openbmclapi-cluster/{}", PROTOCOL_VERSION);
        Self { config, ua }
    }

    ///     this.ua = `openbmclapi-cluster/${version}`
    /// this.got = got.extend({
    ///     prefixUrl: this.prefixUrl,
    ///     username: this.clusterId,
    ///     password: this.clusterSecret,
    ///     headers: {
    ///       'user-agent': this.ua,
    ///     },
    ///     responseType: 'buffer',
    ///     timeout: ms('1m'),
    ///   })
    /// public async getFileList(): Promise<IFileList> {
    ///   const FileListSchema = avsc.Type.forSchema({
    ///     type: 'array',
    ///     items: {
    ///       type: 'record',
    ///       fields: [
    ///         {name: 'path', type: 'string'},
    ///         {name: 'hash', type: 'string'},
    ///         {name: 'size', type: 'long'},
    ///       ],
    ///     } as schema.RecordType,
    ///   })
    ///   const res = await this.got.get('openbmclapi/files', {
    ///     responseType: 'buffer',
    ///     cache: this.requestCache,
    ///   })
    ///   const decompressed = await decompress(res.body)
    ///   return {
    ///     files: FileListSchema.fromBuffer(Buffer.from(decompressed)),
    ///   }
    /// }
    pub async fn get_file_list(&self) -> Option<Vec<SyncFile>> {
        // server: https://openbmclapi.bangbang93.com
        // path: /openbmclapi/files
        let url = format!(
            "{}://{}/openbmclapi/files",
            self.config.cluster_byoc.to_string(),
            self.config.center_url.clone()
        );
        let password = self.config.cluster_secret.clone();
        let username = self.config.cluster_id.clone();
        let client = Client::builder().user_agent(self.ua.clone()).build().unwrap();
        let res = client
            .get(url)
            .basic_auth(username, Some(password))
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await;
        if res.is_err() {
            warn!("get file list error: {:?}", res.err());
            return None;
        }
        let res = res.unwrap();
        match res.status() {
            StatusCode::OK => {
                let body = res.bytes().await.unwrap();
                let cur = std::io::Cursor::new(body);
                let raw_data = decode_all(cur);
                if raw_data.is_err() {
                    warn!("decompress file list error: {:?}", raw_data.err());
                    return None;
                }
                let raw_data = raw_data.unwrap();
                avro_data_to_file_list(raw_data)
            }
            _ => {
                warn!("faild to get file list, net status: {:?}", res.status());
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct TestConfig {
        pub cluster_port: u32,
        pub cluster_id: String,
        pub cluster_secret: String,
    }

    fn gen_config() -> Config {
        let center_url = "openbmclapi.bangbang93.com".to_string();
        // 读取 config.toml 获得
        let raw_config = std::fs::read_to_string("config.toml").unwrap();
        let test_conf: TestConfig = toml::from_str(raw_config.as_str()).unwrap();

        let cluster_byoc = ClusterByoc::https;
        let no_demaon = false;
        let disable_access_log = false;
        let force_noopen = false;
        let enable_nginx = false;
        Config::new(
            center_url,
            test_conf.cluster_port,
            test_conf.cluster_id,
            test_conf.cluster_secret,
            cluster_byoc,
            no_demaon,
            disable_access_log,
            force_noopen,
            enable_nginx,
        )
    }

    #[tokio::test]
    async fn test_get_file_list() {
        let config = gen_config();
        let cluster = Cluster::new(config);
        cluster.get_file_list().await;
    }
}
