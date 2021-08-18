pub mod ruftp {
    tonic::include_proto!("ruftp");
}
use ruftp::rpc_proxy_client::RpcProxyClient;
use ruftp::RequestVoteArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcProxyClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(RequestVoteArgs {
        term: 100,
        candidate_id: 20,
    });

    let response = client.request_vote(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
