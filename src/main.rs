mod raft;
mod ruftp;
mod server;

use server::Srv;
extern crate timer;

use ruftp::rpc_proxy_server::RpcProxyServer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server;
use crate::ruftp::RequestVoteArgs;
use chrono::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut srv = server::Srv::new(0, vec![]);
    srv.serve().await?;
    srv.connect_to_peer(1, "http://[::1]:50051".to_string()).await?;
    srv.call(1, RequestVoteArgs{ term: 1, candidate_id: 1 }).await?;

    sleep(std::time::Duration::new(10, 0)).await;
    Ok(())
}
