#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
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
pub struct FirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: FirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallRuleProperties {
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InfrastructureEncryption {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFile {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogFileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFileListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogFile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogFileProperties {
    #[serde(rename = "sizeInKB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_kb: Option<i64>,
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MinimalTlsVersion {
    #[serde(rename = "TLS1_0")]
    Tls10,
    #[serde(rename = "TLS1_1")]
    Tls11,
    #[serde(rename = "TLS1_2")]
    Tls12,
    #[serde(rename = "TLSEnforcementDisabled")]
    TlsEnforcementDisabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        NotSpecified,
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceTierListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PerformanceTierProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceTierProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "maxBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i32>,
    #[serde(rename = "minBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i32>,
    #[serde(rename = "maxStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[serde(rename = "minLargeStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_large_storage_mb: Option<i32>,
    #[serde(rename = "maxLargeStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_large_storage_mb: Option<i32>,
    #[serde(rename = "minStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
    #[serde(rename = "serviceLevelObjectives", default, skip_serializing_if = "Vec::is_empty")]
    pub service_level_objectives: Vec<PerformanceTierServiceLevelObjectives>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceTierServiceLevelObjectives {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i32>,
    #[serde(rename = "hardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<String>,
    #[serde(rename = "maxBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub max_backup_retention_days: Option<i32>,
    #[serde(rename = "minBackupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub min_backup_retention_days: Option<i32>,
    #[serde(rename = "maxStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub max_storage_mb: Option<i32>,
    #[serde(rename = "minStorageMB", default, skip_serializing_if = "Option::is_none")]
    pub min_storage_mb: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccess {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableServerProperties {
    #[serde(rename = "lastAvailableBackupDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_available_backup_date_time: Option<String>,
    #[serde(rename = "serviceLevelObjective", default, skip_serializing_if = "Option::is_none")]
    pub service_level_objective: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<i32>,
    #[serde(rename = "hardwareGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hardware_generation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoverableServerResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoverableServerProperties>,
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
pub struct ResourceIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
pub mod resource_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAlertPolicyProperties {
    pub state: security_alert_policy_properties::State,
    #[serde(rename = "disabledAlerts", default, skip_serializing_if = "Vec::is_empty")]
    pub disabled_alerts: Vec<String>,
    #[serde(rename = "emailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub email_addresses: Vec<String>,
    #[serde(rename = "emailAccountAdmins", default, skip_serializing_if = "Option::is_none")]
    pub email_account_admins: Option<bool>,
    #[serde(rename = "storageEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,
    #[serde(rename = "storageAccountAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,
    #[serde(rename = "retentionDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}
pub mod security_alert_policy_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministratorProperties {
    #[serde(rename = "administratorType")]
    pub administrator_type: server_administrator_properties::AdministratorType,
    pub login: String,
    pub sid: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
pub mod server_administrator_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AdministratorType {
        ActiveDirectory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAdministratorResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerAdministratorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerAdministratorResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerAdministratorResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerForCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    pub properties: ServerPropertiesForCreate,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerPrivateEndpointConnection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerPrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerPrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ServerPrivateLinkServiceConnectionStateProperty>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<server_private_endpoint_connection_properties::ProvisioningState>,
}
pub mod server_private_endpoint_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Approving,
        Ready,
        Dropping,
        Failed,
        Rejecting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPrivateLinkServiceConnectionStateProperty {
    pub status: server_private_link_service_connection_state_property::Status,
    pub description: String,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<server_private_link_service_connection_state_property::ActionsRequired>,
}
pub mod server_private_link_service_connection_state_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Approved,
        Pending,
        Rejected,
        Disconnected,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionsRequired {
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[serde(rename = "administratorLogin", default, skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[serde(rename = "byokEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub byok_enforcement: Option<String>,
    #[serde(rename = "infrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_encryption: Option<InfrastructureEncryption>,
    #[serde(rename = "userVisibleState", default, skip_serializing_if = "Option::is_none")]
    pub user_visible_state: Option<server_properties::UserVisibleState>,
    #[serde(rename = "fullyQualifiedDomainName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[serde(rename = "earliestRestoreDate", default, skip_serializing_if = "Option::is_none")]
    pub earliest_restore_date: Option<String>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
    pub replication_role: Option<String>,
    #[serde(rename = "masterServerId", default, skip_serializing_if = "Option::is_none")]
    pub master_server_id: Option<String>,
    #[serde(rename = "replicaCapacity", default, skip_serializing_if = "Option::is_none")]
    pub replica_capacity: Option<i32>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<ServerPrivateEndpointConnection>,
}
pub mod server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserVisibleState {
        Ready,
        Dropping,
        Disabled,
        Inaccessible,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<ServerVersion>,
    #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
    pub ssl_enforcement: Option<SslEnforcement>,
    #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimal_tls_version: Option<MinimalTlsVersion>,
    #[serde(rename = "infrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_encryption: Option<InfrastructureEncryption>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "createMode")]
    pub create_mode: server_properties_for_create::CreateMode,
}
pub mod server_properties_for_create {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        GeoRestore,
        Replica,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForDefaultCreate {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "administratorLogin")]
    pub administrator_login: String,
    #[serde(rename = "administratorLoginPassword")]
    pub administrator_login_password: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForGeoRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForReplica {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerPropertiesForRestore {
    #[serde(flatten)]
    pub server_properties_for_create: ServerPropertiesForCreate,
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
    #[serde(rename = "restorePointInTime")]
    pub restore_point_in_time: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAlertPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSecurityAlertPolicyListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerSecurityAlertPolicy>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<server_update_parameters::Properties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod server_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
        pub storage_profile: Option<StorageProfile>,
        #[serde(rename = "administratorLoginPassword", default, skip_serializing_if = "Option::is_none")]
        pub administrator_login_password: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<ServerVersion>,
        #[serde(rename = "sslEnforcement", default, skip_serializing_if = "Option::is_none")]
        pub ssl_enforcement: Option<SslEnforcement>,
        #[serde(rename = "minimalTlsVersion", default, skip_serializing_if = "Option::is_none")]
        pub minimal_tls_version: Option<MinimalTlsVersion>,
        #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
        pub public_network_access: Option<PublicNetworkAccess>,
        #[serde(rename = "replicationRole", default, skip_serializing_if = "Option::is_none")]
        pub replication_role: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerVersion {
    #[serde(rename = "9.5")]
    N9_5,
    #[serde(rename = "9.6")]
    N9_6,
    #[serde(rename = "10")]
    N10,
    #[serde(rename = "10.0")]
    N10_0,
    #[serde(rename = "10.2")]
    N10_2,
    #[serde(rename = "11")]
    N11,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        GeneralPurpose,
        MemoryOptimized,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SslEnforcement {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[serde(rename = "backupRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_days: Option<i32>,
    #[serde(rename = "geoRedundantBackup", default, skip_serializing_if = "Option::is_none")]
    pub geo_redundant_backup: Option<storage_profile::GeoRedundantBackup>,
    #[serde(rename = "storageMB", default, skip_serializing_if = "Option::is_none")]
    pub storage_mb: Option<i32>,
    #[serde(rename = "storageAutogrow", default, skip_serializing_if = "Option::is_none")]
    pub storage_autogrow: Option<storage_profile::StorageAutogrow>,
}
pub mod storage_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GeoRedundantBackup {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageAutogrow {
        Enabled,
        Disabled,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRuleProperties {
    #[serde(rename = "virtualNetworkSubnetId")]
    pub virtual_network_subnet_id: String,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<virtual_network_rule_properties::State>,
}
pub mod virtual_network_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Initializing,
        InProgress,
        Ready,
        Deleting,
        Unknown,
    }
}
