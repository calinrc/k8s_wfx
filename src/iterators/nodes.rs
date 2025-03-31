use super::FsDataHandler;
use crate::iterators::{K8sAsyncResource, K8sClusterResourceIterator, ResourceData};
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, ResourceExt};
use std::iter::Iterator;

// NodesIterator: Iterator for node, similar to PodIterator
pub struct NodesIterator {
    it: Box<std::vec::IntoIter<Node>>,
    next_elem: Option<Node>,
}

impl Drop for NodesIterator {
    fn drop(&mut self) {
        println!("Drop NodesIterator")
    }
}

impl Iterator for NodesIterator {
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

impl FsDataHandler for NodesIterator {
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

impl K8sAsyncResource<Node> for NodesIterator {}

impl K8sClusterResourceIterator<Node> for NodesIterator {
    async fn list_cluster_resources(config_name: &str) -> anyhow::Result<Vec<Node>> {
        let client = Self::create_kube_client(config_name, None).await?;
        let mut vec = Vec::new();
        let res_api: Api<Node> = Api::all(client);
        for p in res_api.list(&Default::default()).await? {
            vec.push(p.clone());
            //info!("{}", p.name_any());
        }
        Ok(vec)
    }
}

impl NodesIterator {
    pub fn new(config_name: &str) -> Box<Self> {
        let v = Self::get_resources(config_name);
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
}
