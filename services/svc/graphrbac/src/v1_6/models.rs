#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdGroup {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "mailEnabled", default, skip_serializing_if = "Option::is_none")]
    pub mail_enabled: Option<bool>,
    #[serde(rename = "mailNickname", default, skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(rename = "securityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub security_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddOwnerParameters {
    pub url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "allowedMemberTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_member_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppRoleAssignment {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "principalDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_display_name: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[serde(rename = "resourceDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub resource_display_name: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppRoleAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AppRoleAssignment>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "allowGuestsSignIn", default, skip_serializing_if = "Option::is_none")]
    pub allow_guests_sign_in: Option<bool>,
    #[serde(rename = "allowPassthroughUsers", default, skip_serializing_if = "Option::is_none")]
    pub allow_passthrough_users: Option<bool>,
    #[serde(rename = "appLogoUrl", default, skip_serializing_if = "Option::is_none")]
    pub app_logo_url: Option<String>,
    #[serde(rename = "appRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub app_roles: Vec<AppRole>,
    #[serde(rename = "appPermissions", default, skip_serializing_if = "Vec::is_empty")]
    pub app_permissions: Vec<String>,
    #[serde(rename = "availableToOtherTenants", default, skip_serializing_if = "Option::is_none")]
    pub available_to_other_tenants: Option<bool>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "errorUrl", default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[serde(rename = "groupMembershipClaims", default, skip_serializing_if = "Option::is_none")]
    pub group_membership_claims: Option<GroupMembershipClaims>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "identifierUris", default, skip_serializing_if = "Vec::is_empty")]
    pub identifier_uris: Vec<String>,
    #[serde(rename = "informationalUrls", default, skip_serializing_if = "Option::is_none")]
    pub informational_urls: Option<InformationalUrl>,
    #[serde(rename = "isDeviceOnlyAuthSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_device_only_auth_supported: Option<bool>,
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[serde(rename = "knownClientApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub known_client_applications: Vec<String>,
    #[serde(rename = "logoutUrl", default, skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "oauth2AllowImplicitFlow", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_implicit_flow: Option<bool>,
    #[serde(rename = "oauth2AllowUrlPathMatching", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_url_path_matching: Option<bool>,
    #[serde(rename = "oauth2Permissions", default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2_permissions: Vec<OAuth2Permission>,
    #[serde(rename = "oauth2RequirePostResponse", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_require_post_response: Option<bool>,
    #[serde(rename = "orgRestrictions", default, skip_serializing_if = "Vec::is_empty")]
    pub org_restrictions: Vec<String>,
    #[serde(rename = "optionalClaims", default, skip_serializing_if = "Option::is_none")]
    pub optional_claims: Option<OptionalClaims>,
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[serde(rename = "preAuthorizedApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub pre_authorized_applications: Vec<PreAuthorizedApplication>,
    #[serde(rename = "publicClient", default, skip_serializing_if = "Option::is_none")]
    pub public_client: Option<bool>,
    #[serde(rename = "publisherDomain", default, skip_serializing_if = "Option::is_none")]
    pub publisher_domain: Option<String>,
    #[serde(rename = "replyUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_urls: Vec<String>,
    #[serde(rename = "requiredResourceAccess", default, skip_serializing_if = "Vec::is_empty")]
    pub required_resource_access: Vec<RequiredResourceAccess>,
    #[serde(rename = "samlMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<String>,
    #[serde(rename = "signInAudience", default, skip_serializing_if = "Option::is_none")]
    pub sign_in_audience: Option<String>,
    #[serde(rename = "wwwHomepage", default, skip_serializing_if = "Option::is_none")]
    pub www_homepage: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationBase {
    #[serde(rename = "allowGuestsSignIn", default, skip_serializing_if = "Option::is_none")]
    pub allow_guests_sign_in: Option<bool>,
    #[serde(rename = "allowPassthroughUsers", default, skip_serializing_if = "Option::is_none")]
    pub allow_passthrough_users: Option<bool>,
    #[serde(rename = "appLogoUrl", default, skip_serializing_if = "Option::is_none")]
    pub app_logo_url: Option<String>,
    #[serde(rename = "appRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub app_roles: Vec<AppRole>,
    #[serde(rename = "appPermissions", default, skip_serializing_if = "Vec::is_empty")]
    pub app_permissions: Vec<String>,
    #[serde(rename = "availableToOtherTenants", default, skip_serializing_if = "Option::is_none")]
    pub available_to_other_tenants: Option<bool>,
    #[serde(rename = "errorUrl", default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[serde(rename = "groupMembershipClaims", default, skip_serializing_if = "Option::is_none")]
    pub group_membership_claims: Option<GroupMembershipClaims>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "informationalUrls", default, skip_serializing_if = "Option::is_none")]
    pub informational_urls: Option<InformationalUrl>,
    #[serde(rename = "isDeviceOnlyAuthSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_device_only_auth_supported: Option<bool>,
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[serde(rename = "knownClientApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub known_client_applications: Vec<String>,
    #[serde(rename = "logoutUrl", default, skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "oauth2AllowImplicitFlow", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_implicit_flow: Option<bool>,
    #[serde(rename = "oauth2AllowUrlPathMatching", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_url_path_matching: Option<bool>,
    #[serde(rename = "oauth2Permissions", default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2_permissions: Vec<OAuth2Permission>,
    #[serde(rename = "oauth2RequirePostResponse", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_require_post_response: Option<bool>,
    #[serde(rename = "orgRestrictions", default, skip_serializing_if = "Vec::is_empty")]
    pub org_restrictions: Vec<String>,
    #[serde(rename = "optionalClaims", default, skip_serializing_if = "Option::is_none")]
    pub optional_claims: Option<OptionalClaims>,
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[serde(rename = "preAuthorizedApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub pre_authorized_applications: Vec<PreAuthorizedApplication>,
    #[serde(rename = "publicClient", default, skip_serializing_if = "Option::is_none")]
    pub public_client: Option<bool>,
    #[serde(rename = "publisherDomain", default, skip_serializing_if = "Option::is_none")]
    pub publisher_domain: Option<String>,
    #[serde(rename = "replyUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_urls: Vec<String>,
    #[serde(rename = "requiredResourceAccess", default, skip_serializing_if = "Vec::is_empty")]
    pub required_resource_access: Vec<RequiredResourceAccess>,
    #[serde(rename = "samlMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<String>,
    #[serde(rename = "signInAudience", default, skip_serializing_if = "Option::is_none")]
    pub sign_in_audience: Option<String>,
    #[serde(rename = "wwwHomepage", default, skip_serializing_if = "Option::is_none")]
    pub www_homepage: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCreateParameters {
    #[serde(flatten)]
    pub application_base: ApplicationBase,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "identifierUris", default, skip_serializing_if = "Vec::is_empty")]
    pub identifier_uris: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpdateParameters {
    #[serde(flatten)]
    pub application_base: ApplicationBase,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "identifierUris", default, skip_serializing_if = "Vec::is_empty")]
    pub identifier_uris: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckGroupMembershipParameters {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "memberId")]
    pub member_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckGroupMembershipResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryObject {
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "deletionTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectoryObjectListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DirectoryObject>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "isVerified", default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Domain>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetObjectsParameters {
    #[serde(rename = "objectIds", default, skip_serializing_if = "Vec::is_empty")]
    pub object_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
    #[serde(rename = "includeDirectoryObjectReferences", default, skip_serializing_if = "Option::is_none")]
    pub include_directory_object_references: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphError {
    #[serde(rename = "odata.error", default, skip_serializing_if = "Option::is_none")]
    pub odata_error: Option<OdataError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAddMemberParameters {
    pub url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCreateParameters {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "mailEnabled")]
    pub mail_enabled: group_create_parameters::MailEnabled,
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[serde(rename = "securityEnabled")]
    pub security_enabled: group_create_parameters::SecurityEnabled,
}
pub mod group_create_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MailEnabled {}
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SecurityEnabled {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupGetMemberGroupsParameters {
    #[serde(rename = "securityEnabledOnly")]
    pub security_enabled_only: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupGetMemberGroupsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AdGroup>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GroupMembershipClaims {
    None,
    SecurityGroup,
    All,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationalUrl {
    #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyCredential {
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "customKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub custom_key_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyCredentialListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KeyCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyCredentialsUpdateParameters {
    pub value: Vec<KeyCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2Permission {
    #[serde(rename = "adminConsentDescription", default, skip_serializing_if = "Option::is_none")]
    pub admin_consent_description: Option<String>,
    #[serde(rename = "adminConsentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub admin_consent_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "userConsentDescription", default, skip_serializing_if = "Option::is_none")]
    pub user_consent_description: Option<String>,
    #[serde(rename = "userConsentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub user_consent_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2PermissionGrant {
    #[serde(rename = "odata.type", default, skip_serializing_if = "Option::is_none")]
    pub odata_type: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "consentType", default, skip_serializing_if = "Option::is_none")]
    pub consent_type: Option<o_auth2_permission_grant::ConsentType>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}
pub mod o_auth2_permission_grant {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConsentType {
        AllPrincipals,
        Principal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2PermissionGrantListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OAuth2PermissionGrant>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OdataError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionalClaim {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionalClaims {
    #[serde(rename = "idToken", default, skip_serializing_if = "Vec::is_empty")]
    pub id_token: Vec<OptionalClaim>,
    #[serde(rename = "accessToken", default, skip_serializing_if = "Vec::is_empty")]
    pub access_token: Vec<OptionalClaim>,
    #[serde(rename = "samlToken", default, skip_serializing_if = "Vec::is_empty")]
    pub saml_token: Vec<OptionalClaim>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordCredential {
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "customKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub custom_key_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordCredentialListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PasswordCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordCredentialsUpdateParameters {
    pub value: Vec<PasswordCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordProfile {
    pub password: String,
    #[serde(rename = "forceChangePasswordNextLogin", default, skip_serializing_if = "Option::is_none")]
    pub force_change_password_next_login: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreAuthorizedApplication {
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<PreAuthorizedApplicationPermission>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<PreAuthorizedApplicationExtension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreAuthorizedApplicationExtension {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreAuthorizedApplicationPermission {
    #[serde(rename = "directAccessGrant", default, skip_serializing_if = "Option::is_none")]
    pub direct_access_grant: Option<bool>,
    #[serde(rename = "accessGrants", default, skip_serializing_if = "Vec::is_empty")]
    pub access_grants: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredResourceAccess {
    #[serde(rename = "resourceAccess")]
    pub resource_access: Vec<ResourceAccess>,
    #[serde(rename = "resourceAppId", default, skip_serializing_if = "Option::is_none")]
    pub resource_app_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAccess {
    pub id: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipal {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "alternativeNames", default, skip_serializing_if = "Vec::is_empty")]
    pub alternative_names: Vec<String>,
    #[serde(rename = "appDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub app_display_name: Option<String>,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appOwnerTenantId", default, skip_serializing_if = "Option::is_none")]
    pub app_owner_tenant_id: Option<String>,
    #[serde(rename = "appRoleAssignmentRequired", default, skip_serializing_if = "Option::is_none")]
    pub app_role_assignment_required: Option<bool>,
    #[serde(rename = "appRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub app_roles: Vec<AppRole>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "errorUrl", default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[serde(rename = "logoutUrl", default, skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[serde(rename = "oauth2Permissions", default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2_permissions: Vec<OAuth2Permission>,
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[serde(rename = "preferredTokenSigningKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub preferred_token_signing_key_thumbprint: Option<String>,
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(rename = "replyUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_urls: Vec<String>,
    #[serde(rename = "samlMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<String>,
    #[serde(rename = "servicePrincipalNames", default, skip_serializing_if = "Vec::is_empty")]
    pub service_principal_names: Vec<String>,
    #[serde(rename = "servicePrincipalType", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalBase {
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "appRoleAssignmentRequired", default, skip_serializing_if = "Option::is_none")]
    pub app_role_assignment_required: Option<bool>,
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[serde(rename = "servicePrincipalType", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCreateParameters {
    #[serde(flatten)]
    pub service_principal_base: ServicePrincipalBase,
    #[serde(rename = "appId")]
    pub app_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServicePrincipal>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalObjectResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalUpdateParameters {
    #[serde(flatten)]
    pub service_principal_base: ServicePrincipalBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignInName {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[serde(rename = "usageLocation", default, skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "userType", default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<user::UserType>,
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "mailNickname", default, skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[serde(rename = "signInNames", default, skip_serializing_if = "Vec::is_empty")]
    pub sign_in_names: Vec<SignInName>,
}
pub mod user {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserType {
        Member,
        Guest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserBase {
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[serde(rename = "usageLocation", default, skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "userType", default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<user_base::UserType>,
}
pub mod user_base {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserType {
        Member,
        Guest,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateParameters {
    #[serde(flatten)]
    pub user_base: UserBase,
    #[serde(rename = "accountEnabled")]
    pub account_enabled: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "passwordProfile")]
    pub password_profile: PasswordProfile,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGetMemberGroupsParameters {
    #[serde(rename = "securityEnabledOnly")]
    pub security_enabled_only: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserGetMemberGroupsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdateParameters {
    #[serde(flatten)]
    pub user_base: UserBase,
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "passwordProfile", default, skip_serializing_if = "Option::is_none")]
    pub password_profile: Option<PasswordProfile>,
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "mailNickname", default, skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}
