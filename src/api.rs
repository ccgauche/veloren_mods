use serde::de::DeserializeOwned;
use wasmer_runtime::*;

use veloren_api_common::{Action,events};

pub type Function<'a> = Func<'a,(i32, u32), i32>;

pub fn get_function<'a>(instance: &'a Instance, name: &str) -> Result<Function<'a>, &'static str>{
    instance.exports.get::<Function>(name).map_err(|_| "Can't get Function")
}

const MEMORY_POS: usize = 100000;

pub fn execute<'a,T>(memory: &Memory, function: &Function,input: &impl serde::Serialize) -> Result<T, &'static str> where T: DeserializeOwned {
    let view = memory.view::<u8>();
    let bytes = bincode::serialize(&input).map_err(|_| "Failed to serialize structure with bincode")?;
    let len = bytes.len();
    for (cell, byte) in view[MEMORY_POS..len + MEMORY_POS].iter().zip(bytes.iter()) {
        cell.set(*byte)
    }
    let start = function.call(MEMORY_POS as i32, len as u32).expect("Failed to execute function") as usize;
    let new_view = memory.view::<u8>();
    let mut new_len_bytes = [0u8;4];
    for i in 0..4 {
        new_len_bytes[i] = new_view.get(i + 1).map(|c| c.get()).unwrap_or(0);
    }
    let new_len = u32::from_ne_bytes(new_len_bytes) as usize;
    let end = start + new_len;
    let updated_bytes: Vec<u8> = new_view[start..end]
                                    .iter()
                                    .map(|c|c.get())
                                    .collect();

    let o = bincode::deserialize(&updated_bytes);
    o.map_err(|_| "Failed to convert wasm memory to output")
}

pub fn execute_event<T>(memory: &Memory, function: &Function<'_>, event: &T) -> Result<T::Response, &'static str> where T: events::Event{
    execute(&memory,&function,&event)
}

pub fn read_action(ctx: &mut Ctx, ptr: u32, len: u32) {
    let memory = ctx.memory(0);

    let memory = memory.view::<u8>();

    let str_slice = &memory[ptr as usize..(ptr + len) as usize];

    let bytes: Vec<u8> = str_slice.iter().map(|x| x.get()).collect();

    let e: Option<Vec<Action>> = bincode::deserialize(&bytes).ok();

    println!("Actions sended: {:?}",e);

    // TODO: Handle actions
}