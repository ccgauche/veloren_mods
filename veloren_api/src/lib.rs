
pub extern crate veloren_proc;

pub extern crate veloren_api_common;

pub use veloren_api_common :: *;


pub use veloren_proc::*;

use serde::de::DeserializeOwned;
use serde::Serialize;

extern "C" {
    fn send_action(ptr: *const u8, len: usize);
}

pub fn send_actions(action: Vec<Action>) {
    let ret = bincode::serialize(&action).unwrap();
    unsafe {
        send_action(ret.as_ptr(), ret.len());
    }
}

#[derive(Serialize)]
pub enum Action {
    ServerClose,
    PlayerSendMessage(usize,String),
    KillEntity(usize)
}

pub fn read_input<T>(ptr: i32, len: u32) -> Result<T, &'static str> where T: DeserializeOwned{
    let slice = unsafe { 
        ::std::slice::from_raw_parts(ptr as _, len as _)
    };
    bincode::deserialize(slice).map_err(|_|"Failed to deserialize function input")
}

pub fn write_output(value: impl Serialize) -> i32 {
    let ret = bincode::serialize(&value).unwrap();
    let len = ret.len() as u32;
    unsafe {
        ::std::ptr::write(1 as _, len);
    }
    ret.as_ptr() as _
}