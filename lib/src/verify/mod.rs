use async_trait::async_trait;

#[cfg(feature = "verify-actix")]
mod actix;

mod stub_finder;
mod mapping;

#[async_trait(? Send)]
pub trait StubrVerify {
    async fn verify(self);
}