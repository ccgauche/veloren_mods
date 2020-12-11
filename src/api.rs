use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use serde::de::DeserializeOwned;
use wasmer_runtime::*;

use veloren_api_common::{events, Action};

pub struct Plugin {
    name: String,
    wasm_instance: Instance,
    events: Vec<String>,
}

impl Plugin {
    pub fn new(name: String, bytes: &[u8]) -> Self {
        let module = compile(&bytes).expect("Can't compile WASM");
        let instance = module
            .instantiate(&imports! {"env" => {
                "send_action" => func!(read_action),
            }})
            .expect("Can't import globals and instanciate WASM");
        Self {
            name,
            events: instance.exports.into_iter().map(|(name, _)| name).collect(),
            wasm_instance: instance,
        }
    }

    pub fn try_execute<T>(
        &self,
        event_name: &str,
        request: &PreparedEventQuery<T>,
    ) -> Option<T::Response>
    where
        T: events::Event,
    {
        flame::start("try execute");
        flame::start("check function names");
        if !self.events.iter().any(|x| x == event_name) {
            return None;
        }
        flame::end("check function names");
        flame::start("get function function");
        let func = self.wasm_instance.exports.get(event_name).ok()?;
        flame::end("get function function");
        flame::start("get memory");
        let mem = self.wasm_instance.context().memory(0);
        flame::end("get memory");
        flame::start("execute raw");
        let bytes = execute_raw(&mem, &func, &request.bytes).ok()?;
        flame::end("execute raw");
        flame::start("execute deserialize");
        let tmp = bincode::deserialize(&bytes).ok();
        flame::end("execute deserialize");
        flame::end("try execute");
        tmp
    }
}

pub struct PreparedEventQuery<T> {
    bytes: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T: events::Event> PreparedEventQuery<T> {
    pub fn new(event: &T) -> Option<Self>
    where
        T: events::Event,
    {
        Some(Self {
            bytes: bincode::serialize(&event).ok()?,
            _phantom: PhantomData::default(),
        })
    }
}

pub type Function<'a> = Func<'a, (i32, u32), i32>;

const MEMORY_POS: usize = 100000;

pub fn execute<'a, T>(
    memory: &Memory,
    function: &Function,
    input: &impl serde::Serialize,
) -> Result<T, &'static str>
where
    T: DeserializeOwned,
{
    let bytes =
        bincode::serialize(&input).map_err(|_| "Failed to serialize structure with bincode")?;
    let updated_bytes = execute_raw(memory, function, &bytes)?;
    bincode::deserialize(&updated_bytes).map_err(|_| "Failed to convert wasm memory to output")
}

pub fn execute_raw(
    memory: &Memory,
    function: &Function,
    bytes: &[u8],
) -> Result<Vec<u8>, &'static str> {
    flame::start("get memory view");
    let view = memory.view::<u8>();
    flame::end("get memory view");
    let len = bytes.len();
    flame::start("memory write");
    for (cell, byte) in view[MEMORY_POS..len + MEMORY_POS].iter().zip(bytes.iter()) {
        cell.set(*byte)
    }
    flame::end("memory write");
    flame::start("call");
    let start = function
        .call(MEMORY_POS as i32, len as u32)
        .expect("Failed to execute function") as usize;
    flame::end("call");
    flame::start("get new view");
    let view = memory.view::<u8>();
    flame::end("get new view");
    let mut new_len_bytes = [0u8; 4];
    flame::start("get new view");
    // It is probably better to dirrectly make the new_len_bytes
    for i in 0..4 {
        // Since memory view is a more than 11500 elements array we can get the [1;4] without any bound checks
        unsafe {
            new_len_bytes[i] = view.get_unchecked(i + 1).get();
        }
    }
    flame::end("get new view");
    let new_len = u32::from_ne_bytes(new_len_bytes) as usize;
    flame::start("view to data");
    let tmp = view[start..start + new_len]
        .iter()
        .map(|c| c.get())
        .collect();
    flame::end("view to data");
    Ok(tmp)
}

pub fn execute_event<T>(
    memory: &Memory,
    function: &Function<'_>,
    event: &T,
) -> Result<T::Response, &'static str>
where
    T: events::Event,
{
    execute(&memory, &function, &event)
}

pub fn read_action(ctx: &mut Ctx, ptr: u32, len: u32) {
    let memory = ctx.memory(0);

    let memory = memory.view::<u8>();

    let str_slice = &memory[ptr as usize..(ptr + len) as usize];

    let bytes: Vec<u8> = str_slice.iter().map(|x| x.get()).collect();

    let e: Option<Vec<Action>> = bincode::deserialize(&bytes).ok();

    println!("Actions sended: {:?}", e);

    // TODO: Handle actions
}
