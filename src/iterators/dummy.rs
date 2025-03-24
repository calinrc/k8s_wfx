use crate::consts::WIN32_FIND_DATAA;
use crate::iterators::ResourceData;

use super::FindDataUpdater;

pub struct DummyIterator {}

impl DummyIterator {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Drop for DummyIterator {
    fn drop(&mut self) {
        println!("Drop DummyIterator")
    }
}

impl Iterator for DummyIterator {
    type Item = ResourceData;
    fn next(&mut self) -> Option<ResourceData> {
        None
    }
}
impl FindDataUpdater for DummyIterator {
    unsafe fn update_find_data(&self, _find_data: *mut WIN32_FIND_DATAA) {}
}
