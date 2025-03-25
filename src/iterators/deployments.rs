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
    async fn list_namespace_resources(namespace: &str) -> anyhow::Result<Vec<Deployment>> {
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

        let res_api: Api<Deployment> = Api::namespaced(client, &namespace);
        for p in res_api.list(&Default::default()).await? {
            vec.push(p.clone());
            //info!("{}", p.name_any());
        }
        Ok(vec)
    }
}

impl DeploymentsIterator {
    pub fn new(namespace: &str) -> Box<Self> {
        let v = Self::get_resources(namespace);
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
}
