use serde::{Serialize, de::DeserializeOwned};
use serde::Deserialize;

mod player_join;

pub use player_join::*;


pub trait Event: Serialize + DeserializeOwned{
    type Response: Serialize + DeserializeOwned;
}

pub trait Cancellable {

    fn set_cancelled(&mut self, cancelled: bool);
    fn is_cancelled(&self) -> bool;
}
