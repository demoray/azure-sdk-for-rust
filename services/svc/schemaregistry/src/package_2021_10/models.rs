#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub error: ErrorDetail,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
pub type SchemaGroup = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaGroups {
    #[serde(rename = "schemaGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub schema_groups: Vec<SchemaGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
pub type SchemaVersion = i64;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaVersions {
    #[serde(rename = "schemaVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub schema_versions: Vec<SchemaVersion>,
}
