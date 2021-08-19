pub mod ruftp {
    tonic::include_proto!("ruftp");
}
use ruftp::rpc_proxy_client::RpcProxyClient;
use ruftp::rpc_proxy_server::{RpcProxy, RpcProxyServer};
use ruftp::{RequestVoteArgs, RequestVoteReply};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::{ Request, Response, Status};
use tonic::transport::Server;

#[derive(Default)]
pub struct MyRPCProxy {}

#[tonic::async_trait]
impl RpcProxy for MyRPCProxy {
    async fn request_vote(
        &self,
        request: Request<RequestVoteArgs>,
    ) -> Result<Response<RequestVoteReply>, Status> {
        println!("server received: {:?}", request);
        Ok(Response::new(RequestVoteReply {
            term: request.into_inner().term,
            vote_granted: true,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let proxy = MyRPCProxy::default();
    Server::builder()
        .add_service(RpcProxyServer::new(proxy))
        .serve(addr)
        .await;
    Ok(())
}
