use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerJoinEvent {
    pub player_name: String,
    pub player_id: usize
}

impl Event for PlayerJoinEvent {
    type Response = PlayerJoinResult;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerJoinResult {
    pub keep_connected: bool,
}

impl Default for PlayerJoinResult {

    fn default() -> Self {
        Self {
            keep_connected: true
        }
    }
}

impl Cancellable for PlayerJoinResult {

    fn set_cancelled(&mut self, cancelled: bool) {
        self.keep_connected = cancelled;
    }
    fn is_cancelled(&self) -> bool {
        self.keep_connected
    }
}