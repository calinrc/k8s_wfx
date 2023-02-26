use crate::resources;
use std::slice::Iter;
use crate::consts::WIN32_FIND_DATAA;
use crate::consts::FILETIME;
use crate::consts;
use core::ffi::CStr;

#[derive(Debug)]
pub struct ResourcesIterator{
    it: Box<Iter<'static, resources::K8SResources>>,
    id:u32
}


impl Drop for ResourcesIterator {
    fn drop(&mut self) {
        println!("Drop ResourcesIterator")
    }
}

impl ResourcesIterator{

     pub fn new() -> Self {
        Self{
            it : Box::new(resources::K8SResources::iterator()),
            id : 123
        }
    }

    pub fn iterator(&mut self) -> &mut std::slice::Iter<'static, resources::K8SResources>{
        &mut *(self.it)
    }

    pub unsafe fn update_find_data( find_data: *mut WIN32_FIND_DATAA, res: &resources::K8SResources) {
        
        (*find_data).dw_file_attributes = consts::FILE_ATTRIBUTE_UNIX_MODE | consts::FILE_ATTRIBUTE_DIRECTORY;
;
        (*find_data).ft_creation_time = FILETIME::default();
        (*find_data).ft_last_access_time =  FILETIME::default();
        (*find_data).ft_last_write_time =  FILETIME::default();
        (*find_data).n_file_size_high =  0;
        (*find_data).n_file_size_low =  0;
        (*find_data).dw_reserved_0= 0;
        (*find_data).dw_reserved_1= 0;
        let res_str = res.to_string();
        let bytes =res_str.as_bytes();
        let len = bytes.len();

        std::ptr::copy(
            bytes.as_ptr().cast(),
            (*find_data).c_file_name.as_mut_ptr(),
            consts::MAX_PATH,
        );
        std::ptr::write((*find_data).c_file_name.as_mut_ptr().offset(len as isize) as *mut u8, 0u8);

        //(*find_data).c_file_name= [0i8;260];
        (*find_data).c_alternate_file_name =  [0i8;14];
    
        println!("K8SResources {}", res)
        
    }
}