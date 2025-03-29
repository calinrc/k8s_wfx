use super::FindDataUpdater;
use crate::iterators::{K8sAsyncResource, K8sNamespaceResourceIterator, ResourceData};
use hyper_util::rt::TokioExecutor;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, Client, Config, ResourceExt, client::ConfigExt};
use tower::{BoxError, ServiceBuilder};

pub struct DeploymentsIterator {
    it: Box<std::vec::IntoIter<Deployment>>,
    next_elem: Option<Deployment>,
}

impl Drop for DeploymentsIterator {
    fn drop(&mut self) {
        println!("Drop DeploymentsIterator")
    }
}

impl Iterator for DeploymentsIterator {
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

impl FindDataUpdater for DeploymentsIterator {
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

impl K8sAsyncResource<Deployment> for DeploymentsIterator {}

impl K8sNamespaceResourceIterator<Deployment> for DeploymentsIterator {
    async fn list_namespace_resources(
        config_name: &str,
        namespace: &str,
    ) -> anyhow::Result<Vec<Deployment>> {
        let client = Self::create_kube_client(config_name, Some(namespace)).await?;
        let mut vec = Vec::new();
        let res_api: Api<Deployment> = Api::namespaced(client, &namespace);
        for p in res_api.list(&Default::default()).await? {
            vec.push(p.clone());
            //info!("{}", p.name_any());
        }
        Ok(vec)
    }
}

impl DeploymentsIterator {
    pub fn new(config_name: &str, namespace: &str) -> Box<Self> {
        let v = Self::get_resources(config_name, namespace);
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
}
