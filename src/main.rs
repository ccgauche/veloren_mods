
use wasmer_runtime::*;

mod api;


static WASM: &[u8] = include_bytes!("../wasmplugin/target/wasm32-unknown-unknown/release/wasmplugin.wasm");

use veloren_api_common::events::*;

fn main() {
    let instance = instantiate(&WASM, &imports!{"env" => {
        "send_action" => func!(api::read_action),
    }}).expect("failed to instantiate wasm module");
    let memory = instance.context().memory(0);
    let on_player_join = api::get_function(&instance, "on_player_join").expect("Failed to bind on_player_join");
    let out = api::execute_event(memory, &on_player_join, &PlayerJoinEvent {
        player_name: "ccgauche".to_owned(),
        player_id: 585
    });
    println!("{:#?}", out);
}