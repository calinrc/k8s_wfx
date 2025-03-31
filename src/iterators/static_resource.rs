use crate::consts::{FILETIME, WIN32_FIND_DATAA};
use crate::iterators::{FsDataHandler, ResourceData};
use crate::{consts, resources};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use std::slice::Iter;

#[derive(Debug)]
pub struct StaticListResourcesIterator<'a> {
    it: Box<Iter<'static, resources::K8SResources>>,
    next_elem: Option<&'a resources::K8SResources>,
}

impl StaticListResourcesIterator<'_> {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            it: Box::new(resources::K8SResources::iterator()),
            next_elem: None,
        });
    }
}
impl Iterator for StaticListResourcesIterator<'_> {
    type Item = ResourceData;
    fn next(&mut self) -> Option<ResourceData> {
        let result = self.it.next();
        self.next_elem = result;

        self.next_elem.map(|_| ResourceData::default())
    }
}

impl Drop for StaticListResourcesIterator<'_> {
    fn drop(&mut self) {
        println!("Drop BaseResourcesIterator")
    }
}

impl FsDataHandler for StaticListResourcesIterator<'_> {
    fn creation_time(&self) -> Option<Time> {
        None
    }

    fn artifact_name(&self) -> String {
        String::from("")
    }

    fn has_next(&self) -> bool {
        false
    }

    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        unsafe {
            match self.next_elem {
                Some(res) => {
                    (*find_data).dw_file_attributes =
                        consts::FILE_ATTRIBUTE_UNIX_MODE | consts::FILE_ATTRIBUTE_DIRECTORY;
                    (*find_data).ft_creation_time = FILETIME::default();
                    (*find_data).ft_last_access_time = FILETIME::default();
                    (*find_data).ft_last_write_time = FILETIME::default();
                    (*find_data).n_file_size_high = 0;
                    (*find_data).n_file_size_low = 0;
                    (*find_data).dw_reserved_0 = consts::S_IFDIR;
                    (*find_data).dw_reserved_1 = 0;
                    let res_str = res.as_res_str();
                    let bytes = res_str.as_bytes();
                    let len = bytes.len();

                    std::ptr::copy(
                        bytes.as_ptr().cast(),
                        (*find_data).c_file_name.as_mut_ptr(),
                        consts::MAX_PATH,
                    );
                    std::ptr::write(
                        (*find_data).c_file_name.as_mut_ptr().offset(len as isize) as *mut u8,
                        0u8,
                    );

                    //(*find_data).c_file_name= [0i8;260];
                    (*find_data).c_alternate_file_name = [0i8; 14];

                    println!("K8SResources {}", res)
                }
                None => {
                    eprint!("Unable to update_find_data. None resource")
                }
            }
        }
    }
}
