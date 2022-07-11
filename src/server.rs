use futures::{
    future::{self, Ready},
};
use tarpc::{
    context,
    server::{incoming::Incoming, BaseChannel},
    tokio_serde::formats::Json,
};
use lib::EngineRPC;
use futures::{prelude::*};


#[derive(Clone)]
struct EngineServer;


impl EngineRPC for EngineServer {

    type GuestListFut = Ready<Vec<String>>;

    fn guest_list(self, _: context::Context) ->  Self::GuestListFut {
        let mut ret: Vec<String> = Vec::new();
        ret.push(format!("Hello, guest!"));
        future::ready(ret)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let add_listener = tarpc::serde_transport::tcp::listen("127.0.0.1:8080", Json::default)
        .await?
        .filter_map(|r| future::ready(r.ok()));
    let add_server = add_listener
        .map(BaseChannel::with_defaults)
        .execute(EngineServer.serve());
    add_server.await;

    Ok(())
}
