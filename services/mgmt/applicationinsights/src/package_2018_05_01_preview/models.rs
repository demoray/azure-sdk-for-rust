#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponent {
    #[serde(flatten)]
    pub components_resource: ComponentsResource,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentListResult {
    pub value: Vec<ApplicationInsightsComponent>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentProactiveDetectionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentProactiveDetectionConfigurationProperties>,
}
pub type ApplicationInsightsComponentProactiveDetectionConfigurationListResult =
    Vec<ApplicationInsightsComponentProactiveDetectionConfiguration>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsComponentProactiveDetectionConfigurationProperties {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "SendEmailsToSubscriptionOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_emails_to_subscription_owners: Option<bool>,
    #[serde(rename = "CustomEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
    #[serde(rename = "LastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "RuleDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub rule_definitions: Option<application_insights_component_proactive_detection_configuration_properties::RuleDefinitions>,
}
pub mod application_insights_component_proactive_detection_configuration_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RuleDefinitions {
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "DisplayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "HelpUrl", default, skip_serializing_if = "Option::is_none")]
        pub help_url: Option<String>,
        #[serde(rename = "IsHidden", default, skip_serializing_if = "Option::is_none")]
        pub is_hidden: Option<bool>,
        #[serde(rename = "IsEnabledByDefault", default, skip_serializing_if = "Option::is_none")]
        pub is_enabled_by_default: Option<bool>,
        #[serde(rename = "IsInPreview", default, skip_serializing_if = "Option::is_none")]
        pub is_in_preview: Option<bool>,
        #[serde(rename = "SupportsEmailNotifications", default, skip_serializing_if = "Option::is_none")]
        pub supports_email_notifications: Option<bool>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentProperties {
    #[serde(rename = "ApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AppId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "Application_Type")]
    pub application_type: application_insights_component_properties::ApplicationType,
    #[serde(rename = "Flow_Type", default, skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<application_insights_component_properties::FlowType>,
    #[serde(rename = "Request_Source", default, skip_serializing_if = "Option::is_none")]
    pub request_source: Option<application_insights_component_properties::RequestSource>,
    #[serde(rename = "InstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
    #[serde(rename = "CreationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "TenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "HockeyAppId", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_id: Option<String>,
    #[serde(rename = "HockeyAppToken", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_token: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "SamplingPercentage", default, skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<f64>,
    #[serde(rename = "ConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[serde(rename = "RetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i64>,
    #[serde(rename = "DisableIpMasking", default, skip_serializing_if = "Option::is_none")]
    pub disable_ip_masking: Option<bool>,
    #[serde(rename = "ImmediatePurgeDataOn30Days", default, skip_serializing_if = "Option::is_none")]
    pub immediate_purge_data_on30_days: Option<bool>,
    #[serde(rename = "PrivateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
    #[serde(rename = "publicNetworkAccessForIngestion", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<PublicNetworkAccessType>,
    #[serde(rename = "publicNetworkAccessForQuery", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<PublicNetworkAccessType>,
    #[serde(rename = "IngestionMode", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_mode: Option<application_insights_component_properties::IngestionMode>,
}
pub mod application_insights_component_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationType {
        #[serde(rename = "web")]
        Web,
        #[serde(rename = "other")]
        Other,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FlowType {
        Bluefield,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RequestSource {
        #[serde(rename = "rest")]
        Rest,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IngestionMode {
        ApplicationInsights,
        ApplicationInsightsWithDiagnosticSettings,
        LogAnalytics,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeBody {
    pub table: String,
    pub filters: Vec<ComponentPurgeBodyFilters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentPurgeBodyFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeStatusResponse {
    pub status: component_purge_status_response::Status,
}
pub mod component_purge_status_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentsResource {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeaderField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationInfo {
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
pub struct OperationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkScopedResource {
    #[serde(rename = "ResourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTest {
    #[serde(flatten)]
    pub webtests_resource: WebtestsResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<web_test::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebTestProperties>,
}
pub mod web_test {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebTestGeolocation {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestProperties {
    #[serde(rename = "SyntheticMonitorId")]
    pub synthetic_monitor_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "Kind")]
    pub kind: web_test_properties::Kind,
    #[serde(rename = "RetryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub retry_enabled: Option<bool>,
    #[serde(rename = "Locations")]
    pub locations: Vec<WebTestGeolocation>,
    #[serde(rename = "Configuration", default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<web_test_properties::Configuration>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "Request", default, skip_serializing_if = "Option::is_none")]
    pub request: Option<web_test_properties::Request>,
    #[serde(rename = "ValidationRules", default, skip_serializing_if = "Option::is_none")]
    pub validation_rules: Option<web_test_properties::ValidationRules>,
}
pub mod web_test_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
        #[serde(rename = "basic")]
        Basic,
        #[serde(rename = "standard")]
        Standard,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Configuration {
        #[serde(rename = "WebTest", default, skip_serializing_if = "Option::is_none")]
        pub web_test: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Request {
        #[serde(rename = "RequestUrl", default, skip_serializing_if = "Option::is_none")]
        pub request_url: Option<String>,
        #[serde(rename = "Headers", default, skip_serializing_if = "Vec::is_empty")]
        pub headers: Vec<HeaderField>,
        #[serde(rename = "HttpVerb", default, skip_serializing_if = "Option::is_none")]
        pub http_verb: Option<String>,
        #[serde(rename = "RequestBody", default, skip_serializing_if = "Option::is_none")]
        pub request_body: Option<String>,
        #[serde(rename = "ParseDependentRequests", default, skip_serializing_if = "Option::is_none")]
        pub parse_dependent_requests: Option<bool>,
        #[serde(rename = "FollowRedirects", default, skip_serializing_if = "Option::is_none")]
        pub follow_redirects: Option<bool>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ValidationRules {
        #[serde(rename = "ContentValidation", default, skip_serializing_if = "Option::is_none")]
        pub content_validation: Option<validation_rules::ContentValidation>,
        #[serde(rename = "SSLCheck", default, skip_serializing_if = "Option::is_none")]
        pub ssl_check: Option<bool>,
        #[serde(rename = "SSLCertRemainingLifetimeCheck", default, skip_serializing_if = "Option::is_none")]
        pub ssl_cert_remaining_lifetime_check: Option<i32>,
        #[serde(rename = "ExpectedHttpStatusCode", default, skip_serializing_if = "Option::is_none")]
        pub expected_http_status_code: Option<i32>,
        #[serde(rename = "IgnoreHttpsStatusCode", default, skip_serializing_if = "Option::is_none")]
        pub ignore_https_status_code: Option<bool>,
    }
    pub mod validation_rules {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ContentValidation {
            #[serde(rename = "ContentMatch", default, skip_serializing_if = "Option::is_none")]
            pub content_match: Option<String>,
            #[serde(rename = "IgnoreCase", default, skip_serializing_if = "Option::is_none")]
            pub ignore_case: Option<bool>,
            #[serde(rename = "PassIfTextFound", default, skip_serializing_if = "Option::is_none")]
            pub pass_if_text_found: Option<bool>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestsResource {
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
pub struct WebTestListResult {
    pub value: Vec<WebTest>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
