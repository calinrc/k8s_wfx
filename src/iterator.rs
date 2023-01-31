use crate::resources;
use std::slice::Iter;
use crate::WIN32_FIND_DATAA;

#[derive(Debug)]
pub struct ResourcesIterator{
    it: Iter<'static, resources::K8SResources>
}


impl ResourcesIterator{

    pub fn new() -> Self {
        Self{
            it : resources::K8SResources::iterator()
        }
    }

    pub fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {

    }
}