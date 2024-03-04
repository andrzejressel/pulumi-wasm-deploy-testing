// use log::{Record, Level, Metadata, SetLoggerError, error, LevelFilter};
// use pulumi_wasm::PulumiLogSeverity;
//
// pub struct PulumiLogger {
//     pub engine_url: String
// }
//
// impl log::Log for PulumiLogger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         true
//     }
//
//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//             let severity = match record.level() {
//                 Level::Error => PulumiLogSeverity::Error,
//                 Level::Warn => PulumiLogSeverity::Warning,
//                 Level::Info => PulumiLogSeverity::Info,
//                 Level::Debug => PulumiLogSeverity::Debug,
//                 Level::Trace => PulumiLogSeverity::Debug
//             };
//             let message = record.args().to_string();
//             let engine_url = self.engine_url.clone();
//             tokio::spawn(async {
//                 let _ = crate::log(engine_url, severity, message).await;
//             });
//         }
//     }
//
//     fn flush(&self) {}
// }
//
// impl PulumiLogger {
//
//     pub fn init(self) -> Result<(), SetLoggerError> {
//         log::set_max_level(LevelFilter::Trace);
//         log::set_boxed_logger(Box::new(self))
//     }
// }

pub enum PulumiLogSeverity {
    Debug,
    Info,
    Warning,
    Error,
}
