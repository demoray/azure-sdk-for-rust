#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedDatabaseConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AttachedDatabaseConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedDatabaseConfigurationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AttachedDatabaseConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedDatabaseConfigurationProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(rename = "clusterResourceId")]
    pub cluster_resource_id: String,
    #[serde(rename = "attachedDatabaseNames", default, skip_serializing_if = "Vec::is_empty")]
    pub attached_database_names: Vec<String>,
    #[serde(rename = "defaultPrincipalsModificationKind")]
    pub default_principals_modification_kind: attached_database_configuration_properties::DefaultPrincipalsModificationKind,
}
pub mod attached_database_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultPrincipalsModificationKind {
        Union,
        Replace,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCapacity {
    #[serde(rename = "scaleType")]
    pub scale_type: azure_capacity::ScaleType,
    pub minimum: i32,
    pub maximum: i32,
    pub default: i32,
}
pub mod azure_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "none")]
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<AzureCapacity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSku {
    pub name: azure_sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    pub tier: azure_sku::Tier,
}
pub mod azure_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_DS13_v2+1TB_PS")]
        StandardDs13V21tbPs,
        #[serde(rename = "Standard_DS13_v2+2TB_PS")]
        StandardDs13V22tbPs,
        #[serde(rename = "Standard_DS14_v2+3TB_PS")]
        StandardDs14V23tbPs,
        #[serde(rename = "Standard_DS14_v2+4TB_PS")]
        StandardDs14V24tbPs,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_L8s")]
        StandardL8s,
        #[serde(rename = "Standard_L16s")]
        StandardL16s,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_L4s")]
        StandardL4s,
        #[serde(rename = "Dev(No SLA)_Standard_D11_v2")]
        DevNoSlaStandardD11V2,
        #[serde(rename = "Standard_E64i_v3")]
        StandardE64iV3,
        #[serde(rename = "Standard_E2a_v4")]
        StandardE2aV4,
        #[serde(rename = "Standard_E4a_v4")]
        StandardE4aV4,
        #[serde(rename = "Standard_E8a_v4")]
        StandardE8aV4,
        #[serde(rename = "Standard_E16a_v4")]
        StandardE16aV4,
        #[serde(rename = "Standard_E8as_v4+1TB_PS")]
        StandardE8asV41tbPs,
        #[serde(rename = "Standard_E8as_v4+2TB_PS")]
        StandardE8asV42tbPs,
        #[serde(rename = "Standard_E16as_v4+3TB_PS")]
        StandardE16asV43tbPs,
        #[serde(rename = "Standard_E16as_v4+4TB_PS")]
        StandardE16asV44tbPs,
        #[serde(rename = "Dev(No SLA)_Standard_E2a_v4")]
        DevNoSlaStandardE2aV4,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlobStorageEventType {
    #[serde(rename = "Microsoft.Storage.BlobCreated")]
    MicrosoftStorageBlobCreated,
    #[serde(rename = "Microsoft.Storage.BlobRenamed")]
    MicrosoftStorageBlobRenamed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_request::Type,
}
pub mod check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases")]
        MicrosoftKustoClustersDatabases,
        #[serde(rename = "Microsoft.Kusto/clusters/attachedDatabaseConfigurations")]
        MicrosoftKustoClustersAttachedDatabaseConfigurations,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
