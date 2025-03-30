use lazy_static::lazy_static;
use std::sync::Mutex;
use std::ffi::c_int;

#[derive(Debug, Clone, Default)]
pub struct SharedData {
    pub plugin_nr: c_int,
    pub selected_namespace: String,
}

impl SharedData {
    pub fn new() -> Self {
        SharedData {
            plugin_nr: 1,
            selected_namespace: String::from("default"),
        }
    }
}


lazy_static! {
    pub static ref GLOBAL_SHARED_DATA: Mutex<SharedData> = Mutex::new(SharedData::new());
}