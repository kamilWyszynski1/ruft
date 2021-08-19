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
    srv.connect_to_peer(2, "http://[::1]:50052".to_string()).await?;
    srv.connect_to_peer(3, "http://[::1]:50053".to_string()).await?;

    srv.call(1, RequestVoteArgs{ term: 1, candidate_id: 1 }).await?;
    srv.call(2, RequestVoteArgs{ term: 2, candidate_id: 1 }).await?;
    srv.call(3, RequestVoteArgs{ term: 3, candidate_id: 1 }).await?;
    Ok(())
}
