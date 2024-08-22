use hello::greeter_client::GreeterClient;
use hello::HelloRequest;
use tonic::transport::Endpoint;
use tonic::Request;
use tower::discover::Change;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (channel, rx) = tonic::transport::Channel::balance_channel(24);
    let mut client = GreeterClient::new(channel);
    let insert = Change::Insert("1", Endpoint::from_static("http://[::1]:50051"));
    rx.send(insert).await?;
    let response = client
        .say_hello(Request::new(HelloRequest {
            name: "Tonic".into(),
        }))
        .await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
