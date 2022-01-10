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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesCreateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
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
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<storage_account_properties::AccountType>,
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
    pub enum AccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountPropertiesCreateParameters {
    #[serde(rename = "accountType")]
    pub account_type: storage_account_properties_create_parameters::AccountType,
}
pub mod storage_account_properties_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountPropertiesUpdateParameters {
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<storage_account_properties_update_parameters::AccountType>,
    #[serde(rename = "customDomain", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<CustomDomain>,
}
pub mod storage_account_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Standard_GRS")]
        StandardGrs,
        #[serde(rename = "Standard_RAGRS")]
        StandardRagrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
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
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountPropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub unit: usage::Unit,
    #[serde(rename = "currentValue")]
    pub current_value: i32,
    pub limit: i32,
    pub name: UsageName,
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
