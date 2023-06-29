//! A collection of different matching strategies provided out-of-the-box by `wiremock`.
//!
//! If the set of matchers provided out-of-the-box is not enough for your specific testing needs
//! you can implement your own thanks to the [`Match`] trait.
//!
//! Furthermore, `Fn` closures that take an immutable [`Request`] reference as input and return a boolean
//! as input automatically implement [`Match`] and can be used where a matcher is expected.
//!
//! Check [`Match`]'s documentation for examples.
use crate::wiremock_rs::{Match, Request};
use http_types::headers::{HeaderName, HeaderValue, HeaderValues};
use http_types::{Method, Url};
use regex::Regex;
use std::convert::TryInto;
use std::str;

/// Implement the `Match` trait for all closures, out of the box,
/// if their signature is compatible.
impl<F> Match for F
where
    F: Fn(&Request) -> bool,
    F: Send + Sync,
{
    fn matches(&self, request: &Request) -> bool {
        // Just call the closure itself!
        self(request)
    }
}

#[derive(Debug)]
/// Match **exactly** the method of a request.
///
/// ### Example:
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::method;
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     let response = ResponseTemplate::new(200);
///     let mock = Mock::given(method("GET")).respond_with(response);
///
///     mock_server.register(mock).await;
///
///     // Act
///     let status = surf::get(&mock_server.uri())
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
pub struct MethodExactMatcher(Method);

/// Shorthand for [`MethodExactMatcher::new`].
pub fn method<T>(method: T) -> MethodExactMatcher
where
    T: TryInto<Method>,
    <T as TryInto<Method>>::Error: std::fmt::Debug,
{
    MethodExactMatcher::new(method)
}

impl MethodExactMatcher {
    pub fn new<T>(method: T) -> Self
    where
        T: TryInto<Method>,
        <T as TryInto<Method>>::Error: std::fmt::Debug,
    {
        let method = method.try_into().expect("Failed to convert to HTTP method.");
        Self(method)
    }
}

impl Match for MethodExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.method == self.0
    }
}

#[derive(Debug)]
/// Match **exactly** the path of a request.
///
/// ### Example:
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::path;
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     let response = ResponseTemplate::new(200).set_body_string("world");
///     let mock = Mock::given(path("/hello")).respond_with(response);
///
///     mock_server.register(mock).await;
///
///     // Act
///     let status = surf::get(format!("{}/hello", &mock_server.uri()))
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
///
/// ### Example:
///
/// The path matcher ignores query parameters:
///
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::path;
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     let response = ResponseTemplate::new(200).set_body_string("world");
///     let mock = Mock::given(path("/hello")).respond_with(response);
///
///     mock_server.register(mock).await;
///
///     // Act
///     let status = surf::get(format!("{}/hello?a_parameter=some_value", &mock_server.uri()))
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
pub struct PathExactMatcher(String);

/// Shorthand for [`PathExactMatcher::new`].
pub fn path<T>(path: T) -> PathExactMatcher
where
    T: Into<String>,
{
    PathExactMatcher::new(path)
}

impl PathExactMatcher {
    pub fn new<T: Into<String>>(path: T) -> Self {
        let path = path.into();

        if path.contains('?') {
            panic!("Wiremock can't match the path `{}` because it contains a `?`. You must use `wiremock::matchers::query_param` to match on query parameters (the part of the path after the `?`).", path);
        }

        if let Ok(url) = Url::parse(&path) {
            if let Some(host) = url.host_str() {
                panic!("Wiremock can't match the path `{}` because it contains the host `{}`. You don't have to specify the host - wiremock knows it. Try replacing your path with `path(\"{}\")`", path, host, url.path());
            }
        }

        // Prepend "/" to the path if missing.
        if path.starts_with('/') {
            Self(path)
        } else {
            Self(format!("/{}", path))
        }
    }
}

impl Match for PathExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.url.path() == self.0
    }
}

