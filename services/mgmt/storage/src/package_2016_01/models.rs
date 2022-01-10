#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    pub name: String,
    #[serde(rename = "useSubDomainName", default, skip_serializing_if = "Option::is_none")]
    pub use_sub_domain_name: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<EncryptionServices>,
    #[serde(rename = "keySource")]
    pub key_source: encryption::KeySource,
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Storage")]
        MicrosoftStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionService {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastEnabledTime", default, skip_serializing_if = "Option::is_none")]
    pub last_enabled_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionServices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<EncryptionService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Endpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<storage_account::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
pub mod storage_account {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: storage_account_check_name_availability_parameters::Type,
}
pub mod storage_account_check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Storage/storageAccounts")]
        MicrosoftStorageStorageAccounts,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    pub sku: Sku,
    pub kind: storage_account_create_parameters::Kind,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
pub mod storage_account_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Storage,
        BlobStorage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountKey {
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<storage_account_key::Permissions>,
}
pub mod storage_account_key {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Permissions {
        #[serde(rename = "READ")]
        Read,
        #[serde(rename = "FULL")]
        Full,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListKeysResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<StorageAccountKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_account_properties::ProvisioningState>,
    #[serde(rename = "primaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<Endpoints>,
    #[serde(rename = "primaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_location: Option<String>,
    #[serde(rename = "statusOfPrimary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_primary: Option<storage_account_properties::StatusOfPrimary>,
    #[serde(rename = "lastGeoFailoverTime", default, skip_serializing_if = "Option::is_none")]
    pub last_geo_failover_time: Option<String>,
    #[serde(rename = "secondaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub secondary_location: Option<String>,
    #[serde(rename = "statusOfSecondary", default, skip_serializing_if = "Option::is_none")]
    pub status_of_secondary: Option<storage_account_properties::StatusOfSecondary>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(rename = "secondaryEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub secondary_endpoints: Option<Endpoints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties::AccessTier>,
}
pub mod storage_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
        Succeeded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfPrimary {
        Available,
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StatusOfSecondary {
        Available,
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesCreateParameters {
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_create_parameters::AccessTier>,
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "accessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<storage_account_properties_update_parameters::AccessTier>,
}
pub mod storage_account_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTier {
        Hot,
        Cool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountRegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
