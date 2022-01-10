#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessUri {
    #[serde(rename = "accessSAS", default, skip_serializing_if = "Option::is_none")]
    pub access_sas: Option<String>,
    #[serde(rename = "securityDataAccessSAS", default, skip_serializing_if = "Option::is_none")]
    pub security_data_access_sas: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ApiErrorBase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiErrorBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationData {
    #[serde(rename = "createOption")]
    pub create_option: creation_data::CreateOption,
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageDiskReference>,
    #[serde(rename = "galleryImageReference", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<ImageDiskReference>,
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[serde(rename = "sourceUniqueId", default, skip_serializing_if = "Option::is_none")]
    pub source_unique_id: Option<String>,
    #[serde(rename = "uploadSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub upload_size_bytes: Option<i64>,
    #[serde(rename = "logicalSectorSize", default, skip_serializing_if = "Option::is_none")]
    pub logical_sector_size: Option<i32>,
    #[serde(rename = "securityDataUri", default, skip_serializing_if = "Option::is_none")]
    pub security_data_uri: Option<String>,
}
pub mod creation_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateOption {
        Empty,
        Attach,
        FromImage,
        Import,
        Copy,
        Restore,
        Upload,
        CopyStart,
        ImportSecure,
        UploadPreparedSecure,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_by_extended: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskAccess {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskAccessProperties>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskAccessList {
    pub value: Vec<DiskAccess>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskAccessProperties {
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskAccessUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionSetIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EncryptionSetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetList {
    pub value: Vec<DiskEncryptionSet>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiskEncryptionSetType {
    EncryptionAtRestWithCustomerKey,
    EncryptionAtRestWithPlatformAndCustomerKeys,
    ConfidentialVmEncryptedWithCustomerKey,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionSetUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskEncryptionSetUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionSetIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionSetUpdateProperties {
    #[serde(rename = "encryptionType", default, skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<DiskEncryptionSetType>,
    #[serde(rename = "activeKey", default, skip_serializing_if = "Option::is_none")]
    pub active_key: Option<KeyForDiskEncryptionSet>,
    #[serde(rename = "rotationToLatestKeyVersionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rotation_to_latest_key_version_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskList {
    pub value: Vec<Disk>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskProperties {
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_properties::OsType>,
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<disk_properties::HyperVGeneration>,
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "diskSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_bytes: Option<i64>,
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[serde(rename = "diskIOPSReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_only: Option<i64>,
    #[serde(rename = "diskMBpsReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_only: Option<i64>,
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<DiskState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "maxShares", default, skip_serializing_if = "Option::is_none")]
    pub max_shares: Option<i32>,
    #[serde(rename = "shareInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub share_info: Vec<ShareInfoElement>,
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "burstingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub bursting_enabled: Option<bool>,
    #[serde(rename = "propertyUpdatesInProgress", default, skip_serializing_if = "Option::is_none")]
    pub property_updates_in_progress: Option<PropertyUpdatesInProgress>,
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<DiskSecurityProfile>,
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<f64>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
}
pub mod disk_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskRestorePoint {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskRestorePointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskRestorePointList {
    pub value: Vec<DiskRestorePoint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskRestorePointProperties {
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_restore_point_properties::OsType>,
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<disk_restore_point_properties::HyperVGeneration>,
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[serde(rename = "familyId", default, skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
    #[serde(rename = "sourceUniqueId", default, skip_serializing_if = "Option::is_none")]
    pub source_unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<f64>,
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<String>,
    #[serde(rename = "sourceResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_location: Option<String>,
}
pub mod disk_restore_point_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSecurityProfile {
    #[serde(rename = "securityType", default, skip_serializing_if = "Option::is_none")]
    pub security_type: Option<DiskSecurityType>,
    #[serde(rename = "secureVMDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub secure_vm_disk_encryption_set_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiskSecurityType {
    TrustedLaunch,
    #[serde(rename = "ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey")]
    ConfidentialVmVmGuestStateOnlyEncryptedWithPlatformKey,
    #[serde(rename = "ConfidentialVM_DiskEncryptedWithPlatformKey")]
    ConfidentialVmDiskEncryptedWithPlatformKey,
    #[serde(rename = "ConfidentialVM_DiskEncryptedWithCustomerKey")]
    ConfidentialVmDiskEncryptedWithCustomerKey,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<disk_sku::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
pub mod disk_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(rename = "UltraSSD_LRS")]
        UltraSsdLrs,
        #[serde(rename = "Premium_ZRS")]
        PremiumZrs,
        #[serde(rename = "StandardSSD_ZRS")]
        StandardSsdZrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiskState {
    Unattached,
    Attached,
    Reserved,
    Frozen,
    #[serde(rename = "ActiveSAS")]
    ActiveSas,
    #[serde(rename = "ActiveSASFrozen")]
    ActiveSasFrozen,
    ReadyToUpload,
    ActiveUpload,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskUpdateProperties {
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_update_properties::OsType>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[serde(rename = "diskIOPSReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_only: Option<i64>,
    #[serde(rename = "diskMBpsReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_only: Option<i64>,
    #[serde(rename = "maxShares", default, skip_serializing_if = "Option::is_none")]
    pub max_shares: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "burstingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub bursting_enabled: Option<bool>,
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[serde(rename = "propertyUpdatesInProgress", default, skip_serializing_if = "Option::is_none")]
    pub property_updates_in_progress: Option<PropertyUpdatesInProgress>,
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
}
pub mod disk_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EncryptionType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSetIdentity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<encryption_set_identity::Type>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
pub mod encryption_set_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSetProperties {
    #[serde(rename = "encryptionType", default, skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<DiskEncryptionSetType>,
    #[serde(rename = "activeKey", default, skip_serializing_if = "Option::is_none")]
    pub active_key: Option<KeyForDiskEncryptionSet>,
    #[serde(rename = "previousKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub previous_keys: Vec<KeyForDiskEncryptionSet>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "rotationToLatestKeyVersionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rotation_to_latest_key_version_enabled: Option<bool>,
    #[serde(rename = "lastKeyRotationTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_key_rotation_timestamp: Option<String>,
    #[serde(rename = "autoKeyRotationError", default, skip_serializing_if = "Option::is_none")]
    pub auto_key_rotation_error: Option<ApiError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsCollection {
    pub enabled: bool,
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub encryption_settings: Vec<EncryptionSettingsElement>,
    #[serde(rename = "encryptionSettingsVersion", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSettingsElement {
    #[serde(rename = "diskEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultAndSecretReference>,
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultAndKeyReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EncryptionType {
    EncryptionAtRestWithPlatformKey,
    EncryptionAtRestWithCustomerKey,
    EncryptionAtRestWithPlatformAndCustomerKeys,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExtendedLocationType {
    EdgeZone,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantAccessData {
    pub access: grant_access_data::Access,
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i32,
    #[serde(rename = "getSecureVMGuestStateSAS", default, skip_serializing_if = "Option::is_none")]
    pub get_secure_vm_guest_state_sas: Option<bool>,
}
pub mod grant_access_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        None,
        Read,
        Write,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDiskReference {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptiontype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errordetail: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyForDiskEncryptionSet {
    #[serde(rename = "sourceVault", default, skip_serializing_if = "Option::is_none")]
    pub source_vault: Option<SourceVault>,
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndKeyReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndSecretReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkAccessPolicy {
    AllowAll,
    AllowPrivate,
    DenyAll,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyUpdatesInProgress {
    #[serde(rename = "targetTier", default, skip_serializing_if = "Option::is_none")]
    pub target_tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccess {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchasePlan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceUriList {
    pub value: Vec<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareInfoElement {
    #[serde(rename = "vmUri", default, skip_serializing_if = "Option::is_none")]
    pub vm_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotList {
    pub value: Vec<Snapshot>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_properties::OsType>,
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<snapshot_properties::HyperVGeneration>,
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "diskSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_bytes: Option<i64>,
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<DiskState>,
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<DiskSecurityProfile>,
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<f64>,
}
pub mod snapshot_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<snapshot_sku::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
pub mod snapshot_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdateProperties {
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_update_properties::OsType>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
}
pub mod snapshot_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceVault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedCapabilities {
    #[serde(rename = "acceleratedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub accelerated_network: Option<bool>,
}
