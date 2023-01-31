use crate::wiremock::{Match, Request};

pub struct GrpcPathMatcher(regex::Regex);

impl GrpcPathMatcher {
    pub fn try_new(path: &str) -> Self {
        Self(regex::Regex::new(path).unwrap())
    }

    pub fn parse_svc_name(request: &Request) -> &str {
        const GRPC_PREFIX: &str = "grpc.Grpc";
        let mut paths = request.url.path_segments().expect("Invalid gRPC request");
        let grpc_prefix = paths
            .next()
            .unwrap_or_else(|| panic!("gRPC request should have prefix '{GRPC_PREFIX}'"));
        assert_eq!(
            grpc_prefix, GRPC_PREFIX,
            "expected first path segment to be '{}' but was '{}'",
            GRPC_PREFIX, grpc_prefix
        );
        paths.next().expect("gRPC request does not have a service name")
    }
}

impl Match for GrpcPathMatcher {
    fn matches(&self, request: &Request) -> bool {
        self.0.is_match(Self::parse_svc_name(request))
    }
}
