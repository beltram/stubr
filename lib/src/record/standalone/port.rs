use std::net::{SocketAddr, TcpListener};

pub struct PortAllocator;

impl PortAllocator {
    pub(crate) fn new_binding(port: Option<u16>) -> SocketAddr {
        if let Some(p) = port {
            TcpListener::bind(format!("127.0.0.1:{p}"))
        } else {
            TcpListener::bind("127.0.0.1:0")
        }
        .and_then(|it| it.local_addr())
        .expect("Failed binding stubr recorder to a local port")
    }
}
