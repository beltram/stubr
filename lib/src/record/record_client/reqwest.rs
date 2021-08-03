use reqwest::blocking::RequestBuilder as ReqwestRequestBuilder;

use crate::record::core::Record;

impl Record for ReqwestRequestBuilder {
    fn record(self) {
        let req = self.try_clone().and_then(|it| it.build().ok()).unwrap();
        let resp = self.send().unwrap();
        // see warp_exchange.rs
        // TODO into RecordedExchange
        // then
        // let writer = StubWriter { stub };
        // writer.write(&host, cfg.output)
    }
}