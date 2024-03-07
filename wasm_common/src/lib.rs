use std::sync::atomic::Ordering::Relaxed;

use crate::logger::Logger;
use log::kv::{Source, VisitSource};
use log::Log;

mod bindings;
mod logger;

static IS_SET : std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static LOGGER: Logger = logger::Logger {};

pub fn setup_logger() {
    if IS_SET.swap(true, Relaxed) {
        return;
    }
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}