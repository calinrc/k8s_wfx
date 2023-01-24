use std::slice::Iter;
use std::fmt;
use core::future::Future;

use crate::pods;


pub enum K8SResources {
    ConfigMap,
    Endpoint,
    Event,
    Namespace,
    Node,
    PersistentVolumeClaim,
    PersistentVolume,
    Pod,
    ReplicationController,
    Secret,
    ServiceAccount,
    Service,
    CustomResourceDefinition,
    ApiService,
    Deployment,
    ReplicaSet,
    StatefulSet,
    CronJob,
    Job,
    Ingresse,
    ClusterRoleBinding,
    ClusterRole,
    RoleBinding,
    Role,
}
const RESOURCES_ARR: [K8SResources; 24] = [
    K8SResources::ConfigMap,
    K8SResources::Endpoint,
    K8SResources::Event,
    K8SResources::Namespace,
    K8SResources::Node,
    K8SResources::PersistentVolumeClaim,
    K8SResources::PersistentVolume,
    K8SResources::Pod,
    K8SResources::ReplicationController,
    K8SResources::Secret,
    K8SResources::ServiceAccount,
    K8SResources::Service,
    K8SResources::CustomResourceDefinition,
    K8SResources::ApiService,
    K8SResources::Deployment,
    K8SResources::ReplicaSet,
    K8SResources::StatefulSet,
    K8SResources::CronJob,
    K8SResources::Job,
    K8SResources::Ingresse,
    K8SResources::ClusterRoleBinding,
    K8SResources::ClusterRole,
    K8SResources::RoleBinding,
    K8SResources::Role,
];

impl fmt::Display for K8SResources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            K8SResources::ConfigMap => write!(f, "ConfigMap"),
            K8SResources::Endpoint => write!(f, "Endpoint"),
            K8SResources::Event => write!(f, "Event"),
            K8SResources::Namespace => write!(f, "Namespace"),
            K8SResources::Node => write!(f, "Node"),
            K8SResources::PersistentVolumeClaim => write!(f, "PersistentVolumeClaim"),
            K8SResources::PersistentVolume => write!(f, "PersistentVolume"),
            K8SResources::Pod => write!(f, "Pod"),
            K8SResources::ReplicationController => write!(f, "ReplicationController"),
            K8SResources::Secret => write!(f, "Secret"),
            K8SResources::ServiceAccount => write!(f, "ServiceAccount"),
            K8SResources::Service => write!(f, "Service"),
            K8SResources::CustomResourceDefinition => write!(f, "CustomResourceDefinition"),
            K8SResources::ApiService => write!(f, "ApiService"),
            K8SResources::Deployment => write!(f, "Deployment"),
            K8SResources::ReplicaSet => write!(f, "ReplicaSet"),
            K8SResources::StatefulSet => write!(f, "StatefulSet"),
            K8SResources::CronJob => write!(f, "CronJob"),
            K8SResources::Job => write!(f, "Job"),
            K8SResources::Ingresse => write!(f, "Ingresse"),
            K8SResources::ClusterRoleBinding => write!(f, "ClusterRoleBinding"),
            K8SResources::ClusterRole => write!(f, "ClusterRole"),
            K8SResources::RoleBinding => write!(f, "RoleBinding"),
            K8SResources::Role => write!(f, "Role"),
        }
    }
}


impl K8SResources {
    pub fn iterator() -> Iter<'static, K8SResources> {
        RESOURCES_ARR.iter()
    }

    fn get_resource(&self, namespace:String) -> Vec<String> {
        let mut vec_empt:Vec<String> = Vec::new();
        match self {
            K8SResources::Pod => {
                let ftr = pods::list_pods(namespace);
                
                let runtime_res  = self.async_to_sync_res(ftr);
                match runtime_res {
                    Ok(vec) => vec,
                    Err(_err) => vec_empt,
                }
            
            },
            _ => vec_empt
        }
    }
    
    
    fn async_to_sync_res(&self, future: impl Future<Output = anyhow::Result<Vec<String>>>) -> anyhow::Result<Vec<String>>{
        let runtime_res = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();
        runtime_res?.block_on(future)
        // let _ = match runtime_res {
        //     Ok(runtime) => { _= runtime.block_on(future);
        //                         println!("Done listing pods")},
        //     Err(err) => panic!("Problem opening the file: {:?}", err),
        // };
    
        // Ok(Vec::new())
    }


}
