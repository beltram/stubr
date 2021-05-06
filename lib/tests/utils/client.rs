#![allow(dead_code)]

use std::{fmt::Debug, str::FromStr};

use async_std::task::block_on;
use http_types::headers::{HeaderName, HeaderValue};
use isahc::{
    Body as IsahcBody,
    ReadResponseExt,
    Response as IsahcResponse,
};
use regex::Regex;
use serde::de::DeserializeOwned;
use surf::Response as SurfResponse;

pub use stubr::Stubr;

pub trait ResponseAsserter {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self;
    fn assert_ok(&mut self) -> &mut Self { self.assert_status_eq(200) }
    fn assert_bad_request(&mut self) -> &mut Self { self.assert_status_eq(400) }
    fn assert_not_found(&mut self) -> &mut Self { self.assert_status_eq(404) }
    fn assert_error(&mut self) -> &mut Self { self.assert_status_eq(500) }
    fn assert_body_text(&mut self, body: &str) -> &mut Self;
    fn assert_body_text_satisfies(&mut self, asserter: fn(&str)) -> &mut Self;
    fn assert_body_text_matches(&mut self, regex: &str) -> &mut Self;
    fn assert_body_json<T>(&mut self, body: T) -> &mut Self where T: DeserializeOwned + PartialEq + Debug;
    fn assert_body_empty(&mut self) -> &mut Self { self.assert_body_text("") }
    fn assert_header(&mut self, key: &str, value: &str) -> &mut Self;
    fn assert_no_header(&mut self, key: &str) -> &mut Self;
    fn assert_content_type_json(&mut self) -> &mut Self { self.assert_header("Content-Type", "application/json") }
    fn assert_content_type_text(&mut self) -> &mut Self { self.assert_header("Content-Type", "text/plain") }
}

impl ResponseAsserter for SurfResponse {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(u16::from(self.status()), status);
        self
    }

    fn assert_body_text(&mut self, body: &str) -> &mut Self {
        assert_eq!(block_on(self.body_string()).unwrap(), body);
        self
    }

    fn assert_body_text_satisfies(&mut self, asserter: fn(&str)) -> &mut Self {
        asserter(block_on(self.body_string()).unwrap().as_str());
        self
    }

    fn assert_body_text_matches(&mut self, regex: &str) -> &mut Self {
        let regex = Regex::new(regex).unwrap();
        let body = block_on(self.body_string()).unwrap();
        assert!(regex.is_match(body.as_str()));
        self
    }

    fn assert_body_json<T>(&mut self, body: T) -> &mut Self where T: DeserializeOwned + PartialEq + Debug {
        assert_eq!(block_on(self.body_json::<T>()).unwrap(), body);
        self
    }

    fn assert_header(&mut self, key: &str, value: &str) -> &mut Self {
        let key = HeaderName::from_str(key).unwrap();
        let value = HeaderValue::from_str(value).unwrap();
        assert_eq!(self.header(key).unwrap().last(), &value);
        self
    }

    fn assert_no_header(&mut self, key: &str) -> &mut Self {
        let key = HeaderName::from_str(key).unwrap();
        assert!(self.header(key).is_none());
        self
    }
}

impl ResponseAsserter for IsahcResponse<IsahcBody> {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(u16::from(self.status()), status);
        self
    }

    fn assert_body_text(&mut self, body: &str) -> &mut Self {
        assert_eq!(self.text().unwrap(), body);
        self
    }

    fn assert_body_text_satisfies(&mut self, asserter: fn(&str)) -> &mut Self {
        asserter(self.text().unwrap().as_str());
        self
    }

    fn assert_body_text_matches(&mut self, regex: &str) -> &mut Self {
        let regex = Regex::new(regex).unwrap();
        assert!(regex.is_match(self.text().unwrap().as_str()));
        self
    }

    fn assert_body_json<T>(&mut self, body: T) -> &mut Self where T: DeserializeOwned + PartialEq + Debug {
        assert_eq!(self.json::<T>().unwrap(), body);
        self
    }

    fn assert_header(&mut self, key: &str, value: &str) -> &mut Self {
        assert_eq!(self.headers().get(key).unwrap(), &value);
        self
    }

    fn assert_no_header(&mut self, key: &str) -> &mut Self {
        assert!(self.headers().get(key).is_none());
        self
    }
}