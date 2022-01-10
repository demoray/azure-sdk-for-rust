#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AclAction {
    Allow,
    Deny,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureFlags {
    ServiceMode,
    EnableConnectivityLogs,
    EnableMessagingLogs,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyType {
    Primary,
    Secondary,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityType>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentitySettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkAcl {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<SignalRRequestType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deny: Vec<SignalRRequestType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
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
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointAcl {
    #[serde(flatten)]
    pub network_acl: NetworkAcl,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
    pub status: Option<PrivateLinkServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateLinkServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Failed,
    Canceled,
    Running,
    Creating,
    Updating,
    Deleting,
    Moving,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SignalRSkuTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessUpstreamSettings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<UpstreamTemplate>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceKind {
    SignalR,
    RawWebSockets,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRCorsSettings {
    #[serde(rename = "allowedOrigins", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_origins: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRCreateOrUpdateProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<SignalRFeature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<SignalRCorsSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<ServerlessUpstreamSettings>,
    #[serde(rename = "networkACLs", default, skip_serializing_if = "Option::is_none")]
    pub network_ac_ls: Option<SignalRNetworkAcLs>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalRFeature {
    pub flag: FeatureFlags,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRKeys {
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRNetworkAcLs {
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<AclAction>,
    #[serde(rename = "publicNetwork", default, skip_serializing_if = "Option::is_none")]
    pub public_network: Option<NetworkAcl>,
    #[serde(rename = "privateEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoints: Vec<PrivateEndpointAcl>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRProperties {
    #[serde(flatten)]
    pub signal_r_create_or_update_properties: SignalRCreateOrUpdateProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "externalIP", default, skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<SignalRTlsSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SignalRRequestType {
    ClientConnection,
    ServerConnection,
    #[serde(rename = "RESTAPI")]
    Restapi,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SignalRProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ServiceKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SignalRResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SignalRSkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRTlsSettings {
    #[serde(rename = "clientCertEnabled", default, skip_serializing_if = "Option::is_none")]
    pub client_cert_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SignalRUsageName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRUsageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SignalRUsage>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignalRUsageName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpstreamAuthSettings {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpstreamAuthType>,
    #[serde(rename = "managedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub managed_identity: Option<ManagedIdentitySettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UpstreamAuthType {
    None,
    ManagedIdentity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpstreamTemplate {
    #[serde(rename = "hubPattern", default, skip_serializing_if = "Option::is_none")]
    pub hub_pattern: Option<String>,
    #[serde(rename = "eventPattern", default, skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "categoryPattern", default, skip_serializing_if = "Option::is_none")]
    pub category_pattern: Option<String>,
    #[serde(rename = "urlTemplate")]
    pub url_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<UpstreamAuthSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityProperty {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
