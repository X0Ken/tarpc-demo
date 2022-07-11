use tarpc;


#[tarpc::service]
pub trait EngineRPC {
    async fn guest_list() -> Vec<String>;
}

