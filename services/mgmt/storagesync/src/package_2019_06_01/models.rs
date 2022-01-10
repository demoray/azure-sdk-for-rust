#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupRequest {
    #[serde(rename = "azureFileShare", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
pub mod check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.StorageSync/storageSyncServices")]
        MicrosoftStorageSyncStorageSyncServices,
    }
}
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
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointCreateParametersProperties {
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "azureFileShareName", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointProperties {
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "azureFileShareName", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[serde(rename = "partnershipId", default, skip_serializing_if = "Option::is_none")]
    pub partnership_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "backupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub backup_enabled: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationDirection {
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "cancel")]
    Cancel,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayResource {
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
pub struct OperationEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
}
pub type PhysicalPath = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostBackupResponse {
    #[serde(rename = "backupMetadata", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<PostBackupResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostBackupResponseProperties {
    #[serde(rename = "cloudEndpointName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_endpoint_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostRestoreRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", default, skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "failedFileList", default, skip_serializing_if = "Option::is_none")]
    pub failed_file_list: Option<String>,
    #[serde(rename = "restoreFileSpec", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreRestoreRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", default, skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "backupMetadataPropertyBag", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata_property_bag: Option<String>,
    #[serde(rename = "restoreFileSpec", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
    #[serde(
        rename = "pauseWaitForSyncDrainTimePeriodInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_wait_for_sync_drain_time_period_in_seconds: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgressType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "recall")]
    Recall,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecallActionParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "recallPath", default, skip_serializing_if = "Option::is_none")]
    pub recall_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegisteredServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerCreateParametersProperties {
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "serverRole", default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerProperties {
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "serverManagementErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub server_management_error_code: Option<i64>,
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "serverRole", default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[serde(rename = "discoveryEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub discovery_endpoint_uri: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "serviceLocation", default, skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "managementEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub management_endpoint_uri: Option<String>,
    #[serde(rename = "monitoringConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<String>,
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
pub type ResourceId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesMoveInfo {
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreFileSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isdir: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointCloudTieringHealthState {
    Healthy,
    Error,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCloudTieringStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<ServerEndpointCloudTieringHealthState>,
    #[serde(rename = "lastUpdatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "lastCloudTieringResult", default, skip_serializing_if = "Option::is_none")]
    pub last_cloud_tiering_result: Option<i32>,
    #[serde(rename = "lastSuccessTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_success_timestamp: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCreateParametersProperties {
    #[serde(rename = "serverLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointFilesNotSyncingError {
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "persistentCount", default, skip_serializing_if = "Option::is_none")]
    pub persistent_count: Option<i64>,
    #[serde(rename = "transientCount", default, skip_serializing_if = "Option::is_none")]
    pub transient_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointOfflineDataTransferState {
    InProgress,
    Stopping,
    NotRunning,
    Complete,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointProperties {
    #[serde(rename = "serverLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[serde(rename = "syncStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<ServerEndpointSyncStatus>,
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(
        rename = "offlineDataTransferStorageAccountResourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_data_transfer_storage_account_resource_id: Option<String>,
    #[serde(
        rename = "offlineDataTransferStorageAccountTenantId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_data_transfer_storage_account_tenant_id: Option<String>,
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
    #[serde(rename = "cloudTieringStatus", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering_status: Option<ServerEndpointCloudTieringStatus>,
    #[serde(rename = "recallStatus", default, skip_serializing_if = "Option::is_none")]
    pub recall_status: Option<ServerEndpointRecallStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointRecallError {
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointRecallStatus {
    #[serde(rename = "lastUpdatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "totalRecallErrorsCount", default, skip_serializing_if = "Option::is_none")]
    pub total_recall_errors_count: Option<i64>,
    #[serde(rename = "recallErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub recall_errors: Vec<ServerEndpointRecallError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointSyncActivityState {
    Upload,
    Download,
    UploadAndDownload,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointSyncActivityStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "perItemErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub per_item_error_count: Option<i64>,
    #[serde(rename = "appliedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub applied_item_count: Option<i64>,
    #[serde(rename = "totalItemCount", default, skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i64>,
    #[serde(rename = "appliedBytes", default, skip_serializing_if = "Option::is_none")]
    pub applied_bytes: Option<i64>,
    #[serde(rename = "totalBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointSyncHealthState {
    Healthy,
    Error,
    SyncBlockedForRestore,
    SyncBlockedForChangeDetectionPostRestore,
    NoActivity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointSyncSessionStatus {
    #[serde(rename = "lastSyncResult", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_result: Option<i32>,
    #[serde(rename = "lastSyncTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_timestamp: Option<String>,
    #[serde(rename = "lastSyncSuccessTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_success_timestamp: Option<String>,
    #[serde(rename = "lastSyncPerItemErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_per_item_error_count: Option<i64>,
    #[serde(rename = "persistentFilesNotSyncingCount", default, skip_serializing_if = "Option::is_none")]
    pub persistent_files_not_syncing_count: Option<i64>,
    #[serde(rename = "transientFilesNotSyncingCount", default, skip_serializing_if = "Option::is_none")]
    pub transient_files_not_syncing_count: Option<i64>,
    #[serde(rename = "filesNotSyncingErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub files_not_syncing_errors: Vec<ServerEndpointFilesNotSyncingError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointSyncStatus {
    #[serde(rename = "downloadHealth", default, skip_serializing_if = "Option::is_none")]
    pub download_health: Option<ServerEndpointSyncHealthState>,
    #[serde(rename = "uploadHealth", default, skip_serializing_if = "Option::is_none")]
    pub upload_health: Option<ServerEndpointSyncHealthState>,
    #[serde(rename = "combinedHealth", default, skip_serializing_if = "Option::is_none")]
    pub combined_health: Option<ServerEndpointSyncHealthState>,
    #[serde(rename = "syncActivity", default, skip_serializing_if = "Option::is_none")]
    pub sync_activity: Option<ServerEndpointSyncActivityState>,
    #[serde(rename = "totalPersistentFilesNotSyncingCount", default, skip_serializing_if = "Option::is_none")]
    pub total_persistent_files_not_syncing_count: Option<i64>,
    #[serde(rename = "lastUpdatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "uploadStatus", default, skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<ServerEndpointSyncSessionStatus>,
    #[serde(rename = "downloadStatus", default, skip_serializing_if = "Option::is_none")]
    pub download_status: Option<ServerEndpointSyncSessionStatus>,
    #[serde(rename = "uploadActivity", default, skip_serializing_if = "Option::is_none")]
    pub upload_activity: Option<ServerEndpointSyncActivityStatus>,
    #[serde(rename = "downloadActivity", default, skip_serializing_if = "Option::is_none")]
    pub download_activity: Option<ServerEndpointSyncActivityStatus>,
    #[serde(rename = "offlineDataTransferStatus", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_status: Option<ServerEndpointOfflineDataTransferState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointUpdateProperties {
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncApiError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<StorageSyncErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<StorageSyncApiError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageSyncService>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceProperties {
    #[serde(rename = "storageSyncServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_status: Option<i64>,
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_state::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub istransitioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionStateProperties>,
}
pub mod subscription_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionStateProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupCreateParametersProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupCreateParametersProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupProperties {
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "syncGroupStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_group_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerChangeDetectionParameters {
    #[serde(rename = "directoryPath", default, skip_serializing_if = "Option::is_none")]
    pub directory_path: Option<String>,
    #[serde(rename = "changeDetectionMode", default, skip_serializing_if = "Option::is_none")]
    pub change_detection_mode: Option<trigger_change_detection_parameters::ChangeDetectionMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}
pub mod trigger_change_detection_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeDetectionMode {
        Default,
        Recursive,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerRolloverRequest {
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workflow {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowProperties {
    #[serde(rename = "lastStepName", default, skip_serializing_if = "Option::is_none")]
    pub last_step_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationDirection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    #[serde(rename = "lastOperationId", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "failed")]
    Failed,
}
