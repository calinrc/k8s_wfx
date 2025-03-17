use super::FindDataUpdater;
use crate::consts;
use crate::consts::FILETIME;
use crate::consts::WIN32_FIND_DATAA;
use crate::iterators::{K8sResourceIterator, ReasourceData};
use kube::config::Kubeconfig;
use kube::config::NamedContext;

pub struct ConfigsIterator {
    it: Box<std::vec::IntoIter<NamedContext>>,
    next_elem: Option<NamedContext>,
}

impl Drop for ConfigsIterator {
    fn drop(&mut self) {
        println!("Drop ConfigsIterator")
    }
}

impl Iterator for ConfigsIterator {
    type Item = ReasourceData;
    fn next(&mut self) -> Option<ReasourceData> {
        let result = self.it.next();
        self.next_elem = result;
        if !self.next_elem.is_none() {
            Some(ReasourceData::default())
        } else {
            None
        }

        //self.next_elem.map(|_| ReasourceData::default())
    }
}

impl FindDataUpdater for ConfigsIterator {
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        match &self.next_elem {
            Some(next_elem) => {
                let ct_unwrap = FILETIME::default();

                unsafe { *find_data }.dw_file_attributes = consts::FILE_ATTRIBUTE_UNIX_MODE;
                unsafe { *find_data }.ft_creation_time = ct_unwrap;
                unsafe { *find_data }.ft_last_access_time = ct_unwrap;
                unsafe { *find_data }.ft_last_write_time = ct_unwrap;
                unsafe { *find_data }.n_file_size_high = 0;
                unsafe { *find_data }.n_file_size_low = 0;
                unsafe { *find_data }.dw_reserved_0 = 0;
                unsafe { *find_data }.dw_reserved_1 = 0;
                let res_str = &next_elem.name;
                let bytes = res_str.as_bytes();
                let len = bytes.len();

                unsafe {
                    std::ptr::copy(
                        bytes.as_ptr().cast(),
                        (*find_data).c_file_name.as_mut_ptr(),
                        consts::MAX_PATH,
                    )
                };
                unsafe {
                    std::ptr::write(
                        (*find_data).c_file_name.as_mut_ptr().offset(len as isize) as *mut u8,
                        0u8,
                    )
                };

                //(*find_data).c_file_name= [0i8;260];
                unsafe { *find_data }.c_alternate_file_name = [0i8; 14];

                println!("Config resource {}", res_str)
            }
            None => println!("update_find_data on None Pods"),
        }
    }
}

impl K8sResourceIterator<NamedContext> for ConfigsIterator {}

impl ConfigsIterator {
    pub fn new() -> Box<Self> {
        let v = Self::get_resources();
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }

    fn get_resources() -> Vec<NamedContext> {
        let vec_empt: Vec<NamedContext> = Vec::new();

        let runtime_res: Result<Vec<NamedContext>, anyhow::Error> =
            Self::async_to_sync_res(list_configs());
        match runtime_res {
            Ok(vec) => vec,
            Err(_err) => {
                eprintln!("Fail on getting pods list {}", _err.to_string());
                vec_empt
            }
        }
    }
}

pub async fn list_configs() -> anyhow::Result<Vec<NamedContext>> {
    let config = Kubeconfig::read()?;
    let vec = config.contexts.iter().map(|ctx| ctx.clone()).collect();
    Ok(vec)
}
