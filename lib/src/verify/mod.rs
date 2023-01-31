#[cfg(feature = "verify-actix")]
pub mod actix;

mod mapping;
mod stub_finder;

#[async_trait::async_trait(? Send)]
pub trait StubrVerify<T>
where
    Self: Sized,
{
    /// Triggers verification of the application from published stubs
    async fn verify(self) {
        self.verify_except(|_| false).await
    }
    /// Same as [`verify`] but accepts ignoring some stubs.
    /// * `except` - ignore stub given its name (without .json suffix).
    async fn verify_except<N>(self, except: impl VerifyExcept<N> + 'async_trait);
}

/// Helps passing a `fn(&str) -> bool` in `verify_except`
pub trait VerifyExcept<T> {
    fn call(&self, name: String) -> bool;
}

impl<F> VerifyExcept<String> for F
where
    F: Fn(String) -> bool,
{
    fn call(&self, name: String) -> bool {
        self(name)
    }
}

impl<F> VerifyExcept<&str> for F
where
    F: Fn(&str) -> bool,
{
    fn call(&self, name: String) -> bool {
        self(name.as_str())
    }
}
