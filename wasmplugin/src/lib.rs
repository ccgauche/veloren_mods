use veloren_api::*;

use events::*;

#[export_function]
pub fn on_player_join(input: PlayerJoinEvent) -> PlayerJoinResult {
    send_actions(vec![Action::PlayerSendMessage(input.player_id,format!("Welcome {} on our server",input.player_name))]);
    if input.player_name == "Cheater123" {
        PlayerJoinResult {
            keep_connected: false
        }
    } else {
        PlayerJoinResult::default()
    }
}