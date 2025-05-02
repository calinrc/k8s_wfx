use std::path::{Component, Path};
use super::FsDataHandler;
use crate::iterators::{K8sAsyncResource, K8sNamespaceResourceIterator, ResourceData};
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, Discovery, ResourceExt};
use std::time::Duration;
use kube::discovery::{ApiCapabilities, ApiResource};
use tokio::time::timeout;
use crate::helper;
use crate::shareddata::GLOBAL_SHARED_DATA;

pub struct PodsIterator {
    it: Box<std::vec::IntoIter<Pod>>,
    next_elem: Option<Pod>,
}

fn resolve_api_resource(discovery: &Discovery, name: &str) -> Option<(ApiResource, ApiCapabilities)> {
    // iterate through groups to find matching kind/plural names at recommended versions
    // and then take the minimal match by group.name (equivalent to sorting groups by group.name).
    // this is equivalent to kubectl's api group preference
    discovery
        .groups()
        .flat_map(|group| {
            group
                .resources_by_stability()
                .into_iter()
                .map(move |res| (group, res))
        })
        .filter(|(_, (res, _))| {
            // match on both resource name and kind name
            // ideally we should allow shortname matches as well
            name.eq_ignore_ascii_case(&res.kind) || name.eq_ignore_ascii_case(&res.plural)
        })
        .min_by_key(|(group, _res)| group.name())
        .map(|(_, res)| res)
}

impl Drop for PodsIterator {
    fn drop(&mut self) {
        println!("Drop PodsIterator")
    }
}

impl Iterator for PodsIterator {
    type Item = ResourceData;
    fn next(&mut self) -> Option<ResourceData> {
        let result = self.it.next();
        self.next_elem = result;
        if !self.next_elem.is_none() {
            Some(ResourceData::default())
        } else {
            None
        }
    }
}

impl FsDataHandler for PodsIterator {
    fn creation_time(&self) -> Option<Time> {
        self.next_elem.as_ref()?.creation_timestamp()
    }

    fn artifact_name(&self) -> String {
        self.next_elem
            .as_ref()
            .map(|e| e.name_any())
            .unwrap_or(String::from(""))
    }

    fn has_next(&self) -> bool {
        !self.next_elem.is_none()
    }

    fn fs_get(&self, remote_path: &Path, local_path: &Path, flags:i32) -> anyhow::Result<()> {
        eprintln!(
            "get remote path {} using local  {} and flags {}",
            remote_path.to_str().unwrap_or("unknown"),
            local_path.to_str().unwrap_or("unknown"),
            flags
        );
        Self::async_to_sync_res(self.fs_get_async(remote_path))
    }

   
}

impl K8sAsyncResource<Pod> for PodsIterator {}

impl K8sNamespaceResourceIterator<Pod> for PodsIterator {
    async fn list_namespace_resources(
        config_name: &str,
        namespace: &str,
    ) -> anyhow::Result<Vec<Pod>> {
        let client = Self::create_kube_client(config_name, Some(namespace)).await?;
        let res_api: Api<Pod> = Api::namespaced(client, &namespace);
        let timeout_duration = Duration::from_secs(10);
        match timeout(timeout_duration, res_api.list(&Default::default())).await {
            Ok(Ok(job_list)) => {
                // The API call succeeded within the timeout
                let mut vec = Vec::new();
                for p in job_list {
                    vec.push(p.clone());
                }
                Ok(vec)
            }
            Ok(Err(e)) => {
                // The API call failed within the timeout
                Err(e.into()) // Convert kube::Error to anyhow::Error
            }
            Err(_elapsed) => {
                // The timeout elapsed
                Err(anyhow::anyhow!(
                    "Timeout while listing jobs in namespace '{}'",
                    namespace
                ))
            }
        }
    }
}

impl PodsIterator {
    pub fn new(config_name: &str, namespace: &str) -> Box<Self> {
        let v = Self::get_resources(config_name, namespace);
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }


    async fn fs_get_async(&self, remote_path: &Path,)-> anyhow::Result<()> {
        let components = helper::path_components(remote_path);
        let comp_count = components.len();
        let shared_data = GLOBAL_SHARED_DATA.lock().unwrap();
        let ns = shared_data.selected_namespace.clone();
        if comp_count >2 {
            let config_name_component = components[0];
            let pod_name = components[2];
            match (pod_name, config_name_component){
                (Component::Normal(name), Component::Normal(config_name)) => {
                    let conf_name = config_name.to_str().unwrap();
                    let client = Self::create_kube_client(conf_name, Some(ns.as_str())).await?;
                    let res_api: Api<Pod> = Api::namespaced(client, &ns);
                    let timeout_duration = Duration::from_secs(10);
                    match timeout(timeout_duration, res_api.list(&Default::default())).await {
                        Ok(Ok(job_list)) => {
                            // The API call succeeded within the timeout
                            let mut vec = Vec::new();
                            for p in job_list {
                                vec.push(p.clone());
                            }
                            Ok(())
                        }
                        Ok(Err(e)) => {
                            // The API call failed within the timeout
                            Err(e.into()) // Convert kube::Error to anyhow::Error
                        }
                        Err(_elapsed) => {
                            // The timeout elapsed
                            Err(anyhow::anyhow!(
                    "Timeout while listing jobs in namespace '{}'",
                    ns
                ))
                        }
                    }
                },
                _ => Ok(())
            }
        }else{
            Ok(())
        }
    }
    
}
