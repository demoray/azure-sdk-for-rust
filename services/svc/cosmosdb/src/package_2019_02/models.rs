#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicy {
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "Expiry")]
    pub expiry: String,
    #[serde(rename = "Permission")]
    pub permission: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoReplication {
    #[serde(rename = "Status")]
    pub status: geo_replication::Status,
    #[serde(rename = "LastSyncTime")]
    pub last_sync_time: String,
}
pub mod geo_replication {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "live")]
        Live,
        #[serde(rename = "bootstrap")]
        Bootstrap,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Delete")]
    pub delete: bool,
    #[serde(rename = "Read")]
    pub read: bool,
    #[serde(rename = "Write")]
    pub write: bool,
    #[serde(rename = "RetentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "IncludeAPIs", default, skip_serializing_if = "Option::is_none")]
    pub include_ap_is: Option<bool>,
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedIdentifier {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "AccessPolicy")]
    pub access_policy: AccessPolicy,
}
pub type SignedIdentifiers = Vec<SignedIdentifier>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableEntityProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableEntityQueryResponse {
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TableEntityProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableProperties {
    #[serde(rename = "TableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableQueryResponse {
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TableResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableResponse {
    #[serde(flatten)]
    pub table_response_properties: TableResponseProperties,
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableResponseProperties {
    #[serde(rename = "TableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "odata.type", default, skip_serializing_if = "Option::is_none")]
    pub odata_type: Option<String>,
    #[serde(rename = "odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[serde(rename = "odata.editLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_edit_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceError {
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceProperties {
    #[serde(rename = "Logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(rename = "HourMetrics", default, skip_serializing_if = "Option::is_none")]
    pub hour_metrics: Option<Metrics>,
    #[serde(rename = "MinuteMetrics", default, skip_serializing_if = "Option::is_none")]
    pub minute_metrics: Option<Metrics>,
    #[serde(rename = "Cors", default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceStats {
    #[serde(rename = "GeoReplication", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication: Option<GeoReplication>,
}
