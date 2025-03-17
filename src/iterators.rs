use crate::consts;
use crate::consts::FILETIME;
use crate::consts::WIN32_FIND_DATAA;
use crate::iterators::configs::ConfigsIterator;
use crate::iterators::dummy::DummyIterator;
use crate::iterators::namespaces::NamespacesIterator;
use crate::iterators::pods::PodsIterator;
use crate::resources;
use core::future::Future;
use std::path::{Component, Path};
use std::slice::Iter;

pub mod configs;
pub mod dummy;
mod namespaces;
pub mod pods;

#[derive(Debug, Default)]
pub struct ReasourceData;

pub trait FindDataUpdater: Iterator<Item = ReasourceData> {
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA);
}

pub struct ResourcesIteratorFactory;

impl ResourcesIteratorFactory {
    pub fn new(_path: &Path) -> Box<dyn FindDataUpdater> {
        // return res_iter;

        // if _path.parent().is_none() {
        //     Box::new(BaseResourcesIterator {
        //         it: Box::new(resources::K8SResources::iterator()),
        //         next_elem: None,
        //     })
        // } else {
        //     let mut components = _path.components();
        //     let _rd = components.next(); //root dir
        //     let fp = components.next(); // first part
        //     let iterator_info = fp
        //         .map(|c| c.as_os_str().to_str())
        //         .flatten()
        //         .map(|res_name| resources::K8SResources::from_str(res_name))
        //         .flatten()
        //         .and_then(|res| match res {
        //             resources::K8SResources::Pod => Some(PodsIterator::new()),
        //             _ => None,
        //         });
        //
        //     if let Some(it) = iterator_info {
        //         it
        //     } else {
        //         Box::new(DummyIterator {})
        //     }
        // }

        let filtered_components = _path.components().filter(|c| match c {
            Component::Normal(_) => true,
            _ => false,
        });
        let components = filtered_components.collect::<Vec<_>>();
        let c2 = components.len();

        let comp_count = c2;
        match comp_count {
            0 => ConfigsIterator::new(),
            1 => Self::handle_one_component(components),
            2 => Self::handle_two_components(components),
            _ => DummyIterator::new(),
        }
    }

    fn handle_one_component(components: Vec<Component>) -> Box<dyn FindDataUpdater> {
        let component = components[0];
        match component {
            Component::Normal(_) => BaseResourcesIterator::new(),
            _ => DummyIterator::new(),
        }
    }

    fn handle_two_components(components: Vec<Component>) -> Box<dyn FindDataUpdater> {
        let config_part = components[0];
        let resource_part = components[1];
        match resource_part {
            Component::Normal(res_name) => {
                match resources::K8SResources::from_str(res_name.to_str().unwrap()) {
                    Some(resources::K8SResources::Pod) => PodsIterator::new(),
                    Some(resources::K8SResources::Namespace) => NamespacesIterator::new(),
                    _ => DummyIterator::new()
                }

            },
            _ => DummyIterator::new()
        }

        // let iterator_info = resource_part
        //     .map(|c| c.as_os_str().to_str())
        //     .flatten()
        //     .map(|res_name| resources::K8SResources::from_str(res_name))
        //     .flatten()
        //     .and_then(|res| {
        //         let fupd: Option<Box<dyn FindDataUpdater>> = match res {
        //             resources::K8SResources::Pod => Some(PodsIterator::new()),
        //             resources::K8SResources::Namespace => Some(NamespacesIterator::new()),
        //             _ => None,
        //         };
        //         fupd
        //     });
        // iterator_info.unwrap_or_else(|| DummyIterator::new())
    }
}

#[derive(Debug)]
pub struct BaseResourcesIterator<'a> {
    it: Box<Iter<'static, resources::K8SResources>>,
    next_elem: Option<&'a resources::K8SResources>,
}

impl BaseResourcesIterator<'_> {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            it: Box::new(resources::K8SResources::iterator()),
            next_elem: None,
        });
    }
}
impl Iterator for BaseResourcesIterator<'_> {
    type Item = ReasourceData;
    fn next(&mut self) -> Option<ReasourceData> {
        let result = self.it.next();
        self.next_elem = result;

        self.next_elem.map(|_| ReasourceData::default())
    }
}

impl Drop for BaseResourcesIterator<'_> {
    fn drop(&mut self) {
        println!("Drop BaseResourcesIterator")
    }
}

impl FindDataUpdater for BaseResourcesIterator<'_> {
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        match self.next_elem {
            Some(res) => {
                unsafe { *find_data }.dw_file_attributes =
                    consts::FILE_ATTRIBUTE_UNIX_MODE | consts::FILE_ATTRIBUTE_DIRECTORY;
                unsafe { *find_data }.ft_creation_time = FILETIME::default();
                unsafe { *find_data }.ft_last_access_time = FILETIME::default();
                unsafe { *find_data }.ft_last_write_time = FILETIME::default();
                unsafe { *find_data }.n_file_size_high = 0;
                unsafe { *find_data }.n_file_size_low = 0;
                unsafe { *find_data }.dw_reserved_0 = 0;
                unsafe { *find_data }.dw_reserved_1 = 0;
                let res_str = res.as_res_str();
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

                println!("K8SResources {}", res)
            }
            None => {
                eprint!("Unable to update_find_data. None resource")
            }
        }
    }
}

trait K8sResourceIterator<T> {
    fn async_to_sync_res(
        future: impl Future<Output = anyhow::Result<Vec<T>>>,
    ) -> anyhow::Result<Vec<T>> {
        let runtime_res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();
        runtime_res?.block_on(future)
        // let _ = match runtime_res {
        //     Ok(runtime) => { _= runtime.block_on(future);
        //                         println!("Done listing pods")},
        //     Err(err) => panic!("Problem opening the file: {:?}", err),
        // };

        // Ok(Vec::new())
    }
}
