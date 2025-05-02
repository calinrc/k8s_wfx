use crate::consts::{FILETIME, WIN32_FIND_DATAA};
use crate::iterators::configs::ConfigsIterator;
use crate::iterators::deployments::DeploymentsIterator;
use crate::iterators::dummy::DummyIterator;
use crate::iterators::jobs::JobsIterator;
use crate::iterators::namespaces::NamespacesIterator;
use crate::iterators::nodes::NodesIterator;
use crate::iterators::pods::PodsIterator;
use crate::iterators::static_resource::StaticListResourcesIterator;
use crate::shareddata::GLOBAL_SHARED_DATA;
use crate::{consts, helper, resources};
use core::future::Future;
use hyper_util::rt::TokioExecutor;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::client::ConfigExt;
use kube::config::KubeconfigError;
use kube::{
    Client,
    config::{Config, KubeConfigOptions, Kubeconfig},
};
use std::path::{Component, Path};
use tower::{BoxError, ServiceBuilder};

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

pub trait FsDataHandler: Iterator<Item = ResourceData> {
    fn creation_time(&self) -> Option<Time>;

    fn artifact_name(&self) -> String;

    fn has_next(&self) -> bool;

    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        unsafe {
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

                println!("Resource {}", name)
            } else {
                println!("update_find_data on None")
            }
        }
    }

    fn fs_execute(&self, path: &Path, verb: &str) -> anyhow::Result<()>{
        eprintln!(
            "execute path {} verb {}",
            path.to_str().unwrap_or("unknown"),
            verb
        );
        Ok(())
    }

    fn fs_get(&self, remote_path: &Path, local_path: &Path, flags:i32) -> anyhow::Result<()> {
        eprintln!(
            "get remote path {} using local  {} and flags {}",
            remote_path.to_str().unwrap_or("unknown"),
            local_path.to_str().unwrap_or("unknown"),
            flags
        );
        Ok(())
    }

    fn fs_put(&self, remote_path: &Path, local_path: &Path, flags:i32) -> anyhow::Result<()> {
        eprintln!(
            "put remote path {} using local  {} and flags {}",
            remote_path.to_str().unwrap_or("unknown"),
            local_path.to_str().unwrap_or("unknown"),
            flags
        );
        Ok(())
    }

    fn fs_delete(&self, remote_path: &Path) -> anyhow::Result<()> {
        eprintln!(
            "delete remote path {}",
            remote_path.to_str().unwrap_or("unknown"),
        );
        Ok(())
    }
}

pub struct ResourcesIteratorFactory;

impl ResourcesIteratorFactory {
    pub fn new(_path: &Path) -> Box<dyn FsDataHandler> {
        let components = helper::path_components(_path);
        let comp_count = components.len();
        match comp_count {
            0 => ConfigsIterator::new(),
            1 => Self::handle_resources_component(&components),
            _ => Self::handle_detailed_component(&components),
            // _ => DummyIterator::new(),
        }
    }

    fn handle_resources_component(components: &Vec<Component>) -> Box<dyn FsDataHandler> {
        let component = components[0];
        match component {
            Component::Normal(_) => StaticListResourcesIterator::new(),
            _ => DummyIterator::new(),
        }
    }

    fn handle_detailed_component(components: &Vec<Component>) -> Box<dyn FsDataHandler> {
        let _config_part = components[0];
        let resource_part = components[1];
        // let ns = String::from("kube-system");
        let shared_data = GLOBAL_SHARED_DATA.lock().unwrap();
        let ns = shared_data.selected_namespace.clone();
        match (resource_part, _config_part) {
            (Component::Normal(res_name), Component::Normal(config_name)) => {
                let conf_name = config_name.to_str().unwrap();
                match resources::K8SResources::from_str(res_name.to_str().unwrap()) {
                    Some(resources::K8SResources::Pod) => PodsIterator::new(conf_name, ns.as_str()),
                    Some(resources::K8SResources::Namespace) => NamespacesIterator::new(conf_name),
                    Some(resources::K8SResources::Node) => NodesIterator::new(conf_name),
                    Some(resources::K8SResources::Job) => JobsIterator::new(conf_name, ns.as_str()),
                    Some(resources::K8SResources::Deployment) => {
                        DeploymentsIterator::new(conf_name, ns.as_str())
                    }

                    _ => DummyIterator::new(),
                }
            }
            _ => DummyIterator::new(),
        }
    }
}

trait K8sAsyncResource<T> {
    fn async_to_sync_res<V>(
        future: impl Future<Output = anyhow::Result<V>>,
    ) -> anyhow::Result<V> {
        let runtime_res = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();
        runtime_res?.block_on(future)
    }

    async fn create_config_from_named_context(
        kubeconfig: Kubeconfig,
        config_name: &str,
    ) -> Result<Config, KubeconfigError> {
        // Find the NamedContext with the matching name
        let _named_context = kubeconfig
            .contexts
            .iter()
            .find(|nc| nc.name == config_name)
            .ok_or_else(|| KubeconfigError::LoadClusterOfContext(config_name.to_string()))?;

        let options = KubeConfigOptions {
            context: Some(config_name.to_string()),
            cluster: None, // We'll use the cluster from the context
            user: None,    // We'll use the user from the context
        };

        let mut config = Config::from_kubeconfig(&options).await?;
        config.apply_debug_overrides();
        Ok(config)
    }

    async fn create_kube_client(
        config_name: &str,
        namespace: Option<&str>,
    ) -> anyhow::Result<Client> {
        let kube_config = Kubeconfig::read()?;
        let config = Self::create_config_from_named_context(kube_config, config_name).await?;
        let https = config.openssl_https_connector()?;
        let service = ServiceBuilder::new()
            .layer(config.base_uri_layer())
            .option_layer(config.auth_layer()?)
            .map_err(BoxError::from)
            .service(
                hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https),
            );
        let client = match namespace {
            Some(ns) => Client::new(service, ns),
            _ => Client::new(service, &config.default_namespace),
        };
        Ok(client)
    }
}

trait K8sClusterResourceIterator<T>: K8sAsyncResource<T> {
    async fn list_cluster_resources(config_name: &str) -> anyhow::Result<Vec<T>>;

    fn get_resources(config_name: &str) -> Vec<T> {
        let vec_empt: Vec<T> = Vec::new();

        let runtime_res = Self::async_to_sync_res(Self::list_cluster_resources(config_name));
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
    async fn list_namespace_resources(config_name: &str, namespace: &str)
    -> anyhow::Result<Vec<T>>;

    fn get_resources(config_name: &str, namespace: &str) -> Vec<T> {
        let vec_empt: Vec<T> = Vec::new();

        let runtime_res =
            Self::async_to_sync_res(Self::list_namespace_resources(config_name, namespace));
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
