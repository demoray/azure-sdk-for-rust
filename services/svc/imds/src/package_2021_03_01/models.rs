#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub type ApplicationResponse = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compute {
    #[serde(rename = "azEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub az_environment: Option<String>,
    #[serde(rename = "evictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocationProperties>,
    #[serde(rename = "isHostCompatibilityLayerVm", default, skip_serializing_if = "Option::is_none")]
    pub is_host_compatibility_layer_vm: Option<String>,
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(rename = "placementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanProperties>,
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKeysProperties>,
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<String>,
    #[serde(rename = "platformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "tagsList", default, skip_serializing_if = "Vec::is_empty")]
    pub tags_list: Vec<TagsProperties>,
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "virtualMachineScaleSet", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_scale_set: Option<VirtualMachineScaleSet>,
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(rename = "vmScaleSetName", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set_name: Option<String>,
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[serde(rename = "createOption", default, skip_serializing_if = "Option::is_none")]
    pub create_option: Option<String>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<DiskImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<String>,
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDisk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiffDiskSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocationProperties {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<identity_error_response::Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}
pub mod identity_error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Error {
        #[serde(rename = "invalid_request")]
        InvalidRequest,
        #[serde(rename = "unauthorized_client")]
        UnauthorizedClient,
        #[serde(rename = "access_denied")]
        AccessDenied,
        #[serde(rename = "unsupported_response_type")]
        UnsupportedResponseType,
        #[serde(rename = "invalid_scope")]
        InvalidScope,
        #[serde(rename = "server_error")]
        ServerError,
        #[serde(rename = "service_unavailable")]
        ServiceUnavailable,
        #[serde(rename = "bad_request")]
        BadRequest,
        #[serde(rename = "forbidden")]
        Forbidden,
        #[serde(rename = "not_found")]
        NotFound,
        #[serde(rename = "method_not_allowed")]
        MethodNotAllowed,
        #[serde(rename = "too_many_requests")]
        TooManyRequests,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityInfoResponse {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityTokenResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_expires_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msi_res_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Instance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute: Option<Compute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv4Properties {
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv6Properties {
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interface: Vec<NetworkInterface>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<network_interface::Ipv4>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<network_interface::Ipv6>,
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}
pub mod network_interface {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ipv4 {
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_address: Vec<Ipv4Properties>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub subnet: Vec<SubnetProperties>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ipv6 {
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_address: Vec<Ipv6Properties>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[serde(rename = "createOption", default, skip_serializing_if = "Option::is_none")]
    pub create_option: Option<String>,
    #[serde(rename = "diffDiskSettings", default, skip_serializing_if = "Option::is_none")]
    pub diff_disk_settings: Option<DiffDiskSettings>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<String>,
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings: Option<EncryptionSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<DiskImage>,
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDisk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "disablePasswordAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub disable_password_authentication: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicKeysProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<String>,
    #[serde(rename = "virtualTpmEnabled", default, skip_serializing_if = "Option::is_none")]
    pub virtual_tpm_enabled: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
    #[serde(rename = "resourceDisk", default, skip_serializing_if = "Option::is_none")]
    pub resource_disk: Option<storage_profile::ResourceDisk>,
}
pub mod storage_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ResourceDisk {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualHardDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
