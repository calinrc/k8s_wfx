use super::FsDataHandler;
use crate::iterators::{K8sAsyncResource, K8sNamespaceResourceIterator, ResourceData};
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, ResourceExt};
use std::iter::Iterator;
use tokio::time::{Duration, timeout};

// JobsIterator: Iterator for job, similar to PodIterator
pub struct JobsIterator {
    it: Box<std::vec::IntoIter<Job>>,
    next_elem: Option<Job>,
}

impl Drop for JobsIterator {
    fn drop(&mut self) {
        println!("Drop JobsIterator")
    }
}

impl Iterator for JobsIterator {
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

impl FsDataHandler for JobsIterator {
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

impl K8sAsyncResource<Job> for JobsIterator {}

impl K8sNamespaceResourceIterator<Job> for JobsIterator {
    async fn list_namespace_resources(
        config_name: &str,
        namespace: &str,
    ) -> anyhow::Result<Vec<Job>> {
        let client = Self::create_kube_client(config_name, Some(namespace)).await?;
        let res_api: Api<Job> = Api::namespaced(client, &namespace);
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

impl JobsIterator {
    pub fn new(config_name: &str, namespace: &str) -> Box<Self> {
        let v = Self::get_resources(config_name, namespace);
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
}
