pub mod ruftp {
    tonic::include_proto!("ruftp");
}
use ruftp::rpc_proxy_client::RpcProxyClient;
use ruftp::rpc_proxy_server::{RpcProxy, RpcProxyServer};
use ruftp::{RequestVoteArgs, RequestVoteReply};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use tonic::transport::{Error, Server};
use tonic::{Request, Response, Status};

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
    let args: Vec<String> = env::args().collect();

    let num_o = args.get(1);
    let mut num = 1;
    if num_o.is_some() {
        num = num_o.unwrap().parse::<i32>()?;
    }
    let mut handles = vec![];

    for id in 0..num {
        let addr = format!("[::1]:5005{}", id + 1).parse().unwrap();
        println!("started {} server on: {}", id + 1, addr);

        handles.push(tokio::spawn(start_server(addr)));
    }
    futures::future::join_all(handles).await;
    loop {

    }
    Ok(())
}

async fn start_server(addr: SocketAddr) -> Result<(), Error> {
    let proxy = MyRPCProxy::default();
    Server::builder()
        .add_service(RpcProxyServer::new(proxy))
        .serve(addr)
        .await?;
    Ok(())
}
