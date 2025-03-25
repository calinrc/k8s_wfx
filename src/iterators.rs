use crate::consts::{FILETIME, WIN32_FIND_DATAA};
use crate::iterators::configs::ConfigsIterator;
use crate::iterators::deployments::DeploymentsIterator;
use crate::iterators::dummy::DummyIterator;
use crate::iterators::jobs::JobsIterator;
use crate::iterators::namespaces::NamespacesIterator;
use crate::iterators::nodes::NodesIterator;
use crate::iterators::pods::PodsIterator;
use crate::iterators::static_resource::StaticListResourcesIterator;
use crate::{consts, helper, resources};
use core::future::Future;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use std::path::{Component, Path};

pub mod configs;
mod deployments;
pub mod dummy;
mod jobs;
mod namespaces;
mod nodes;
pub mod pods;
mod static_resource;

#[derive(Debug, Default)]
pub struct ResourceData;

pub trait FindDataUpdater: Iterator<Item = ResourceData> {
    fn creation_time(&self) -> Option<Time>;

    fn artifact_name(&self) -> String;

    fn has_next(&self) -> bool;

    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        if self.has_next() {
            let creation_time_opt = self.creation_time();
            let name = self.artifact_name();
            let ct = creation_time_opt
                .clone()
                .map(|ts| helper::to_split_file_time(ts.0.timestamp_millis()))
                .map(|(l, h)| FILETIME::new(l as u32, h as u32))
                .unwrap_or(FILETIME::default());
            (*find_data).dw_file_attributes = consts::FILE_ATTRIBUTE_UNIX_MODE;
            (*find_data).ft_creation_time = ct;
            (*find_data).ft_last_access_time = ct;
            (*find_data).ft_last_write_time = ct;
            (*find_data).n_file_size_high = 0;
            (*find_data).n_file_size_low = 0;
            (*find_data).dw_reserved_0 = 0;
            (*find_data).dw_reserved_1 = 0;
            let bytes = name.as_bytes();
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
            (*find_data).c_alternate_file_name = [0i8; 14];

            println!("Resource {}", name)
        } else {
            println!("update_find_data on None")
        }
    }
}

pub struct ResourcesIteratorFactory;

impl ResourcesIteratorFactory {
    pub fn new(_path: &Path) -> Box<dyn FindDataUpdater> {
        let filtered_components = _path.components().filter(|c| match c {
            Component::Normal(_) => true,
            _ => false,
        });
        let components = filtered_components.collect::<Vec<_>>();
        let comp_count = components.len();
        match comp_count {
            0 => ConfigsIterator::new(),
            1 => Self::handle_resources_component(components),
            2 => Self::handle_detailed_component(components),
            _ => DummyIterator::new(),
        }
    }

    fn handle_resources_component(components: Vec<Component>) -> Box<dyn FindDataUpdater> {
        let component = components[0];
        match component {
            Component::Normal(_) => StaticListResourcesIterator::new(),
            _ => DummyIterator::new(),
        }
    }

    fn handle_detailed_component(components: Vec<Component>) -> Box<dyn FindDataUpdater> {
        let _config_part = components[0];
        let resource_part = components[1];
        let ns = String::from("kube-system");
        match resource_part {
            Component::Normal(res_name) => {
                match resources::K8SResources::from_str(res_name.to_str().unwrap()) {
                    Some(resources::K8SResources::Pod) => PodsIterator::new(ns.as_str()),
                    Some(resources::K8SResources::Namespace) => NamespacesIterator::new(),
                    Some(resources::K8SResources::Node) => NodesIterator::new(),
                    Some(resources::K8SResources::Job) => JobsIterator::new(ns.as_str()),
                    Some(resources::K8SResources::Deployment) => {
                        DeploymentsIterator::new(ns.as_str())
                    }

                    _ => DummyIterator::new(),
                }
            }
            _ => DummyIterator::new(),
        }
    }
}

trait K8sAsyncResource<T> {
    fn async_to_sync_res(
        future: impl Future<Output = anyhow::Result<Vec<T>>>,
    ) -> anyhow::Result<Vec<T>> {
        let runtime_res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();
        runtime_res?.block_on(future)
    }
}

trait K8sClusterResourceIterator<T>: K8sAsyncResource<T> {
    async fn list_cluster_resources() -> anyhow::Result<Vec<T>>;

    fn get_resources() -> Vec<T> {
        let vec_empt: Vec<T> = Vec::new();

        let runtime_res = Self::async_to_sync_res(Self::list_cluster_resources());
        match runtime_res {
            Ok(vec) => vec,
            Err(_err) => {
                eprintln!(
                    "Fail on getting cluster bound resource list {}",
                    _err.to_string()
                );
                vec_empt
            }
        }
    }
}

trait K8sNamespaceResourceIterator<T>: K8sAsyncResource<T> {
    async fn list_namespace_resources(namespace: &str) -> anyhow::Result<Vec<T>>;

    fn get_resources(namespace: &str) -> Vec<T> {
        let vec_empt: Vec<T> = Vec::new();

        let runtime_res = Self::async_to_sync_res(Self::list_namespace_resources(namespace));
        match runtime_res {
            Ok(vec) => vec,
            Err(_err) => {
                eprintln!(
                    "Fail on getting namespace bound resource list {}",
                    _err.to_string()
                );
                vec_empt
            }
        }
    }
}
