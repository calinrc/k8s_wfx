use crate::resources;
use std::slice::Iter;

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
}