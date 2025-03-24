use std::fmt;
use std::slice::Iter;

#[derive(Debug)]
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

    pub fn as_res_str(&self) -> String {
        match self {
            K8SResources::ConfigMap => String::from("configmap"),
            K8SResources::Endpoint => String::from("endpoint"),
            K8SResources::Event => String::from("event"),
            K8SResources::Namespace => String::from("namespace"),
            K8SResources::Node => String::from("node"),
            K8SResources::PersistentVolumeClaim => String::from("pvc"),
            K8SResources::PersistentVolume => String::from("pv"),
            K8SResources::Pod => String::from("pod"),
            K8SResources::ReplicationController => String::from("replicationcontroller"),
            K8SResources::Secret => String::from("secret"),
            K8SResources::ServiceAccount => String::from("serviceaccount"),
            K8SResources::Service => String::from("service"),
            K8SResources::CustomResourceDefinition => String::from("crd"),
            K8SResources::ApiService => String::from("apiservice"),
            K8SResources::Deployment => String::from("deployment"),
            K8SResources::ReplicaSet => String::from("replicaset"),
            K8SResources::StatefulSet => String::from("Statefulset"),
            K8SResources::CronJob => String::from("cronjob"),
            K8SResources::Job => String::from("job"),
            K8SResources::Ingresse => String::from("ingresse"),
            K8SResources::ClusterRoleBinding => String::from("clusterrolebinding"),
            K8SResources::ClusterRole => String::from("clusterrole"),
            K8SResources::RoleBinding => String::from("rolebinding"),
            K8SResources::Role => String::from("role"),
        }
    }

    pub fn from_str(name: &str) -> Option<K8SResources> {
        match name {
            "configmap" => Some(K8SResources::ConfigMap),
            "endpoint" => Some(K8SResources::Endpoint),
            "event" => Some(K8SResources::Event),
            "namespace" => Some(K8SResources::Namespace),
            "node" => Some(K8SResources::Node),
            "pvc" => Some(K8SResources::PersistentVolumeClaim),
            "pv" => Some(K8SResources::PersistentVolume),
            "pod" => Some(K8SResources::Pod),
            "replicationcontroller" => Some(K8SResources::ReplicationController),
            "secret" => Some(K8SResources::Secret),
            "serviceaccount" => Some(K8SResources::ServiceAccount),
            "service" => Some(K8SResources::Service),
            "crd" => Some(K8SResources::CustomResourceDefinition),
            "apiservice" => Some(K8SResources::ApiService),
            "deployment" => Some(K8SResources::Deployment),
            "replicaset" => Some(K8SResources::ReplicaSet),
            "statefulset" => Some(K8SResources::StatefulSet),
            "cronjob" => Some(K8SResources::CronJob),
            "job" => Some(K8SResources::Job),
            "ingresse" => Some(K8SResources::Ingresse),
            "clusterrolebinding" => Some(K8SResources::ClusterRoleBinding),
            "clusterrole" => Some(K8SResources::ClusterRole),
            "rolebinding" => Some(K8SResources::RoleBinding),
            "role" => Some(K8SResources::Role),
            _ => None,
        }
    }
}
