use crate::raft::ConsensusModule;
use crate::ruftp;

use std::collections::HashMap;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use ruftp::rpc_proxy_client::RpcProxyClient;
use ruftp::rpc_proxy_server::{RpcProxy, RpcProxyServer};
use ruftp::{RequestVoteArgs, RequestVoteReply};
use std::cell::RefCell;
use std::fmt::Error;
use std::ops::Deref;
use std::rc::Rc;
use tonic::transport::Channel;

pub trait RPCServer {
    fn call_vote(&mut self, peer_id: &i32, req: RequestVoteArgs) -> Result<RequestVoteReply, Error>;
}

// Server wraps a raft.ConsensusModule along with a rpc.Server that exposes its
// methods as RPC endpoints. It also manages the peers of the Raft server. The
// main goal of this type is to simplify the code of raft.Server for
// presentation purposes. raft.ConsensusModule has a *Server to do its peer
// communication and doesn't have to worry about the specifics of running an
// RPC server.
pub struct Srv {
    server_id: i32,
    peer_ids: Vec<i32>,
    peer_clients: HashMap<i32, RefCell<RpcProxyClient<Channel>>>,
}

impl Srv {
    pub fn new(server_id: i32, peer_ids: Vec<i32>) -> Srv {
        Srv {
            server_id,
            peer_ids,
            peer_clients: Default::default(),
        }
    }

    pub fn default() -> Srv {
        Srv {
            server_id: 0,
            peer_ids: vec![],
            peer_clients: Default::default(),
        }
    }

    pub async fn serve(&mut self, proxy: MyRPCProxy) -> Result<(), tonic::transport::Error> {
        // Create a new RPC server and register a RPCProxy that forwards all methods
        // to n.cm
        let addr = "[::1]:50052".parse().unwrap();
        Server::builder()
            .add_service(RpcProxyServer::new(proxy))
            .serve(addr);
        Ok(())
    }

    pub async fn connect_to_peer(
        &mut self,
        peer_id: i32,
        addr: String,
    ) -> Result<(), tonic::transport::Error> {
        println!("connecting");
        if self.peer_clients.get(&peer_id).is_none() {
            let cli = RpcProxyClient::connect(addr).await?;
            self.peer_clients.insert(peer_id, RefCell::new(cli));
        }
        Ok(())
    }

    fn disconnect_peer(&mut self, peer_id: i32) {
        // TODO: check if remove drop connection
        self.peer_clients.remove(&peer_id);
    }

    pub fn get_peers(&self) -> Vec<i32> {
        self.peer_clients.keys().cloned().collect()
    }

    pub async fn call(
        &mut self,
        peer_id: i32,
        req: RequestVoteArgs,
    ) -> Result<RequestVoteReply, Error> {
        println!("calling");
        let mut res = self.peer_clients.get(&peer_id).unwrap().borrow_mut();
        Ok(res.request_vote(req).await.unwrap().into_inner())
    }
}
#[derive(Default)]
pub struct MyRPCProxy {}

#[tonic::async_trait]
impl RpcProxy for MyRPCProxy {
    async fn request_vote(
        &self,
        request: Request<RequestVoteArgs>,
    ) -> Result<Response<RequestVoteReply>, Status> {
        Ok(Response::new(RequestVoteReply {
            term: request.into_inner().term,
            vote_granted: true,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::server::{HelloServer, MyGreeter, RPCProxy, Srv};
    use std::net::{IpAddr, Ipv6Addr};
    use tarpc::server;

    #[test]
    fn it_works() {}
}
