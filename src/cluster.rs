use crate::config::Config;
use crate::fatal;
use crate::utils::avro_data_to_file_list;
use crate::PROTOCOL_VERSION;

use futures_util::FutureExt;
use reqwest::{Client as reqClient, StatusCode};
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload, TransportType,
};
use serde::Deserialize;
use tracing::{debug, info, warn};
use zstd::stream::decode_all;

#[derive(Deserialize, Debug, Clone)]
pub struct SyncFile {
    pub path: String,
    pub hash: String,
    pub size: i64,
}

#[derive(Clone)]
pub struct Cluster {
    pub config: Config,
    pub ua: String,
    pub socket: Client,
}

impl Cluster {
    pub async fn new(config: Config) -> Self {
        let disconnect = |reason: Payload, _: Client| {
            async move {
                fatal!("socket disconnect: {:?}", reason);
            }
            .boxed()
        };
        let ua = format!("openbmclapi-cluster/{}", PROTOCOL_VERSION);
        let socket = ClientBuilder::new(config.center_url.clone())
            .transport_type(TransportType::Websocket)
            .on("error", |err, _| {
                fatal!("socket error {:?}", err);
            })
            .on("message", |msg, _| {
                async move { debug!("socket message: {:?}", msg) }.boxed()
            })
            .on("disconnect", disconnect)
            .connect()
            .await
            .expect("Failed to connect to center");
        info!("websocket connected");
        Self { config, ua, socket }
    }

    pub async fn disconnect(&self) {
        self.socket
            .disconnect()
            .await
            .expect("Failed to disconnect");
    }

    /// ```typescript
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
    /// ```
    pub async fn get_file_list(&self) -> Option<Vec<SyncFile>> {
        // server: https://openbmclapi.bangbang93.com
        // path: /openbmclapi/files
        info!("initing");
        let url = self.config.join_center_url("/openbmclapi/files");
        let password = self.config.cluster_secret.clone();
        let username = self.config.cluster_id.clone();
        let client = reqClient::builder()
            .user_agent(self.ua.clone())
            .build()
            .unwrap();
        info!("getting file list from: {}", url);
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
                info!("got file list len: {}, decompressing", body.len());
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
        pub cluster_port: Option<u32>,
        pub cluster_id: String,
        pub cluster_secret: String,
    }

    fn gen_config() -> Config {
        // 读取 config.toml 获得
        let raw_config = std::fs::read_to_string("config.toml").unwrap();
        let test_conf: TestConfig = toml::from_str(raw_config.as_str()).unwrap();

        Config::new(
            None,
            "127.0.0.1".to_string(),
            test_conf.cluster_port,
            test_conf.cluster_id,
            test_conf.cluster_secret,
            None,
            None,
            None,
        )
    }

    #[cfg(feature = "local_test")]
    #[tokio::test]
    async fn test_get_file_list() {
        crate::log::init_log_with_cli();
        let config = gen_config();
        let cluster = Cluster::new(config).await;
        cluster.get_file_list().await.unwrap();
        cluster.disconnect().await;
    }
}
