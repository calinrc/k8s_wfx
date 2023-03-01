use crate::consts;
use crate::consts::FILETIME;
use crate::consts::WIN32_FIND_DATAA;
use crate::iterators::ReasourceData;
use crate::resources;
use k8s_openapi::api::core::v1::Pod;
use kube::{client::ConfigExt, Api, Client, Config, ResourceExt};
use std::slice::Iter;

use super::FindDataUpdater;

pub struct PodsIterator{};

impl Iterator for PodsIterator {
    type Item = ReasourceData;
    fn next(&mut self) -> Option<ReasourceData> {
        None
    }
}
impl FindDataUpdater for PodsIterator {
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {}
}

pub async fn list_pods(namespace: String) -> anyhow::Result<Vec<String>> {
    let config = Config::infer().await?;
    let https = config.openssl_https_connector()?;
    let mut vec = Vec::new();
    let service = tower::ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .service(hyper::Client::builder().build(https));

    let client = Client::new(service, namespace);

    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&Default::default()).await? {
        vec.push(p.name_any());
        //info!("{}", p.name_any());
    }
    Ok(vec)
}
