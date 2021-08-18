mod raft;
mod server;
mod ruftp;

use server::Srv;
extern crate timer;

use tonic::transport::Server;
use ruftp::rpc_proxy_server::RpcProxyServer;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut srv = server::Srv::new(0, vec![]);
    srv.serve();
    srv.connect_to_peer(1, "http://[::1]:50051".to_string());

    Ok(())
}
