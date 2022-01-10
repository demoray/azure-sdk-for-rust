#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsClusterProperties {
    #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub cluster_fqdn: Option<String>,
    #[serde(rename = "orchestratorType")]
    pub orchestrator_type: acs_cluster_properties::OrchestratorType,
    #[serde(rename = "orchestratorProperties", default, skip_serializing_if = "Option::is_none")]
    pub orchestrator_properties: Option<KubernetesClusterProperties>,
    #[serde(rename = "systemServices", default, skip_serializing_if = "Vec::is_empty")]
    pub system_services: Vec<SystemService>,
    #[serde(rename = "masterCount", default, skip_serializing_if = "Option::is_none")]
    pub master_count: Option<i64>,
    #[serde(rename = "agentCount", default, skip_serializing_if = "Option::is_none")]
    pub agent_count: Option<i64>,
    #[serde(rename = "agentVmSize", default, skip_serializing_if = "Option::is_none")]
    pub agent_vm_size: Option<acs_cluster_properties::AgentVmSize>,
}
pub mod acs_cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrchestratorType {
        Kubernetes,
        None,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AgentVmSize {
        #[serde(rename = "Standard_A0")]
        StandardA0,
        #[serde(rename = "Standard_A1")]
        StandardA1,
        #[serde(rename = "Standard_A2")]
        StandardA2,
        #[serde(rename = "Standard_A3")]
        StandardA3,
        #[serde(rename = "Standard_A4")]
        StandardA4,
        #[serde(rename = "Standard_A5")]
        StandardA5,
        #[serde(rename = "Standard_A6")]
        StandardA6,
        #[serde(rename = "Standard_A7")]
        StandardA7,
        #[serde(rename = "Standard_A8")]
        StandardA8,
        #[serde(rename = "Standard_A9")]
        StandardA9,
        #[serde(rename = "Standard_A10")]
        StandardA10,
        #[serde(rename = "Standard_A11")]
        StandardA11,
        #[serde(rename = "Standard_D1")]
        StandardD1,
        #[serde(rename = "Standard_D2")]
        StandardD2,
        #[serde(rename = "Standard_D3")]
        StandardD3,
        #[serde(rename = "Standard_D4")]
        StandardD4,
        #[serde(rename = "Standard_D11")]
        StandardD11,
        #[serde(rename = "Standard_D12")]
        StandardD12,
        #[serde(rename = "Standard_D13")]
        StandardD13,
        #[serde(rename = "Standard_D14")]
        StandardD14,
        #[serde(rename = "Standard_D1_v2")]
        StandardD1V2,
        #[serde(rename = "Standard_D2_v2")]
        StandardD2V2,
        #[serde(rename = "Standard_D3_v2")]
        StandardD3V2,
        #[serde(rename = "Standard_D4_v2")]
        StandardD4V2,
        #[serde(rename = "Standard_D5_v2")]
        StandardD5V2,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_G1")]
        StandardG1,
        #[serde(rename = "Standard_G2")]
        StandardG2,
        #[serde(rename = "Standard_G3")]
        StandardG3,
        #[serde(rename = "Standard_G4")]
        StandardG4,
        #[serde(rename = "Standard_G5")]
        StandardG5,
        #[serde(rename = "Standard_DS1")]
        StandardDs1,
        #[serde(rename = "Standard_DS2")]
        StandardDs2,
        #[serde(rename = "Standard_DS3")]
        StandardDs3,
        #[serde(rename = "Standard_DS4")]
        StandardDs4,
        #[serde(rename = "Standard_DS11")]
        StandardDs11,
        #[serde(rename = "Standard_DS12")]
        StandardDs12,
        #[serde(rename = "Standard_DS13")]
        StandardDs13,
        #[serde(rename = "Standard_DS14")]
        StandardDs14,
        #[serde(rename = "Standard_GS1")]
        StandardGs1,
        #[serde(rename = "Standard_GS2")]
        StandardGs2,
        #[serde(rename = "Standard_GS3")]
        StandardGs3,
        #[serde(rename = "Standard_GS4")]
        StandardGs4,
        #[serde(rename = "Standard_GS5")]
        StandardGs5,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInsightsCredentials {
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "instrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppInsightsProperties {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoScaleConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<auto_scale_configuration::Status>,
    #[serde(rename = "minReplicas", default, skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i64>,
    #[serde(rename = "maxReplicas", default, skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i64>,
    #[serde(rename = "targetUtilization", default, skip_serializing_if = "Option::is_none")]
    pub target_utilization: Option<f64>,
    #[serde(rename = "refreshPeriodInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub refresh_period_in_seconds: Option<i64>,
}
pub mod auto_scale_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperations {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckSystemServicesUpdatesAvailableResponse {
    #[serde(rename = "updatesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub updates_available: Option<check_system_services_updates_available_response::UpdatesAvailable>,
}
pub mod check_system_services_updates_available_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdatesAvailable {
        Yes,
        No,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryCredentials {
    #[serde(rename = "loginServer", default, skip_serializing_if = "Option::is_none")]
    pub login_server: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerRegistryProperties {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerServiceCredentials {
    #[serde(rename = "acsKubeConfig", default, skip_serializing_if = "Option::is_none")]
    pub acs_kube_config: Option<String>,
    #[serde(rename = "servicePrincipalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_configuration: Option<ServicePrincipalProperties>,
    #[serde(rename = "imagePullSecretName", default, skip_serializing_if = "Option::is_none")]
    pub image_pull_secret_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalServiceConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SslConfiguration>,
    #[serde(rename = "serviceAuth", default, skip_serializing_if = "Option::is_none")]
    pub service_auth: Option<ServiceAuthConfiguration>,
    #[serde(rename = "autoScale", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale: Option<AutoScaleConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KubernetesClusterProperties {
    #[serde(rename = "servicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<ServicePrincipalProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationalizationCluster {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationalizationClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationalizationClusterCredentials {
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountCredentials>,
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<ContainerRegistryCredentials>,
    #[serde(rename = "containerService", default, skip_serializing_if = "Option::is_none")]
    pub container_service: Option<ContainerServiceCredentials>,
    #[serde(rename = "appInsights", default, skip_serializing_if = "Option::is_none")]
    pub app_insights: Option<AppInsightsCredentials>,
    #[serde(rename = "serviceAuthConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub service_auth_configuration: Option<ServiceAuthConfiguration>,
    #[serde(rename = "sslConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<SslConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationalizationClusterProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", default, skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<operationalization_cluster_properties::ProvisioningState>,
    #[serde(rename = "provisioningErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub provisioning_errors: Vec<ErrorResponseWrapper>,
    #[serde(rename = "clusterType")]
    pub cluster_type: operationalization_cluster_properties::ClusterType,
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountProperties>,
    #[serde(rename = "containerRegistry", default, skip_serializing_if = "Option::is_none")]
    pub container_registry: Option<ContainerRegistryProperties>,
    #[serde(rename = "containerService", default, skip_serializing_if = "Option::is_none")]
    pub container_service: Option<AcsClusterProperties>,
    #[serde(rename = "appInsights", default, skip_serializing_if = "Option::is_none")]
    pub app_insights: Option<AppInsightsProperties>,
    #[serde(rename = "globalServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub global_service_configuration: Option<GlobalServiceConfiguration>,
}
pub mod operationalization_cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClusterType {
        #[serde(rename = "ACS")]
        Acs,
        Local,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationalizationClusterUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaginatedOperationalizationClustersList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationalizationCluster>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
pub mod resource_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAuthConfiguration {
    #[serde(rename = "primaryAuthKeyHash")]
    pub primary_auth_key_hash: String,
    #[serde(rename = "secondaryAuthKeyHash")]
    pub secondary_auth_key_hash: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalProperties {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub secret: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SslConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ssl_configuration::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
}
pub mod ssl_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountCredentials {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountProperties {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemService {
    #[serde(rename = "systemServiceType")]
    pub system_service_type: system_service::SystemServiceType,
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
pub mod system_service {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SystemServiceType {
        None,
        ScoringFrontEnd,
        BatchFrontEnd,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateSystemServicesResponse {
    #[serde(rename = "updateStatus", default, skip_serializing_if = "Option::is_none")]
    pub update_status: Option<update_system_services_response::UpdateStatus>,
    #[serde(rename = "updateStartedOn", default, skip_serializing_if = "Option::is_none")]
    pub update_started_on: Option<String>,
    #[serde(rename = "updateCompletedOn", default, skip_serializing_if = "Option::is_none")]
    pub update_completed_on: Option<String>,
}
pub mod update_system_services_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateStatus {
        Unknown,
        Updating,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
    }
}
