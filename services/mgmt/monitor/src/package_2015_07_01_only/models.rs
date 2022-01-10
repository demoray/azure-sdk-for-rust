#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggregationType {
    None,
    Average,
    Count,
    Minimum,
    Maximum,
    Total,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    pub condition: RuleCondition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RuleAction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<RuleAction>,
    #[serde(rename = "lastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: AlertRule,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleResourceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertRuleResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleResourcePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "defaultDimensionValues", default, skip_serializing_if = "Option::is_none")]
    pub default_dimension_values: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    pub value: String,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "failedLocationCount")]
    pub failed_location_count: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub enabled: bool,
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementEventAggregationCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<ConditionOperator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementEventRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<ManagementEventAggregationCondition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<MetricAvailabilityLocation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailabilityLocation {
    #[serde(rename = "tableEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub table_endpoint: Option<String>,
    #[serde(rename = "tableInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub table_info: Vec<TableInfoEntry>,
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<AggregationType>,
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<AggregationType>,
    #[serde(rename = "metricAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub value: Vec<MetricDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSettings {
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    pub enabled: bool,
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
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
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<String>,
    #[serde(rename = "lockAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub lock_aggregation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<DimensionProperties>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(rename = "internalMetricName", default, skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub publisher: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
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
pub struct RetentionPolicy {
    pub enabled: bool,
    pub days: i32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleAction {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleCondition {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<RuleDataSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleDataSource {
    #[serde(rename = "odata.type")]
    pub odata_type: String,
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[serde(rename = "legacyResourceId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_resource_id: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleEmailAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[serde(rename = "sendToServiceOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_to_service_owners: Option<bool>,
    #[serde(rename = "customEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleManagementEventClaimsDataSource {
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleManagementEventDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventSource", default, skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subStatus", default, skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<RuleManagementEventClaimsDataSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleMetricDataSource {
    #[serde(flatten)]
    pub rule_data_source: RuleDataSource,
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleWebhookAction {
    #[serde(flatten)]
    pub rule_action: RuleAction,
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceDiagnosticSettings {
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<MetricSettings>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logs: Vec<LogSettings>,
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDiagnosticSettingsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceDiagnosticSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
    #[serde(rename = "legacyMetricSpecifications", default, skip_serializing_if = "Option::is_none")]
    pub legacy_metric_specifications: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableInfoEntry {
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    #[serde(rename = "sasTokenExpirationTime", default, skip_serializing_if = "Option::is_none")]
    pub sas_token_expiration_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThresholdRuleCondition {
    #[serde(flatten)]
    pub rule_condition: RuleCondition,
    pub operator: ConditionOperator,
    pub threshold: f64,
    #[serde(rename = "windowSize", default, skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,
    #[serde(rename = "timeAggregation", default, skip_serializing_if = "Option::is_none")]
    pub time_aggregation: Option<TimeAggregationOperator>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeAggregationOperator {
    Average,
    Minimum,
    Maximum,
    Total,
    Last,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
}
