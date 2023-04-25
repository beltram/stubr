#[allow(clippy::module_inception)]
pub mod grpc {
    tonic::include_proto!("grpc");
    pub mod other {
        tonic::include_proto!("other");
    }
}

pub use grpc::{grpc_client, other::*, *};

pub mod req;
pub mod resp;

#[async_trait::async_trait(? Send)]
pub trait GrpcConnect {
    async fn connect(&self) -> grpc_client::GrpcClient<tonic::transport::Channel>;
    async fn connect_other(&self) -> grpc_other_client::GrpcOtherClient<tonic::transport::Channel>;
}

#[async_trait::async_trait(? Send)]
impl GrpcConnect for stubr::Stubr {
    async fn connect(&self) -> grpc_client::GrpcClient<tonic::transport::Channel> {
        grpc::grpc_client::GrpcClient::connect(self.uri()).await.unwrap()
    }

    async fn connect_other(&self) -> grpc_other_client::GrpcOtherClient<tonic::transport::Channel> {
        grpc::grpc_other_client::GrpcOtherClient::connect(self.uri()).await.unwrap()
    }
}
