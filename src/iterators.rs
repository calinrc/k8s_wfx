use crate::resources;
use std::slice::Iter;
use crate::consts::WIN32_FIND_DATAA;
use crate::consts::FILETIME;
use crate::consts;
use std::path::Path;

pub mod pods;

#[derive(Debug)]
#[derive(Default)]
pub struct ReasourceData;

pub trait FindDataUpdater{
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA);
}


trait FindDataUpdaterIterator: Iterator + FindDataUpdater {}

pub struct ResourcesItertatorFactory;

impl ResourcesItertatorFactory {
    pub fn new(path: &Path) -> ResourcesIterator {
        ResourcesIterator{
            it : Box::new(resources::K8SResources::iterator()),
            next_elem: None

        }
    }
}

#[derive(Debug)]
pub struct ResourcesIterator<'a>{
    
    it: Box<Iter<'static, resources::K8SResources>>,
    next_elem : Option<&'a resources::K8SResources>,

}

impl Iterator for ResourcesIterator<'_>{
    type Item = ReasourceData;
    fn next(&mut self) -> Option<ReasourceData> {
        let result = self.it.next();
        self.next_elem = result;

        if self.next_elem.is_none(){
            None
        }
        else{
            Some(ReasourceData::default())
        }
    }   
}


impl Drop for ResourcesIterator<'_> {
    fn drop(&mut self) {
        println!("Drop ResourcesIterator")
    }
}


impl FindDataUpdater for ResourcesIterator<'_>{

    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        match self.next_elem {
            Some(res) => {
                (*find_data).dw_file_attributes = consts::FILE_ATTRIBUTE_UNIX_MODE | consts::FILE_ATTRIBUTE_DIRECTORY;
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
            None => {
                eprint!("Unable to update_find_data. None resource")

            }
        }
        
    }
}