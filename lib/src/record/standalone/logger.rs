use std::{fmt::{Display, Formatter, Result}, path::PathBuf};

use http_types::Url;
use log::{error, info};

pub struct RecordLogger;

impl RecordLogger {
    pub fn success(file: PathBuf, status: u16, method: &str, url: &Url) {
        info!("{} -> {:?}", ExchangeLog(status, method, url), file);
    }
    pub fn error(error: anyhow::Error, status: u16, method: &str, url: &Url) {
        error!("failed recording {} because {:?}", ExchangeLog(status, method, url), error);
    }
}

struct ExchangeLog<'a>(u16, &'a str, &'a Url);

impl Display for ExchangeLog<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.0, self.1.to_uppercase(), self.2)
    }
}