#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableProviderOperation {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableProviderOperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableProviderOperationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationDisplay {
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
pub struct AvailableProviderOperationProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperations {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableProviderOperation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSecret {
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
    #[serde(rename = "keyValue")]
    pub key_value: String,
    pub algorithm: customer_secret::Algorithm,
}
pub mod customer_secret {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Algorithm {
        None,
        #[serde(rename = "RSA1_5")]
        Rsa15,
        #[serde(rename = "RSA_OAEP")]
        RsaOaep,
        PlainText,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataManager {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataManagerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataManager>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataManagerUpdateParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataService {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    pub properties: DataServiceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataServiceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataService>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataServiceProperties {
    pub state: data_service_properties::State,
    #[serde(rename = "supportedDataSinkTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_sink_types: Vec<String>,
    #[serde(rename = "supportedDataSourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_source_types: Vec<String>,
}
pub mod data_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStore {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    pub properties: DataStoreProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStoreFilter {
    #[serde(rename = "dataStoreTypeId", default, skip_serializing_if = "Option::is_none")]
    pub data_store_type_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStoreList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataStore>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreProperties {
    #[serde(rename = "repositoryId", default, skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    pub state: data_store_properties::State,
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[serde(rename = "dataStoreTypeId")]
    pub data_store_type_id: String,
    #[serde(rename = "customerSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub customer_secrets: Vec<CustomerSecret>,
}
pub mod data_store_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreType {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    pub properties: DataStoreTypeProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStoreTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataStoreType>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreTypeProperties {
    #[serde(rename = "repositoryType", default, skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
    pub state: data_store_type_properties::State,
    #[serde(rename = "supportedDataServicesAsSink", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_services_as_sink: Vec<String>,
    #[serde(rename = "supportedDataServicesAsSource", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_services_as_source: Vec<String>,
}
pub mod data_store_type_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DmsBaseObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[serde(rename = "exceptionMessage", default, skip_serializing_if = "Option::is_none")]
    pub exception_message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    pub status: job::Status,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    pub properties: JobProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
pub mod job {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        None,
        InProgress,
        Succeeded,
        WaitingForAction,
        Failed,
        Cancelled,
        Cancelling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinition {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    pub properties: JobDefinitionProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinitionFilter {
    pub state: job_definition_filter::State,
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}
pub mod job_definition_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinitionProperties {
    #[serde(rename = "dataSourceId")]
    pub data_source_id: String,
    #[serde(rename = "dataSinkId")]
    pub data_sink_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<Schedule>,
    pub state: job_definition_properties::State,
    #[serde(rename = "lastModifiedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "runLocation", default, skip_serializing_if = "Option::is_none")]
    pub run_location: Option<job_definition_properties::RunLocation>,
    #[serde(rename = "userConfirmation", default, skip_serializing_if = "Option::is_none")]
    pub user_confirmation: Option<job_definition_properties::UserConfirmation>,
    #[serde(rename = "dataServiceInput", default, skip_serializing_if = "Option::is_none")]
    pub data_service_input: Option<serde_json::Value>,
    #[serde(rename = "customerSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub customer_secrets: Vec<CustomerSecret>,
}
pub mod job_definition_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunLocation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "australiaeast")]
        Australiaeast,
        #[serde(rename = "australiasoutheast")]
        Australiasoutheast,
        #[serde(rename = "brazilsouth")]
        Brazilsouth,
        #[serde(rename = "canadacentral")]
        Canadacentral,
        #[serde(rename = "canadaeast")]
        Canadaeast,
        #[serde(rename = "centralindia")]
        Centralindia,
        #[serde(rename = "centralus")]
        Centralus,
        #[serde(rename = "eastasia")]
        Eastasia,
        #[serde(rename = "eastus")]
        Eastus,
        #[serde(rename = "eastus2")]
        Eastus2,
        #[serde(rename = "japaneast")]
        Japaneast,
        #[serde(rename = "japanwest")]
        Japanwest,
        #[serde(rename = "koreacentral")]
        Koreacentral,
        #[serde(rename = "koreasouth")]
        Koreasouth,
        #[serde(rename = "southeastasia")]
        Southeastasia,
        #[serde(rename = "southcentralus")]
        Southcentralus,
        #[serde(rename = "southindia")]
        Southindia,
        #[serde(rename = "northcentralus")]
        Northcentralus,
        #[serde(rename = "northeurope")]
        Northeurope,
        #[serde(rename = "uksouth")]
        Uksouth,
        #[serde(rename = "ukwest")]
        Ukwest,
        #[serde(rename = "westcentralus")]
        Westcentralus,
        #[serde(rename = "westeurope")]
        Westeurope,
        #[serde(rename = "westindia")]
        Westindia,
        #[serde(rename = "westus")]
        Westus,
        #[serde(rename = "westus2")]
        Westus2,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserConfirmation {
        NotRequired,
        Required,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDetails {
    #[serde(rename = "jobStages", default, skip_serializing_if = "Vec::is_empty")]
    pub job_stages: Vec<JobStages>,
    #[serde(rename = "jobDefinition", default, skip_serializing_if = "Option::is_none")]
    pub job_definition: Option<JobDefinition>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetails>,
    #[serde(rename = "itemDetailsLink", default, skip_serializing_if = "Option::is_none")]
    pub item_details_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobFilter {
    pub status: job_filter::Status,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
pub mod job_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        None,
        InProgress,
        Succeeded,
        WaitingForAction,
        Failed,
        Cancelled,
        Cancelling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Job>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename = "isCancellable")]
    pub is_cancellable: job_properties::IsCancellable,
    #[serde(rename = "bytesProcessed", default, skip_serializing_if = "Option::is_none")]
    pub bytes_processed: Option<i64>,
    #[serde(rename = "itemsProcessed", default, skip_serializing_if = "Option::is_none")]
    pub items_processed: Option<i64>,
    #[serde(rename = "totalBytesToProcess", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_to_process: Option<i64>,
    #[serde(rename = "totalItemsToProcess", default, skip_serializing_if = "Option::is_none")]
    pub total_items_to_process: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<JobDetails>,
    #[serde(rename = "dataSourceName", default, skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(rename = "dataSinkName", default, skip_serializing_if = "Option::is_none")]
    pub data_sink_name: Option<String>,
}
pub mod job_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IsCancellable {
        NotCancellable,
        Cancellable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStages {
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "stageStatus")]
    pub stage_status: job_stages::StageStatus,
    #[serde(rename = "jobStageDetails", default, skip_serializing_if = "Option::is_none")]
    pub job_stage_details: Option<serde_json::Value>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetails>,
}
pub mod job_stages {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        WaitingForAction,
        Failed,
        Cancelled,
        Cancelling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "keyModulus")]
    pub key_modulus: String,
    #[serde(rename = "keyExponent")]
    pub key_exponent: String,
    #[serde(rename = "encryptionChunkSizeInBytes")]
    pub encryption_chunk_size_in_bytes: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    pub properties: PublicKeyProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicKeyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublicKey>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyProperties {
    #[serde(rename = "dataServiceLevel1Key")]
    pub data_service_level1_key: Key,
    #[serde(rename = "dataServiceLevel2Key")]
    pub data_service_level2_key: Key,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunParameters {
    #[serde(rename = "userConfirmation", default, skip_serializing_if = "Option::is_none")]
    pub user_confirmation: Option<run_parameters::UserConfirmation>,
    #[serde(rename = "dataServiceInput", default, skip_serializing_if = "Option::is_none")]
    pub data_service_input: Option<serde_json::Value>,
    #[serde(rename = "customerSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub customer_secrets: Vec<CustomerSecret>,
}
pub mod run_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserConfirmation {
        NotRequired,
        Required,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyList", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_list: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
