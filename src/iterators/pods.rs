use crate::consts::WIN32_FIND_DATAA;
use crate::iterators::{ReasourceData, K8sResourceIterator};
use k8s_openapi::api::core::v1::Pod;
use kube::{client::ConfigExt, Api, Client, Config, ResourceExt};
use std::slice::Iter;

use super::FindDataUpdater;

pub struct PodsIterator{
    it: Box<std::vec::IntoIter<Pod>>,
    next_elem: Option<Pod>,
}

impl Drop for PodsIterator {
    fn drop(&mut self) {
        println!("Drop PodsIterator")
    }
}

impl Iterator for PodsIterator {
    type Item = ReasourceData;
    fn next(&mut self) -> Option<ReasourceData> {
        None
    }
}

impl FindDataUpdater for PodsIterator {
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {}
}

impl K8sResourceIterator<Pod> for PodsIterator {
    fn get_resources(namespace:String) -> Vec<Pod> {
        let vec_empt:Vec<Pod> = Vec::new();
        
        let runtime_res  = Self::async_to_sync_res(list_pods(namespace));
        match runtime_res {
            Ok(vec) => vec,
            Err(_err) => vec_empt,
        }
    }
}

impl PodsIterator{
    pub fn new() -> Box<Self>{
        let v = Self::get_resources(String::from("default"));
        Box::new(Self {
            it:Box::new(v.into_iter()),
            next_elem: None,
        })
        
    }
}




pub async fn list_pods(namespace: String) -> anyhow::Result<Vec<Pod>> {
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
        vec.push(p.clone());
        //info!("{}", p.name_any());
    }
    Ok(vec)
}
