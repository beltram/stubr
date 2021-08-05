use crate::RecordConfig;

pub trait Record {
    fn record(&mut self) -> &mut Self {
        self.record_with(RecordConfig::default())
    }

    fn record_with(&mut self, cfg: RecordConfig) -> &mut Self;
}

