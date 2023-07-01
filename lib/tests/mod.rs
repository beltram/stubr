pub mod attributes;
pub mod book;
pub mod grpc;
pub mod misc;
pub mod record;
pub mod req;
pub mod resp;
pub mod utils;

pub use asserhttp::*;
pub use surf as client;

asserhttp_customize!(AssertWiremock);

pub trait AssertWiremock<T>: Asserhttp<T> {
    fn is_ok_iso(&mut self) -> &mut T {
        self.expect_headers(headers::VARY, ["Accept-Encoding", "User-Agent"])
    }

    fn is_error_iso(&mut self) -> &mut T {
        self.expect_status_in_range(200, 501)
    }
}
