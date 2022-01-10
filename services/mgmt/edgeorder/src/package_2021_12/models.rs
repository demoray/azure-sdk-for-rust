#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressDetails {
    #[serde(rename = "forwardAddress")]
    pub forward_address: AddressProperties,
    #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<AddressProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressProperties {
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    #[serde(rename = "addressValidationStatus", default, skip_serializing_if = "Option::is_none")]
    pub address_validation_status: Option<address_properties::AddressValidationStatus>,
}
pub mod address_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AddressValidationStatus {
        Valid,
        Invalid,
        Ambiguous,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: AddressProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AddressResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressUpdateParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddressUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressUpdateProperties {
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    #[serde(rename = "contactDetails", default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityInformation {
    #[serde(rename = "availabilityStage", default, skip_serializing_if = "Option::is_none")]
    pub availability_stage: Option<availability_information::AvailabilityStage>,
    #[serde(rename = "disabledReason", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<availability_information::DisabledReason>,
    #[serde(rename = "disabledReasonMessage", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_message: Option<String>,
}
pub mod availability_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AvailabilityStage {
        Available,
        ComingSoon,
        Preview,
        Deprecated,
        Signup,
        Unavailable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DisabledReason {
        None,
        Country,
        Region,
        Feature,
        OfferType,
        NoSubscriptionInfo,
        NotAvailable,
        OutOfStock,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicInformation {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(rename = "imageInformation", default, skip_serializing_if = "Vec::is_empty")]
    pub image_information: Vec<ImageInformation>,
    #[serde(rename = "costInformation", default, skip_serializing_if = "Option::is_none")]
    pub cost_information: Option<CostInformation>,
    #[serde(rename = "availabilityInformation", default, skip_serializing_if = "Option::is_none")]
    pub availability_information: Option<AvailabilityInformation>,
    #[serde(rename = "hierarchyInformation", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_information: Option<HierarchyInformation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingMeterDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[serde(rename = "meteringType", default, skip_serializing_if = "Option::is_none")]
    pub metering_type: Option<billing_meter_details::MeteringType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}
pub mod billing_meter_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MeteringType {
        OneTime,
        Recurring,
        Adhoc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancellationReason {
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonProperties {
    #[serde(flatten)]
    pub basic_information: BasicInformation,
    #[serde(rename = "filterableProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub filterable_properties: Vec<FilterableProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationFilters {
    #[serde(rename = "hierarchyInformation")]
    pub hierarchy_information: HierarchyInformation,
    #[serde(rename = "filterableProperty", default, skip_serializing_if = "Vec::is_empty")]
    pub filterable_property: Vec<FilterableProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specifications: Vec<Specification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configurations {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationsRequest {
    #[serde(rename = "configurationFilters")]
    pub configuration_filters: Vec<ConfigurationFilters>,
    #[serde(rename = "customerSubscriptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub customer_subscription_details: Option<CustomerSubscriptionDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDetails {
    #[serde(rename = "contactName")]
    pub contact_name: String,
    pub phone: String,
    #[serde(rename = "phoneExtension", default, skip_serializing_if = "Option::is_none")]
    pub phone_extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostInformation {
    #[serde(rename = "billingMeterDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_meter_details: Vec<BillingMeterDetails>,
    #[serde(rename = "billingInfoUrl", default, skip_serializing_if = "Option::is_none")]
    pub billing_info_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSubscriptionDetails {
    #[serde(rename = "registeredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub registered_features: Vec<CustomerSubscriptionRegisteredFeatures>,
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[serde(rename = "quotaId")]
    pub quota_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerSubscriptionRegisteredFeatures {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Description {
    #[serde(rename = "descriptionType", default, skip_serializing_if = "Option::is_none")]
    pub description_type: Option<description::DescriptionType>,
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
pub mod description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DescriptionType {
        Base,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceDetails {
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "managementResourceId", default, skip_serializing_if = "Option::is_none")]
    pub management_resource_id: Option<String>,
    #[serde(rename = "managementResourceTenantId", default, skip_serializing_if = "Option::is_none")]
    pub management_resource_tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(rename = "lengthHeightUnit", default, skip_serializing_if = "Option::is_none")]
    pub length_height_unit: Option<dimensions::LengthHeightUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[serde(rename = "weightUnit", default, skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<dimensions::WeightUnit>,
}
pub mod dimensions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LengthHeightUnit {
        #[serde(rename = "IN")]
        In,
        #[serde(rename = "CM")]
        Cm,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WeightUnit {
        #[serde(rename = "LBS")]
        Lbs,
        #[serde(rename = "KGS")]
        Kgs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DisplayInfo {
    #[serde(rename = "productFamilyDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub product_family_display_name: Option<String>,
    #[serde(rename = "configurationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub configuration_display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionPreferences {
    #[serde(rename = "doubleEncryptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub double_encryption_status: Option<encryption_preferences::DoubleEncryptionStatus>,
}
pub mod encryption_preferences {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DoubleEncryptionStatus {
        Disabled,
        Enabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterableProperty {
    #[serde(rename = "type")]
    pub type_: filterable_property::Type,
    #[serde(rename = "supportedValues")]
    pub supported_values: Vec<String>,
}
pub mod filterable_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ShipToCountries,
        DoubleEncryptionStatus,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardShippingDetails {
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[serde(rename = "carrierDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_display_name: Option<String>,
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchyInformation {
    #[serde(rename = "productFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub product_family_name: Option<String>,
    #[serde(rename = "productLineName", default, skip_serializing_if = "Option::is_none")]
    pub product_line_name: Option<String>,
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "configurationName", default, skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageInformation {
    #[serde(rename = "imageType", default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<image_information::ImageType>,
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}
pub mod image_information {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImageType {
        MainImage,
        BulletImage,
        GenericImage,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Link {
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<link::LinkType>,
    #[serde(rename = "linkUrl", default, skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
}
pub mod link {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LinkType {
        Generic,
        TermsAndConditions,
        Specification,
        Documentation,
        KnowMore,
        SignUp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementResourcePreferences {
    #[serde(rename = "preferredManagementResourceId", default, skip_serializing_if = "Option::is_none")]
    pub preferred_management_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterDetails {
    #[serde(rename = "billingType")]
    pub billing_type: meter_details::BillingType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(rename = "chargingType", default, skip_serializing_if = "Option::is_none")]
    pub charging_type: Option<meter_details::ChargingType>,
}
pub mod meter_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BillingType {
        Pav2,
        Purchase,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChargingType {
        PerOrder,
        PerDevice,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationPreference {
    #[serde(rename = "stageName")]
    pub stage_name: notification_preference::StageName,
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
}
pub mod notification_preference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        Shipped,
        Delivered,
    }
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
    pub origin: Option<operation::Origin>,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        Internal,
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
pub struct OrderItemDetails {
    #[serde(rename = "productDetails")]
    pub product_details: ProductDetails,
    #[serde(rename = "orderItemType")]
    pub order_item_type: order_item_details::OrderItemType,
    #[serde(rename = "currentStage", default, skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<StageDetails>,
    #[serde(rename = "orderItemStageHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub order_item_stage_history: Vec<StageDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[serde(rename = "forwardShippingDetails", default, skip_serializing_if = "Option::is_none")]
    pub forward_shipping_details: Option<ForwardShippingDetails>,
    #[serde(rename = "reverseShippingDetails", default, skip_serializing_if = "Option::is_none")]
    pub reverse_shipping_details: Option<ReverseShippingDetails>,
    #[serde(rename = "notificationEmailList", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_email_list: Vec<String>,
    #[serde(rename = "cancellationReason", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "cancellationStatus", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_status: Option<order_item_details::CancellationStatus>,
    #[serde(rename = "deletionStatus", default, skip_serializing_if = "Option::is_none")]
    pub deletion_status: Option<order_item_details::DeletionStatus>,
    #[serde(rename = "returnReason", default, skip_serializing_if = "Option::is_none")]
    pub return_reason: Option<String>,
    #[serde(rename = "returnStatus", default, skip_serializing_if = "Option::is_none")]
    pub return_status: Option<order_item_details::ReturnStatus>,
    #[serde(rename = "managementRpDetails", default, skip_serializing_if = "Option::is_none")]
    pub management_rp_details: Option<ResourceProviderDetails>,
    #[serde(rename = "managementRpDetailsList", default, skip_serializing_if = "Vec::is_empty")]
    pub management_rp_details_list: Vec<ResourceProviderDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
pub mod order_item_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrderItemType {
        Purchase,
        Rental,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CancellationStatus {
        Cancellable,
        CancellableWithFee,
        NotCancellable,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeletionStatus {
        Allowed,
        NotAllowed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReturnStatus {
        Returnable,
        ReturnableWithFee,
        NotReturnable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItemProperties {
    #[serde(rename = "orderItemDetails")]
    pub order_item_details: OrderItemDetails,
    #[serde(rename = "addressDetails")]
    pub address_details: AddressDetails,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "orderId")]
    pub order_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItemResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: OrderItemProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OrderItemResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemUpdateParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OrderItemUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemUpdateProperties {
    #[serde(rename = "forwardAddress", default, skip_serializing_if = "Option::is_none")]
    pub forward_address: Option<AddressProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[serde(rename = "notificationEmailList", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_email_list: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderProperties {
    #[serde(rename = "orderItemIds", default, skip_serializing_if = "Vec::is_empty")]
    pub order_item_ids: Vec<String>,
    #[serde(rename = "currentStage", default, skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<StageDetails>,
    #[serde(rename = "orderStageHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub order_stage_history: Vec<StageDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: OrderProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OrderResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pav2MeterDetails {
    #[serde(flatten)]
    pub meter_details: MeterDetails,
    #[serde(rename = "meterGuid", default, skip_serializing_if = "Option::is_none")]
    pub meter_guid: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Preferences {
    #[serde(rename = "notificationPreferences", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_preferences: Vec<NotificationPreference>,
    #[serde(rename = "transportPreferences", default, skip_serializing_if = "Option::is_none")]
    pub transport_preferences: Option<TransportPreferences>,
    #[serde(rename = "encryptionPreferences", default, skip_serializing_if = "Option::is_none")]
    pub encryption_preferences: Option<EncryptionPreferences>,
    #[serde(rename = "managementResourcePreferences", default, skip_serializing_if = "Option::is_none")]
    pub management_resource_preferences: Option<ManagementResourcePreferences>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductDetails {
    #[serde(rename = "displayInfo", default, skip_serializing_if = "Option::is_none")]
    pub display_info: Option<DisplayInfo>,
    #[serde(rename = "hierarchyInformation")]
    pub hierarchy_information: HierarchyInformation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "productDoubleEncryptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_double_encryption_status: Option<product_details::ProductDoubleEncryptionStatus>,
    #[serde(rename = "deviceDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub device_details: Vec<DeviceDetails>,
}
pub mod product_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProductDoubleEncryptionStatus {
        Disabled,
        Enabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamilies {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProductFamily>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamiliesMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProductFamiliesMetadataDetails>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamiliesMetadataDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductFamilyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductFamiliesRequest {
    #[serde(rename = "filterableProperties")]
    pub filterable_properties: serde_json::Value,
    #[serde(rename = "customerSubscriptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub customer_subscription_details: Option<CustomerSubscriptionDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamily {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductFamilyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamilyProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[serde(rename = "productLines", default, skip_serializing_if = "Vec::is_empty")]
    pub product_lines: Vec<ProductLine>,
    #[serde(rename = "resourceProviderDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_provider_details: Vec<ResourceProviderDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductLineProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLineProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<Product>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configurations: Vec<Configuration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseMeterDetails {
    #[serde(flatten)]
    pub meter_details: MeterDetails,
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
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
pub struct ResourceIdentity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDetails {
    #[serde(rename = "resourceProviderNamespace", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_namespace: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnOrderItemDetails {
    #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<AddressProperties>,
    #[serde(rename = "returnReason")]
    pub return_reason: String,
    #[serde(rename = "serviceTag", default, skip_serializing_if = "Option::is_none")]
    pub service_tag: Option<String>,
    #[serde(rename = "shippingBoxRequired", default, skip_serializing_if = "Option::is_none")]
    pub shipping_box_required: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReverseShippingDetails {
    #[serde(rename = "sasKeyForLabel", default, skip_serializing_if = "Option::is_none")]
    pub sas_key_for_label: Option<String>,
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[serde(rename = "carrierDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_display_name: Option<String>,
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[serde(rename = "streetAddress3", default, skip_serializing_if = "Option::is_none")]
    pub street_address3: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    pub country: String,
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "zipExtendedCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_extended_code: Option<String>,
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "addressType", default, skip_serializing_if = "Option::is_none")]
    pub address_type: Option<shipping_address::AddressType>,
}
pub mod shipping_address {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AddressType {
        None,
        Residential,
        Commercial,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShippingDetails {
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[serde(rename = "carrierDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_display_name: Option<String>,
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Specification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageDetails {
    #[serde(rename = "stageStatus", default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<stage_details::StageStatus>,
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<stage_details::StageName>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
pub mod stage_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        Failed,
        Cancelled,
        Cancelling,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        Placed,
        InReview,
        Confirmed,
        ReadyToShip,
        Shipped,
        Delivered,
        InUse,
        ReturnInitiated,
        ReturnPickedUp,
        ReturnedToMicrosoft,
        ReturnCompleted,
        Cancelled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransportPreferences {
    #[serde(rename = "preferredShipmentType")]
    pub preferred_shipment_type: transport_preferences::PreferredShipmentType,
}
pub mod transport_preferences {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredShipmentType {
        CustomerManaged,
        MicrosoftManaged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