#[derive(Debug)]
/// Match the path of a request against a regular expression.
///
/// ### Example:
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::path_regex;
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     let response = ResponseTemplate::new(200).set_body_string("world");
///     let mock = Mock::given(path_regex(r"^/hello/\d{3}$")).respond_with(response);
///
///     mock_server.register(mock).await;
///
///     // Act
///     let status = surf::get(format!("{}/hello/123", &mock_server.uri()))
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
///
/// ### Example:
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::path_regex;
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     let response = ResponseTemplate::new(200).set_body_string("world");
///     let mock = Mock::given(path_regex(r"^/users/[a-z0-9-~_]{1,}/posts$")).respond_with(response);
///
///     mock_server.register(mock).await;
///
///     // Act
///     let status = surf::get(format!("{}/users/da2854ea-b70f-46e7-babc-2846eff4d33c/posts", &mock_server.uri()))
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
pub struct PathRegexMatcher(Regex);

/// Shorthand for [`PathRegexMatcher::new`].
pub fn path_regex<T>(path: T) -> PathRegexMatcher
where
    T: Into<String>,
{
    PathRegexMatcher::new(path)
}

impl PathRegexMatcher {
    pub fn new<T: Into<String>>(path: T) -> Self {
        let path = path.into();

        Self(Regex::new(&path).expect("Failed to create regex for path matcher"))
    }
}

impl Match for PathRegexMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.is_match(request.url.path())
    }
}

#[derive(Debug)]
/// Match **exactly** the header of a request.
///
/// ### Example:
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::{header, headers};
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     Mock::given(header("custom", "header"))
///         .and(headers("cache-control", vec!["no-cache", "no-store"]))
///         .respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     // Act
///     let status = surf::get(&mock_server.uri())
///         .header("custom", "header")
///         .header("cache-control", "no-cache, no-store")
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
pub struct HeaderExactMatcher(HeaderName, HeaderValues);

/// Shorthand for [`HeaderExactMatcher::new`].
pub fn header<K, V>(key: K, value: V) -> HeaderExactMatcher
where
    K: TryInto<HeaderName>,
    <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
    V: TryInto<HeaderValue>,
    <V as TryInto<HeaderValue>>::Error: std::fmt::Debug,
{
    HeaderExactMatcher::new(key, value.try_into().map(HeaderValues::from).unwrap())
}

impl HeaderExactMatcher {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: std::fmt::Debug,
        V: TryInto<HeaderValues>,
        <V as TryInto<HeaderValues>>::Error: std::fmt::Debug,
    {
        let key = key.try_into().expect("Failed to convert to header name.");
        let value = value.try_into().expect("Failed to convert to header value.");
        Self(key, value)
    }
}

impl Match for HeaderExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        match request.headers.get(&self.0) {
            None => false,
            Some(values) => {
                let headers: Vec<&str> = self.1.iter().map(HeaderValue::as_str).collect();
                values.eq(headers.as_slice())
            },
        }
    }
}

#[derive(Debug)]
/// Match **exactly** the query parameter of a request.
///
/// ### Example:
/// ```ignore
/// use crate::wiremock_rs::{MockServer, Mock, ResponseTemplate};
/// use crate::wiremock_rs::matchers::query_param;
///
/// #[async_std::main]
/// async fn main() {
///     // Arrange
///     let mock_server = MockServer::start().await;
///
///     Mock::given(query_param("hello", "world"))
///         .respond_with(ResponseTemplate::new(200))
///         .mount(&mock_server)
///         .await;
///
///     // Act
///     let status = surf::get(format!("{}?hello=world", &mock_server.uri()))
///         .await
///         .unwrap()
///         .status();
///
///     // Assert
///     assert_eq!(status, 200);
/// }
/// ```
pub struct QueryParamExactMatcher(String, String);

impl QueryParamExactMatcher {
    /// Specify the expected value for a query parameter.
    pub fn new<K: Into<String>, V: Into<String>>(key: K, value: V) -> Self {
        let key = key.into();
        let value = value.into();
        Self(key, value)
    }
}

/// Shorthand for [`QueryParamExactMatcher::new`].
pub fn query_param<K, V>(key: K, value: V) -> QueryParamExactMatcher
where
    K: Into<String>,
    V: Into<String>,
{
    QueryParamExactMatcher::new(key, value)
}

impl Match for QueryParamExactMatcher {
    fn matches(&self, request: &Request) -> bool {
        request
            .url
            .query_pairs()
            .any(|q| q.0 == self.0.as_str() && q.1 == self.1.as_str())
    }
}
