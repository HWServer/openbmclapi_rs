use tracing::warn;

pub fn init_log_with_cli() {
    // 命令行参数
    // --warn
    // --debug
    // --trace
    // 从低级开始判断

    let trace = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .try_init();
    if trace.is_err() {
        warn!("init log with trace failed: {:?}", trace.err());
    }
}
