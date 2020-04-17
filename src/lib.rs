use log::{
    Log,
    Level,
    Metadata,
    Record,
    SetLoggerError,
};

mod console {
    use wasm_bindgen::JsValue;
    pub(super) fn trace(message: &str) {
        let msg = JsValue::from_str(message);
        web_sys::console::log_1(&msg);
    }

    pub(super) fn debug(message: &str) {
        let msg = JsValue::from_str(message);
        web_sys::console::debug_1(&msg);
    }

    pub(super) fn info(message: &str) {
        let msg = JsValue::from_str(message);
        web_sys::console::info_1(&msg);
    }

    pub(super) fn warn(message: &str) {
        let msg = JsValue::from_str(message);
        web_sys::console::warn_1(&msg);
    }

    pub(super) fn error(message: &str) {
        let msg = JsValue::from_str(message);
        web_sys::console::error_1(&msg);
    }
}


pub struct Config {
    pub level: Level
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: Level::Trace
        }
    }
}


static LOGGER: WebLogger = WebLogger;

struct WebLogger;

impl Log for WebLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO Check the args of a location
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            let msg = format!("{}:{} -- {}",
                record.level(),
                record.target(),
                record.args());
            match metadata.level() {
                Level::Trace => console::trace(&msg),
                Level::Debug => console::debug(&msg),
                Level::Info => console::info(&msg),
                Level::Warn => console::warn(&msg),
                Level::Error => console::error(&msg),
            }
        }
    }

    fn flush(&self) {
    }
}

pub fn try_init(config: Config) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    let level = config.level;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() {
    try_init(Config::default()).expect("web_logger::init should not be called after logger initialized");
}

pub fn custom_init(config: Config) {
    try_init(config).expect("web_logger::custom_init should not be called after logger initialized");
}
