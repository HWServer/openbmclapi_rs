pub fn init_log_with_cli() {
    // 命令行参数
    // --warn
    // --debug
    // --trace
    // 从低级开始判断

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
