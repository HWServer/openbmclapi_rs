pub fn init_log_with_cli() {
    // 命令行参数
    // --warn
    // --debug
    // --trace
    // 从低级开始判断
    let log_level;
    if std::env::args().any(|x| x == "--trace") {
        log_level = log::LevelFilter::Trace;
    } else if std::env::args().any(|x| x == "--debug") {
        log_level = log::LevelFilter::Debug;
    } else if std::env::args().any(|x| x == "--warn") {
        log_level = log::LevelFilter::Warn;
    } else {
        log_level = log::LevelFilter::Info;
    }
    simple_logger::SimpleLogger::new()
        .with_level(log_level)
        .env()
        .init()
        .unwrap();
}
