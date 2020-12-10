pub mod events;

#[derive(serde::Deserialize,Debug)]
pub enum Action {
    ServerClose,
    PlayerSendMessage(usize,String),
    KillEntity(usize)
}