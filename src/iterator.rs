use crate::resources;
use std::slice::Iter;
use crate::WIN32_FIND_DATAA;
use core::ffi::CStr;

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

    pub fn iterator(&mut self) -> &mut std::slice::Iter<'static, resources::K8SResources>{
        &mut self.it
    }

    pub fn update_find_data( find_data: *mut WIN32_FIND_DATAA, res: &resources::K8SResources) {
        println!("K8SResources {}", res)
        
    }
}