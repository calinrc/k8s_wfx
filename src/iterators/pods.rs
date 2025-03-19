use crate::consts;
use crate::consts::FILETIME;
use crate::consts::WIN32_FIND_DATAA;
use crate::helper;
use crate::iterators::{K8sResourceIterator, ReasourceData};
use hyper_util::rt::TokioExecutor;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, Config, ResourceExt, client::ConfigExt};
use tower::{BoxError, ServiceBuilder};

use super::FindDataUpdater;

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
    type Item = ReasourceData;
    fn next(&mut self) -> Option<ReasourceData> {
        let result = self.it.next();
        self.next_elem = result;
        if !self.next_elem.is_none() {
            Some(ReasourceData::default())
        } else {
            None
        }
    }
}

impl FindDataUpdater for PodsIterator {
    unsafe fn update_find_data(&self, find_data: *mut WIN32_FIND_DATAA) {
        match &self.next_elem {
            Some(next_elem) => {
                let creation_time_opt = next_elem.creation_timestamp();
                let ct = creation_time_opt
                    .map(|ts| helper::to_split_file_time(ts.0.timestamp_millis()))
                    .map(|(l, h)| FILETIME::new(l as u32, h as u32))
                    .unwrap_or(FILETIME::default());
                (*find_data).dw_file_attributes = consts::FILE_ATTRIBUTE_UNIX_MODE;
                (*find_data).ft_creation_time = ct;
                (*find_data).ft_last_access_time = ct;
                (*find_data).ft_last_write_time = ct;
                (*find_data).n_file_size_high = 0;
                (*find_data).n_file_size_low = 0;
                (*find_data).dw_reserved_0 = 0;
                (*find_data).dw_reserved_1 = 0;
                let res_str = next_elem.name_any();
                let bytes = res_str.as_bytes();
                let len = bytes.len();

                unsafe { std::ptr::copy(
                    bytes.as_ptr().cast(),
                    (*find_data).c_file_name.as_mut_ptr(),
                    consts::MAX_PATH,
                ) };
                unsafe { std::ptr::write(
                    (*find_data).c_file_name.as_mut_ptr().offset(len as isize) as *mut u8,
                    0u8,
                ) };

                //(*find_data).c_file_name= [0i8;260];
                (*find_data).c_alternate_file_name = [0i8; 14];

                println!("Pod resource {}", res_str)
            }
            None => println!("update_find_data on None Pods"),
        }
    }
}

impl K8sResourceIterator<Pod> for PodsIterator {
}

impl PodsIterator {
    pub fn new() -> Box<Self> {
        let v = Self::get_resources(String::from("kube-system"));
        Box::new(Self {
            it: Box::new(v.into_iter()),
            next_elem: None,
        })
    }
    fn get_resources(namespace: String) -> Vec<Pod> {
        let vec_empt: Vec<Pod> = Vec::new();

        let runtime_res = Self::async_to_sync_res(list_pods(&namespace));
        match runtime_res {
            Ok(vec) => vec,
            Err(_err) => {
                eprintln!("Fail on getting pods list {}", _err.to_string());
                vec_empt
            }
        }
    }

}

pub async fn list_pods(namespace: &String) -> anyhow::Result<Vec<Pod>> {
    let config = Config::infer().await?;
    let https = config.openssl_https_connector()?;
    let mut vec = Vec::new();
    let service = ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .map_err(BoxError::from)
        .service(hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https));
    // .service(hyper::Client::builder().build(https));

    let client = Client::new(service, namespace);

    let pods: Api<Pod> = Api::namespaced(client, &namespace);
    for p in pods.list(&Default::default()).await? {
        vec.push(p.clone());
        //info!("{}", p.name_any());
    }
    Ok(vec)
}
