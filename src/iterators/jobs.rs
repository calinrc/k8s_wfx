use super::FindDataUpdater;
use crate::iterators::{K8sAsyncResource, K8sNamespaceResourceIterator, ResourceData};
use hyper_util::rt::TokioExecutor;
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
use kube::{Api, Client, Config, ResourceExt, client::ConfigExt};
use std::iter::Iterator;
use tower::{BoxError, ServiceBuilder};

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

impl FindDataUpdater for JobsIterator {
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
    async fn list_namespace_resources(namespace: &str) -> anyhow::Result<Vec<Job>> {
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

        let client = Client::new(service, namespace);

        let res_api: Api<Job> = Api::namespaced(client, &namespace);
        for p in res_api.list(&Default::default()).await? {
            vec.push(p.clone());
            //info!("{}", p.name_any());
        }
        Ok(vec)
    }
}

impl JobsIterator {
    pub fn new(namespace: &str) -> Box<Self> {
        let v = Self::get_resources(namespace);
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
}
