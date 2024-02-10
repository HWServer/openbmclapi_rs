use rust_socketio::{Payload, RawClient, Event};
use tracing::info;

fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 从命令行读取配置文件路径
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <config file>", args[0]);
        std::process::exit(1);
    }
    let config_path = &args[1];
    let raw_config = std::fs::read_to_string(config_path).unwrap();
    // toml 解析
    let config: toml::Value = toml::from_str(&raw_config).unwrap();

    let center_url = "https://openbmclapi.bangbang93.com";
    let url = format!(
        "{}?clusterId={}&clusterSecret={}",
        center_url, config["cluster_id"].as_str().unwrap(), config["cluster_secret"].as_str().unwrap()
    );

    let socket = rust_socketio::ClientBuilder::new(url)
    .transport_type(rust_socketio::TransportType::Websocket)
        .on_any(|event: Event, payload: Payload, _: RawClient| {
            println!("Received event: {:?} with payload: {:?}", event, payload);
        })
        .on("error", |err, _: RawClient| {
            println!("Received error: {:?}", err);
            // panic!("Error received from server {:?}", err);
        })
        .connect()
        .unwrap();

    println!("Connected to server");
    std::thread::sleep(std::time::Duration::from_secs(2));
    let request_callback = |message: Payload, _: RawClient| {
        match message {
            Payload::Text(datas) => {
                info!("Received certs: {:#?}", datas[0][0][1]);
            },
            _ => ()
        }
    };

    println!("Requesting cert");
    let res = socket.emit_with_ack(
        "request-cert",
        "",
        std::time::Duration::from_secs(10),
        request_callback,
    );
    println!("Emit result: {:?}", res);

    let empty: Vec<u8> = vec![];
    socket.emit("request-cert", Payload::from(empty)).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(20));
}
