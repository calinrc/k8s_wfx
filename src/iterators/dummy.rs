use crate::consts::WIN32_FIND_DATAA;
use crate::iterators::ResourceData;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;

use super::FsDataHandler;

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
impl FsDataHandler for DummyIterator {
    fn creation_time(&self) -> Option<Time> {
        None
    }

    fn artifact_name(&self) -> String {
        String::from("")
    }

    fn has_next(&self) -> bool {
        false
    }

    unsafe fn update_find_data(&self, _find_data: *mut WIN32_FIND_DATAA) {}
}
