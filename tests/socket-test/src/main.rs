use rust_socketio::{Payload, RawClient};

fn main() {
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
        center_url, config["cluster_id"], config["cluster_secret"]
    );

    let socket = rust_socketio::ClientBuilder::new(url)
        .on("connect", |args, _| {
            println!("Connected: {:?}", args);
        })
        .on("event", |args, _| {
            println!("Event: {:?}", args);
        })
        .on("disconnect", |args, _| {
            println!("Disconnected: {:?}", args);
        })
        .on("error", |args, _| {
            println!("Error!!: {:?}", args);
        })
        .connect()
        .unwrap();

    println!("Connected to server");
    let request_callback = |message: Payload, _: RawClient| {
        println!("Received message: {:?}", message);
    };

    println!("Requesting cert");
    let res = socket.emit_with_ack(
        "request-cert",
        "",
        std::time::Duration::from_secs(10),
        request_callback,
    );
    println!("Request result: {:?}", res);

    std::thread::sleep(std::time::Duration::from_secs(20));
}
