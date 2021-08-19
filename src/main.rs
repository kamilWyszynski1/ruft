mod raft;
mod ruftp;
mod server;

use server::Srv;
extern crate timer;
use crate::raft::ConsensusModule;
use crate::ruftp::{RequestVoteArgs, RequestVoteReply};
use crate::server::RPCServer;
use chrono::Duration;
use ruftp::rpc_proxy_server::RpcProxyServer;
use std::fmt::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::time::sleep;
use tonic::transport::Server;

struct MockServer;

impl RPCServer for MockServer {
    fn call_vote(
        &mut self,
        peer_id: &i32,
        req: RequestVoteArgs,
    ) -> Result<RequestVoteReply, Error> {
        println!("call_vote, granting vote");
        Ok(RequestVoteReply {
            term: req.term,
            vote_granted: true,
        })
    }
}

unsafe impl std::marker::Send for MockServer {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut srv = server::Srv::new(0, vec![]);
    // srv.connect_to_peer(1, "http://[::1]:50051".to_string()).await?;
    // srv.connect_to_peer(2, "http://[::1]:50052".to_string()).await?;
    // srv.connect_to_peer(3, "http://[::1]:50053".to_string()).await?;
    // let peers = srv.get_peers();
    // println!("{:?}", peers);

    // let cm = raft::ConsensusModule::new()
    // srv.serve().await?;
    //
    // srv.call(1, RequestVoteArgs{ term: 1, candidate_id: 1 }).await?;
    // srv.call(2, RequestVoteArgs{ term: 2, candidate_id: 1 }).await?;
    // srv.call(3, RequestVoteArgs{ term: 3, candidate_id: 1 }).await?;

    let mut cm = ConsensusModule::new(1, vec![], MockServer {});
    cm.run_election_timer().await;
    sleep(core::time::Duration::new(10, 0)).await;
    Ok(())
}
