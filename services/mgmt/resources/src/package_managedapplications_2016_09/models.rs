#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Appliance {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplianceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceArtifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApplianceArtifactType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplianceArtifactType {
    Template,
    Custom,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceDefinition {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplianceDefinitionProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplianceDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceDefinitionProperties {
    #[serde(rename = "lockLevel")]
    pub lock_level: ApplianceLockLevel,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub authorizations: Vec<ApplianceProviderAuthorization>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplianceArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageFileUri")]
    pub package_file_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Appliance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplianceLockLevel {
    CanNotDelete,
    ReadOnly,
    None,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliancePatchable {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliancePropertiesPatchable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanPatchable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceProperties {
    #[serde(rename = "managedResourceGroupId")]
    pub managed_resource_group_id: String,
    #[serde(rename = "applianceDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub appliance_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliancePropertiesPatchable {
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[serde(rename = "applianceDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub appliance_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceProviderAuthorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(rename = "httpStatus", default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    pub version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanPatchable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
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
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
