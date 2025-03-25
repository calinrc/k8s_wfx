use super::FindDataUpdater;
use crate::iterators::{K8sAsyncResource, K8sClusterResourceIterator, ResourceData};
use hyper_util::rt::TokioExecutor;
use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, Client, Config, ResourceExt, client::ConfigExt};
use std::iter::Iterator;
use tower::{BoxError, ServiceBuilder};

// NamespaceIterator: Iterator for namespaces, similar to PodIterator
pub struct NamespacesIterator {
    it: Box<std::vec::IntoIter<Namespace>>,
    next_elem: Option<Namespace>,
}

impl Drop for NamespacesIterator {
    fn drop(&mut self) {
        println!("Drop NamespacesIterator")
    }
}

impl Iterator for NamespacesIterator {
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

impl FindDataUpdater for NamespacesIterator {
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

impl K8sAsyncResource<Namespace> for NamespacesIterator {}

impl K8sClusterResourceIterator<Namespace> for NamespacesIterator {
    async fn list_cluster_resources() -> anyhow::Result<Vec<Namespace>> {
        let config = Config::infer().await?;
        let https = config.openssl_https_connector()?;
        let mut vec = Vec::new();
        let service = ServiceBuilder::new()
            .layer(config.base_uri_layer())
            .option_layer(config.auth_layer()?)
            .map_err(BoxError::from)
            .service(
                hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https),
            );
        // .service(hyper::Client::builder().build(https));

        let client = Client::new(service, &config.default_namespace);

        let res_api: Api<Namespace> = Api::all(client);
        for p in res_api.list(&Default::default()).await? {
            vec.push(p.clone());
            //info!("{}", p.name_any());
        }
        Ok(vec)
    }
}

impl NamespacesIterator {
    pub fn new() -> Box<Self> {
        let v = Self::get_resources();
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
}
