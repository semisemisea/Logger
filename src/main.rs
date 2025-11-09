use crate::logger::CaptureExt;
use logger::logger;
fn main() {
    let lg = logger::Logger::config()
        .enable_file_name(true)
        .enable_lines(true)
        .set_log_level(0)
        .build();
    lg.debug("Hello from debug");
    lg.info("Hello from info");
    lg.warning("Hello from warning");
    lg.error("Hello from error");
    lg.panic("Hello from panic");
    let res: Result<(), i32> = Err(3);
    res.capture(lg).unwrap_err();
}
