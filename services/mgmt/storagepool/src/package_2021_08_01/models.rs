#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Acl {
    #[serde(rename = "initiatorIqn")]
    pub initiator_iqn: String,
    #[serde(rename = "mappedLuns")]
    pub mapped_luns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AclMode {
    Dynamic,
    Static,
}
pub type AdditionalCapability = String;
pub type AvailabilityZone = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    pub properties: DiskPoolProperties,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_extended: Option<ManagedByExtended>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreate {
    pub sku: Sku,
    pub properties: DiskPoolCreateProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_extended: Option<ManagedByExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreateProperties {
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolListResult {
    pub value: Vec<DiskPool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolProperties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    pub status: OperationalStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiskPoolTier {
    Basic,
    Standard,
    Premium,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolUpdate {
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_extended: Option<ManagedByExtended>,
    pub properties: DiskPoolUpdateProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolUpdateProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolZoneInfo {
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolZoneListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiskPoolZoneInfo>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<f64>,
    #[serde(rename = "isAccessible", default, skip_serializing_if = "Option::is_none")]
    pub is_accessible: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiLun {
    pub name: String,
    #[serde(rename = "managedDiskAzureResourceId")]
    pub managed_disk_azure_resource_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTarget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_extended: Option<ManagedByExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetCreateProperties,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_extended: Option<ManagedByExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreateProperties {
    #[serde(rename = "aclMode")]
    pub acl_mode: AclMode,
    #[serde(rename = "targetIqn", default, skip_serializing_if = "Option::is_none")]
    pub target_iqn: Option<String>,
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetList {
    pub value: Vec<IscsiTarget>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetProperties {
    #[serde(rename = "aclMode")]
    pub acl_mode: AclMode,
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    pub status: OperationalStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetUpdateProperties,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_extended: Option<ManagedByExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IscsiTargetUpdateProperties {
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
}
pub type ManagedBy = String;
pub type ManagedByExtended = Vec<ManagedBy>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationalStatus {
    Invalid,
    Unknown,
    Healthy,
    Unhealthy,
    Updating,
    Running,
    Stopped,
    #[serde(rename = "Stopped (deallocated)")]
    StoppedDeallocated,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointList {
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Invalid,
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Creating,
    Updating,
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuInfo {
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapability>,
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Option::is_none")]
    pub location_info: Option<ResourceSkuLocationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSkuInfo>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuLocationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<AvailabilityZone>,
    #[serde(rename = "zoneDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_details: Vec<ResourceSkuZoneDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictionInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictions {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ResourceSkuRestrictionInfo>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
pub mod resource_sku_restrictions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Location,
        Zone,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuZoneDetails {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<AvailabilityZone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationDisplay {
    pub provider: String,
    pub resource: String,
    pub operation: String,
    pub description: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationListResult {
    pub value: Vec<StoragePoolRpOperation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolRpOperation {
    pub name: String,
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    pub display: StoragePoolOperationDisplay,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemMetadata {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_metadata::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_metadata::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_metadata {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
