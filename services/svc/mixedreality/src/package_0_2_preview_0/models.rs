#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngestionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vector3>,
    #[serde(rename = "boundingBoxCenter", default, skip_serializing_if = "Option::is_none")]
    pub bounding_box_center: Option<Vector3>,
    pub gravity: Vector3,
    #[serde(rename = "keyFrameIndexes", default, skip_serializing_if = "Vec::is_empty")]
    pub key_frame_indexes: Vec<i32>,
    #[serde(rename = "gtTrajectory", default, skip_serializing_if = "Vec::is_empty")]
    pub gt_trajectory: Vec<Pose>,
    #[serde(rename = "principalAxis", default, skip_serializing_if = "Option::is_none")]
    pub principal_axis: Option<Quaternion>,
    pub scale: f32,
    #[serde(rename = "supportingPlane", default, skip_serializing_if = "Option::is_none")]
    pub supporting_plane: Option<Vector4>,
    #[serde(rename = "testTrajectory", default, skip_serializing_if = "Vec::is_empty")]
    pub test_trajectory: Vec<Pose>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IngestionProperties {
    #[serde(rename = "clientErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub client_error_details: Option<String>,
    #[serde(rename = "serverErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub server_error_details: Option<String>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "outputModelUri", default, skip_serializing_if = "Option::is_none")]
    pub output_model_uri: Option<String>,
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[serde(rename = "assetFileType", default, skip_serializing_if = "Option::is_none")]
    pub asset_file_type: Option<String>,
    #[serde(rename = "inputAssetUri", default, skip_serializing_if = "Option::is_none")]
    pub input_asset_uri: Option<String>,
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ingestionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_configuration: Option<IngestionConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub rotation: Quaternion,
    pub translation: Vector3,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    #[serde(rename = "isIdentity", default, skip_serializing_if = "Option::is_none")]
    pub is_identity: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadLocation {
    #[serde(rename = "inputAssetUri")]
    pub input_asset_uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
