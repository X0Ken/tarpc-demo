use tarpc::{
    client, context,
    tokio_serde::formats::Json,
};

use lib::EngineRPCClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let transport = tarpc::serde_transport::tcp::connect("127.0.0.1:8080", Json::default);
    let client = EngineRPCClient::new(client::Config::default(), transport.await?).spawn();

    let hello = client.guest_list(context::current()).await?;

    println!("{}", hello[0]);

    Ok(())
}
