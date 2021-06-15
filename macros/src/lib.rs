extern crate proc_macro;

use proc_macro::TokenStream;

use mock::mock_transform;

mod mock;

/// Starts a Stubr mock server and creates a `stubr` variable which can be used to call the server e.g. `stubr.uri()`.
/// It supports both standard and async test functions.
///
/// # Example
/// ```no_run
/// # use isahc;
/// # use stubr_macros as stubr;
/// use asserhttp::*; // optional
///
/// #[test]
/// #[stubr::mock] // <- takes stubs under crate's "tests/stubs" by default
/// #[should_panic] // <- if required place it after '#[stubr::mock]' to avoid unused warnings
/// fn simple_test() {
///     isahc::get(stubr.uri()).expect_status_ok();
/// }
///
/// // also works for async functions
/// #[async_std::test]
/// #[stubr::mock]
/// fn async_test() {
///     isahc::get(stubr.uri()).await.expect_status_ok();
/// }
/// ```
///
/// # Configuration
/// ```no_run
/// # use stubr_macros as stubr;
///
/// // path to stub file (or directory) under crate's "tests/stubs"
/// #[test]
/// #[stubr::mock("path/to/stubs")] // <- or "path/to/stubs/stub.json" for a single file
/// fn default() {}
///
/// // absolute path ; not appending to "tests/stubs"
/// #[test]
/// #[stubr::mock(full_path = "tests/other/stubs")]
/// fn full_path() {}
///
/// // start on a dedicated port
/// #[test]
/// #[stubr::mock(port = 1234)]
/// fn port() {}
/// ```
#[proc_macro_attribute]
pub fn mock(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    mock_transform(args, item.into()).unwrap().into()
}