pub mod check_name_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub sku: AzureSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Zones>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: cluster_check_name_request::Type,
}
pub mod cluster_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters")]
        MicrosoftKustoClusters,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPrincipalAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPrincipalProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPrincipalAssignmentCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: cluster_principal_assignment_check_name_request::Type,
}
pub mod cluster_principal_assignment_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/principalAssignments")]
        MicrosoftKustoClustersPrincipalAssignments,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPrincipalAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClusterPrincipalAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPrincipalProperties {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    pub role: cluster_principal_properties::Role,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalType")]
    pub principal_type: cluster_principal_properties::PrincipalType,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
pub mod cluster_principal_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        AllDatabasesAdmin,
        AllDatabasesViewer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        App,
        Group,
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<cluster_properties::State>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "dataIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_uri: Option<String>,
    #[serde(rename = "stateReason", default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "trustedExternalTenants", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_external_tenants: Vec<TrustedExternalTenant>,
    #[serde(rename = "optimizedAutoscale", default, skip_serializing_if = "Option::is_none")]
    pub optimized_autoscale: Option<OptimizedAutoscale>,
    #[serde(rename = "enableDiskEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enable_disk_encryption: Option<bool>,
    #[serde(rename = "enableStreamingIngest", default, skip_serializing_if = "Option::is_none")]
    pub enable_streaming_ingest: Option<bool>,
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[serde(rename = "enablePurge", default, skip_serializing_if = "Option::is_none")]
    pub enable_purge: Option<bool>,
    #[serde(rename = "languageExtensions", default, skip_serializing_if = "Option::is_none")]
    pub language_extensions: Option<LanguageExtensionsList>,
    #[serde(rename = "enableDoubleEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enable_double_encryption: Option<bool>,
    #[serde(rename = "engineType", default, skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<cluster_properties::EngineType>,
}
pub mod cluster_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Creating,
        Unavailable,
        Running,
        Deleting,
        Deleted,
        Stopping,
        Stopped,
        Starting,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EngineType {
        V2,
        V3,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Compression {
    None,
    GZip,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub kind: data_connection::Kind,
}
pub mod data_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        EventHub,
        EventGrid,
        IotHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectionCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: data_connection_check_name_request::Type,
}
pub mod data_connection_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases/dataConnections")]
        MicrosoftKustoClustersDatabasesDataConnections,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidation {
    #[serde(rename = "dataConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub data_connection_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnectionValidationResult>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataConnectionValidationResult {
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub kind: database::Kind,
}
pub mod database {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        ReadWrite,
        ReadOnlyFollowing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipal {
    pub role: database_principal::Role,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: database_principal::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}
pub mod database_principal {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Admin,
        Ingestor,
        Monitor,
        User,
        UnrestrictedViewers,
        Viewer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        App,
        Group,
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabasePrincipalProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipalAssignmentCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: database_principal_assignment_check_name_request::Type,
}
pub mod database_principal_assignment_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Kusto/clusters/databases/principalAssignments")]
        MicrosoftKustoClustersDatabasesPrincipalAssignments,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipalAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalListRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipal>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabasePrincipalListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipal>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipalProperties {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    pub role: database_principal_properties::Role,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalType")]
    pub principal_type: database_principal_properties::PrincipalType,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
pub mod database_principal_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Admin,
        Ingestor,
        Monitor,
        User,
        UnrestrictedViewers,
        Viewer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        App,
        Group,
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnoseVirtualNetworkResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridConnectionProperties {
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<EventGridDataFormat>,
    #[serde(rename = "ignoreFirstRecord", default, skip_serializing_if = "Option::is_none")]
    pub ignore_first_record: Option<bool>,
    #[serde(rename = "blobStorageEventType", default, skip_serializing_if = "Option::is_none")]
    pub blob_storage_event_type: Option<BlobStorageEventType>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventGridConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventGridDataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConnectionProperties {
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<EventHubDataFormat>,
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<Compression>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventHubDataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FollowerDatabaseDefinition {
    #[serde(rename = "clusterResourceId")]
    pub cluster_resource_id: String,
    #[serde(rename = "attachedDatabaseConfigurationName")]
    pub attached_database_configuration_name: String,
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FollowerDatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FollowerDatabaseDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity::Type,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubConnectionProperties {
    #[serde(rename = "iotHubResourceId")]
    pub iot_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<IotHubDataFormat>,
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[serde(rename = "sharedAccessPolicyName")]
    pub shared_access_policy_name: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IotHubDataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LanguageExtension {
    #[serde(rename = "languageExtensionName", default, skip_serializing_if = "Option::is_none")]
    pub language_extension_name: Option<LanguageExtensionName>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LanguageExtensionName {
    #[serde(rename = "PYTHON")]
    Python,
    R,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LanguageExtensionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LanguageExtension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListResourceSkusResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
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
pub struct OptimizedAutoscale {
    pub version: i32,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    pub minimum: i32,
    pub maximum: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Running,
    Creating,
    Deleting,
    Succeeded,
    Failed,
    Moving,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadOnlyFollowingDatabase {
    #[serde(flatten)]
    pub database: Database,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReadOnlyFollowingDatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadOnlyFollowingDatabaseProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
    #[serde(rename = "leaderClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub leader_cluster_resource_id: Option<String>,
    #[serde(rename = "attachedDatabaseConfigurationName", default, skip_serializing_if = "Option::is_none")]
    pub attached_database_configuration_name: Option<String>,
    #[serde(rename = "principalsModificationKind", default, skip_serializing_if = "Option::is_none")]
    pub principals_modification_kind: Option<read_only_following_database_properties::PrincipalsModificationKind>,
}
pub mod read_only_following_database_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalsModificationKind {
        Union,
        Replace,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadWriteDatabase {
    #[serde(flatten)]
    pub database: Database,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReadWriteDatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReadWriteDatabaseProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
    #[serde(rename = "isFollowed", default, skip_serializing_if = "Option::is_none")]
    pub is_followed: Option<bool>,
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
pub struct SkuDescription {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<SkuLocationInfoItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDescription>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuLocationInfoItem {
    pub location: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
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
pub struct TrustedExternalTenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkConfiguration {
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "enginePublicIpId")]
    pub engine_public_ip_id: String,
    #[serde(rename = "dataManagementPublicIpId")]
    pub data_management_public_ip_id: String,
}
pub type Zones = Vec<String>;
