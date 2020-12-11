
use std::{fs::File, time::Instant};

use api::{Plugin, PreparedEventQuery};
use wasmer_runtime::*;

mod api;


//static WASM: &[u8] = include_bytes!("../wasmplugin/target/wasm32-unknown-unknown/release/wasmplugin.wasm");

use veloren_api_common::events::*;

fn main() {

    let read = std::fs::read("./wasmplugin/target/wasm32-unknown-unknown/release/wasmplugin.wasm").expect("Can't find file");

    let before = Instant::now();
    let plugin= Plugin::new("wasmplugin".to_owned(), &read);
    println!("Plugin compiled by WASM in: {:#?}", before.elapsed());


    let before = Instant::now();
    let prepared = PreparedEventQuery::new(&PlayerJoinEvent {
        player_name: "ccgauche".to_owned(),
        player_id: 1515166644
    }).expect("Can't build WASM request");
    println!("Request prepared in: {:#?}", before.elapsed());
    for _ in 0..100 {
        let before = Instant::now();
        plugin.try_execute("on_player_join", &prepared);
        println!("WASM call made in {:#?}", before.elapsed());
    }
    
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}