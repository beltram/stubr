use async_trait::async_trait;

#[cfg(feature = "verify-actix")]
pub mod actix;

mod stub_finder;
mod mapping;

#[async_trait(? Send)]
pub trait StubrVerify<T> {
    async fn verify(self);
}