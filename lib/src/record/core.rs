use super::RecordConfig;

pub trait Record where Self: Sized {
    fn record(self) -> Self {
        self.record_with(RecordConfig::default())
    }

    fn record_with(self, cfg: RecordConfig) -> Self;
}
