use super::FindDataUpdater;
use crate::iterators::{K8sAsyncResource, K8sNamespaceResourceIterator, ResourceData};
use hyper_util::rt::TokioExecutor;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, Client, Config, ResourceExt, client::ConfigExt};
use std::time::Duration;
use tokio::time::timeout;
use tower::{BoxError, ServiceBuilder};

pub struct PodsIterator {
    it: Box<std::vec::IntoIter<Pod>>,
    next_elem: Option<Pod>,
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

impl FindDataUpdater for PodsIterator {
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
}
