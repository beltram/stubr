use log::LevelFilter;
use simple_logger::SimpleLogger;

pub struct Logger;

impl Logger {
    pub fn init() {
        SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init().unwrap();
    }
}