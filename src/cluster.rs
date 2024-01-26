use crate::utils::avro_data_to_file_list;
use crate::config::{ClusterByoc, Config};
use crate::PROTOCOL_VERSION;

use reqwest::{Client, StatusCode};
use serde::Deserialize;
use zstd::stream::decode_all;

#[derive(Deserialize, Debug, Clone)]
pub struct SyncFile {
    pub path: String,
    pub hash: String,
    pub size: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SyncFileList {
    pub file: Vec<SyncFile>,
}

pub struct Cluster {
    pub config: Config,
}

impl Cluster {
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
    pub async fn get_file_list(&self) -> Vec<SyncFile> {
        // server: https://openbmclapi.bangbang93.com
        // path: /openbmclapi/files
        let url = format!(
            "{}://{}/openbmclapi/files",
            self.config.cluster_byoc.to_string(),
            self.config.center_url.clone()
        );
        let ua = format!("openbmclapi-cluster/{}", PROTOCOL_VERSION);
        let password = self.config.cluster_secret.clone();
        let username = self.config.cluster_id.to_string();
        let client = Client::builder().user_agent(ua).build().unwrap();
        let res = client
            .get(url)
            .basic_auth(username, Some(password))
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await;
        if res.is_err() {
            panic!("get file list error: {:?}", res.err());
        }
        let res = res.unwrap();
        match res.status() {
            StatusCode::OK => {
                let body = res.bytes().await.unwrap();
                let cur = std::io::Cursor::new(body);
                let raw_data = decode_all(cur);
                if raw_data.is_err() {
                    panic!("decompress file list error: {:?}", raw_data.err());
                }
                let raw_data = raw_data.unwrap();
                let file_list = avro_data_to_file_list(raw_data);
                if file_list.is_err() {
                    panic!("parse file list error: {:?}", file_list.err());
                }
                let file_list = file_list.unwrap();
                file_list
            }
            _ => {
                panic!("error status: {:?}", res.status());
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
        let cluster = Cluster { config };
        cluster.get_file_list().await;
    }
}
