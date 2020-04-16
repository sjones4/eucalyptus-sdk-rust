// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
#[cfg(feature = "deserialize_structs")]
use serde::Deserialize;
#[cfg(feature = "serialize_structs")]
use serde::Serialize;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterServiceRequest {
    pub name: String,
    pub type_: Option<String>,
}

/// Serialize `DeregisterServiceRequest` contents to a `SignedRequest`.
struct DeregisterServiceRequestSerializer;
impl DeregisterServiceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeregisterServiceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.type_ {
            params.put(&format!("{}{}", prefix, "Type"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeregisterServiceResponse {
    pub deregistered_services: Option<Vec<ServiceId>>,
    pub registration_metadata: Option<RegistrationMetadata>,
}

struct DeregisterServiceResponseDeserializer;
impl DeregisterServiceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeregisterServiceResponse, XmlParseError> {
        deserialize_elements::<_, DeregisterServiceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "deregisteredServices" => {
                        obj.deregistered_services.get_or_insert(vec![]).extend(
                            ServiceIdListDeserializer::deserialize("deregisteredServices", stack)?,
                        );
                    }
                    "ServiceRegistrationMessage" => {
                        obj.registration_metadata =
                            Some(RegistrationMetadataDeserializer::deserialize(
                                "ServiceRegistrationMessage",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAvailableServiceTypesRequest {
    pub verbose: Option<bool>,
}

/// Serialize `DescribeAvailableServiceTypesRequest` contents to a `SignedRequest`.
struct DescribeAvailableServiceTypesRequestSerializer;
impl DescribeAvailableServiceTypesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeAvailableServiceTypesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.verbose {
            params.put(&format!("{}{}", prefix, "Verbose"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeAvailableServiceTypesResponse {
    pub available: Option<Vec<ServiceType>>,
}

struct DescribeAvailableServiceTypesResponseDeserializer;
impl DescribeAvailableServiceTypesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAvailableServiceTypesResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAvailableServiceTypesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "available" => {
                        obj.available.get_or_insert(vec![]).extend(
                            ServiceTypeListDeserializer::deserialize("available", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServiceCertificatesRequest {
    pub filters: Option<Vec<Filter>>,
    pub fingerprint_digest: Option<String>,
    pub format: Option<String>,
}

/// Serialize `DescribeServiceCertificatesRequest` contents to a `SignedRequest`.
struct DescribeServiceCertificatesRequestSerializer;
impl DescribeServiceCertificatesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeServiceCertificatesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "filters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.fingerprint_digest {
            params.put(&format!("{}{}", prefix, "FingerprintDigest"), &field_value);
        }
        if let Some(ref field_value) = obj.format {
            params.put(&format!("{}{}", prefix, "Format"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeServiceCertificatesResponse {
    pub metadata: Option<ResponseMetadata>,
    pub service_certificates: Option<Vec<ServiceCertificate>>,
}

struct DescribeServiceCertificatesResponseDeserializer;
impl DescribeServiceCertificatesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeServiceCertificatesResponse, XmlParseError> {
        deserialize_elements::<_, DescribeServiceCertificatesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EmpyreanMessage" => {
                        obj.metadata = Some(ResponseMetadataDeserializer::deserialize(
                            "EmpyreanMessage",
                            stack,
                        )?);
                    }
                    "serviceCertificates" => {
                        obj.service_certificates.get_or_insert(vec![]).extend(
                            ServiceCertificateListDeserializer::deserialize(
                                "serviceCertificates",
                                stack,
                            )?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServicesRequest {
    pub filters: Option<Vec<Filter>>,
    pub list_all: Option<bool>,
    pub services: Option<Vec<String>>,
}

/// Serialize `DescribeServicesRequest` contents to a `SignedRequest`.
struct DescribeServicesRequestSerializer;
impl DescribeServicesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeServicesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "filters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.list_all {
            params.put(&format!("{}{}", prefix, "ListAll"), &field_value);
        }
        if let Some(ref field_value) = obj.services {
            ServiceNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "services"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeServicesResponse {
    pub metadata: Option<ResponseMetadata>,
    pub service_statuses: Option<Vec<ServiceStatus>>,
}

struct DescribeServicesResponseDeserializer;
impl DescribeServicesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeServicesResponse, XmlParseError> {
        deserialize_elements::<_, DescribeServicesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "EmpyreanMessage" => {
                        obj.metadata = Some(ResponseMetadataDeserializer::deserialize(
                            "EmpyreanMessage",
                            stack,
                        )?);
                    }
                    "serviceStatuses" => {
                        obj.service_statuses.get_or_insert(vec![]).extend(
                            ServiceStatusListDeserializer::deserialize("serviceStatuses", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    pub name: String,
    pub values: Option<Vec<String>>,
}

/// Serialize `Filter` contents to a `SignedRequest`.
struct FilterSerializer;
impl FilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Filter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.values {
            FilterValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "values"),
                field_value,
            );
        }
    }
}

/// Serialize `FilterList` contents to a `SignedRequest`.
struct FilterListSerializer;
impl FilterListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<Filter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            FilterSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `FilterValueList` contents to a `SignedRequest`.
struct FilterValueListSerializer;
impl FilterValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyServiceRequest {
    pub name: String,
    pub state: String,
}

/// Serialize `ModifyServiceRequest` contents to a `SignedRequest`.
struct ModifyServiceRequestSerializer;
impl ModifyServiceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyServiceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "State"), &obj.state);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyServiceResponse {
    pub metadata: Option<ResponseMetadata>,
}

struct ModifyServiceResponseDeserializer;
impl ModifyServiceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyServiceResponse, XmlParseError> {
        deserialize_elements::<_, ModifyServiceResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EmpyreanMessage" => {
                    obj.metadata = Some(ResponseMetadataDeserializer::deserialize(
                        "EmpyreanMessage",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterServiceRequest {
    pub host: String,
    pub name: String,
    pub partition: Option<String>,
    pub port: i64,
    pub type_: String,
}

/// Serialize `RegisterServiceRequest` contents to a `SignedRequest`.
struct RegisterServiceRequestSerializer;
impl RegisterServiceRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RegisterServiceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Host"), &obj.host);
        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.partition {
            params.put(&format!("{}{}", prefix, "Partition"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Port"), &obj.port);
        params.put(&format!("{}{}", prefix, "Type"), &obj.type_);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RegisterServiceResponse {
    pub registered_services: Option<Vec<ServiceId>>,
    pub registration_metadata: Option<RegistrationMetadata>,
}

struct RegisterServiceResponseDeserializer;
impl RegisterServiceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RegisterServiceResponse, XmlParseError> {
        deserialize_elements::<_, RegisterServiceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "registeredServices" => {
                        obj.registered_services.get_or_insert(vec![]).extend(
                            ServiceIdListDeserializer::deserialize("registeredServices", stack)?,
                        );
                    }
                    "ServiceRegistrationMessage" => {
                        obj.registration_metadata =
                            Some(RegistrationMetadataDeserializer::deserialize(
                                "ServiceRegistrationMessage",
                                stack,
                            )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RegistrationMetadata {
    pub reponse_metadata: Option<ResponseMetadata>,
    pub status_messages: Option<Vec<StringEntry>>,
}

struct RegistrationMetadataDeserializer;
impl RegistrationMetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RegistrationMetadata, XmlParseError> {
        deserialize_elements::<_, RegistrationMetadata, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EmpyreanMessage" => {
                    obj.reponse_metadata = Some(ResponseMetadataDeserializer::deserialize(
                        "EmpyreanMessage",
                        stack,
                    )?);
                }
                "statusMessages" => {
                    obj.status_messages.get_or_insert(vec![]).extend(
                        StringEntryListDeserializer::deserialize("statusMessages", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ResponseMetadata {
    pub request_id: Option<String>,
    pub return_: Option<bool>,
}

struct ResponseMetadataDeserializer;
impl ResponseMetadataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ResponseMetadata, XmlParseError> {
        deserialize_elements::<_, ResponseMetadata, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "correlationId" => {
                    obj.request_id = Some(StringDeserializer::deserialize("correlationId", stack)?);
                }
                "_return" => {
                    obj.return_ = Some(BooleanDeserializer::deserialize("_return", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceCertificate {
    pub certificate: Option<String>,
    pub certificate_fingerprint: Option<String>,
    pub certificate_fingerprint_digest: Option<String>,
    pub certificate_format: Option<String>,
    pub certificate_usage: Option<String>,
    pub service_type: Option<String>,
}

struct ServiceCertificateDeserializer;
impl ServiceCertificateDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceCertificate, XmlParseError> {
        deserialize_elements::<_, ServiceCertificate, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "certificate" => {
                    obj.certificate = Some(StringDeserializer::deserialize("certificate", stack)?);
                }
                "certificateFingerprint" => {
                    obj.certificate_fingerprint = Some(StringDeserializer::deserialize(
                        "certificateFingerprint",
                        stack,
                    )?);
                }
                "certificateFingerprintDigest" => {
                    obj.certificate_fingerprint_digest = Some(StringDeserializer::deserialize(
                        "certificateFingerprintDigest",
                        stack,
                    )?);
                }
                "certificateFormat" => {
                    obj.certificate_format =
                        Some(StringDeserializer::deserialize("certificateFormat", stack)?);
                }
                "certificateUsage" => {
                    obj.certificate_usage =
                        Some(StringDeserializer::deserialize("certificateUsage", stack)?);
                }
                "serviceType" => {
                    obj.service_type = Some(StringDeserializer::deserialize("serviceType", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ServiceCertificateListDeserializer;
impl ServiceCertificateListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServiceCertificate>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "item" {
                obj.push(ServiceCertificateDeserializer::deserialize("item", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceId {
    pub full_name: Option<String>,
    pub host: Option<String>,
    pub name: Option<String>,
    pub partition: Option<String>,
    pub type_: Option<String>,
    pub uri: Option<String>,
}

struct ServiceIdDeserializer;
impl ServiceIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceId, XmlParseError> {
        deserialize_elements::<_, ServiceId, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "fullName" => {
                    obj.full_name = Some(StringDeserializer::deserialize("fullName", stack)?);
                }
                "host" => {
                    obj.host = Some(StringDeserializer::deserialize("host", stack)?);
                }
                "name" => {
                    obj.name = Some(StringDeserializer::deserialize("name", stack)?);
                }
                "partition" => {
                    obj.partition = Some(StringDeserializer::deserialize("partition", stack)?);
                }
                "type" => {
                    obj.type_ = Some(StringDeserializer::deserialize("type", stack)?);
                }
                "uri" => {
                    obj.uri = Some(StringDeserializer::deserialize("uri", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ServiceIdListDeserializer;
impl ServiceIdListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServiceId>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "item" {
                obj.push(ServiceIdDeserializer::deserialize("item", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `ServiceNameList` contents to a `SignedRequest`.
struct ServiceNameListSerializer;
impl ServiceNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceStatus {
    pub local_state: Option<String>,
    pub service_id: Option<ServiceId>,
}

struct ServiceStatusDeserializer;
impl ServiceStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceStatus, XmlParseError> {
        deserialize_elements::<_, ServiceStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "localState" => {
                    obj.local_state = Some(StringDeserializer::deserialize("localState", stack)?);
                }
                "serviceId" => {
                    obj.service_id = Some(ServiceIdDeserializer::deserialize("serviceId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ServiceStatusListDeserializer;
impl ServiceStatusListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServiceStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "item" {
                obj.push(ServiceStatusDeserializer::deserialize("item", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceType {
    pub component_capitalized_name: Option<String>,
    pub component_name: Option<String>,
    pub description: Option<String>,
    pub has_credentials: Option<bool>,
    pub partitioned: Option<bool>,
    pub public_api_service: Option<bool>,
    pub registerable: Option<bool>,
    pub requires_name: Option<bool>,
    pub service_group_members: Option<Vec<StringEntry>>,
    pub service_groups: Option<Vec<StringEntry>>,
}

struct ServiceTypeDeserializer;
impl ServiceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceType, XmlParseError> {
        deserialize_elements::<_, ServiceType, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "componentCapitalizedName" => {
                    obj.component_capitalized_name = Some(StringDeserializer::deserialize(
                        "componentCapitalizedName",
                        stack,
                    )?);
                }
                "componentName" => {
                    obj.component_name =
                        Some(StringDeserializer::deserialize("componentName", stack)?);
                }
                "description" => {
                    obj.description = Some(StringDeserializer::deserialize("description", stack)?);
                }
                "hasCredentials" => {
                    obj.has_credentials =
                        Some(BooleanDeserializer::deserialize("hasCredentials", stack)?);
                }
                "partitioned" => {
                    obj.partitioned = Some(BooleanDeserializer::deserialize("partitioned", stack)?);
                }
                "publicApiService" => {
                    obj.public_api_service =
                        Some(BooleanDeserializer::deserialize("publicApiService", stack)?);
                }
                "registerable" => {
                    obj.registerable =
                        Some(BooleanDeserializer::deserialize("registerable", stack)?);
                }
                "requiresName" => {
                    obj.requires_name =
                        Some(BooleanDeserializer::deserialize("requiresName", stack)?);
                }
                "serviceGroupMembers" => {
                    obj.service_group_members.get_or_insert(vec![]).extend(
                        StringEntryListDeserializer::deserialize("serviceGroupMembers", stack)?,
                    );
                }
                "serviceGroups" => {
                    obj.service_groups.get_or_insert(vec![]).extend(
                        StringEntryListDeserializer::deserialize("serviceGroups", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct ServiceTypeListDeserializer;
impl ServiceTypeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServiceType>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "item" {
                obj.push(ServiceTypeDeserializer::deserialize("item", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StringEntry {
    pub entry: Option<String>,
}

struct StringEntryDeserializer;
impl StringEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StringEntry, XmlParseError> {
        deserialize_elements::<_, StringEntry, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "entry" => {
                    obj.entry = Some(StringDeserializer::deserialize("entry", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct StringEntryListDeserializer;
impl StringEntryListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<StringEntry>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "item" {
                obj.push(StringEntryDeserializer::deserialize("item", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// Errors returned by DeregisterService
#[derive(Debug, PartialEq)]
pub enum DeregisterServiceError {
    ServiceFailure(String),
}

impl DeregisterServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterServiceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ServiceFailure" => {
                        return RusotoError::Service(DeregisterServiceError::ServiceFailure(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeregisterServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterServiceError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterServiceError {}
/// Errors returned by DescribeAvailableServiceTypes
#[derive(Debug, PartialEq)]
pub enum DescribeAvailableServiceTypesError {
    ServiceFailure(String),
}

impl DescribeAvailableServiceTypesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAvailableServiceTypesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ServiceFailure" => {
                        return RusotoError::Service(
                            DescribeAvailableServiceTypesError::ServiceFailure(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAvailableServiceTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAvailableServiceTypesError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAvailableServiceTypesError {}
/// Errors returned by DescribeServiceCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeServiceCertificatesError {
    ServiceFailure(String),
}

impl DescribeServiceCertificatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeServiceCertificatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ServiceFailure" => {
                        return RusotoError::Service(
                            DescribeServiceCertificatesError::ServiceFailure(parsed_error.message),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeServiceCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServiceCertificatesError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServiceCertificatesError {}
/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    ServiceFailure(String),
}

impl DescribeServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServicesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ServiceFailure" => {
                        return RusotoError::Service(DescribeServicesError::ServiceFailure(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServicesError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeServicesError {}
/// Errors returned by ModifyService
#[derive(Debug, PartialEq)]
pub enum ModifyServiceError {
    ModifyService(String),

    ServiceFailure(String),
}

impl ModifyServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyServiceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ModifyServiceType" => {
                        return RusotoError::Service(ModifyServiceError::ModifyService(
                            parsed_error.message,
                        ))
                    }
                    "ServiceFailure" => {
                        return RusotoError::Service(ModifyServiceError::ServiceFailure(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyServiceError::ModifyService(ref cause) => write!(f, "{}", cause),
            ModifyServiceError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyServiceError {}
/// Errors returned by RegisterService
#[derive(Debug, PartialEq)]
pub enum RegisterServiceError {
    ServiceFailure(String),
}

impl RegisterServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterServiceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ServiceFailure" => {
                        return RusotoError::Service(RegisterServiceError::ServiceFailure(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RegisterServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterServiceError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterServiceError {}
/// Trait representing the capabilities of the Eucalyptus Services API. Eucalyptus Services clients implement this trait.
#[async_trait]
pub trait Services {
    async fn deregister_service(
        &self,
        input: DeregisterServiceRequest,
    ) -> Result<DeregisterServiceResponse, RusotoError<DeregisterServiceError>>;

    async fn describe_available_service_types(
        &self,
        input: DescribeAvailableServiceTypesRequest,
    ) -> Result<
        DescribeAvailableServiceTypesResponse,
        RusotoError<DescribeAvailableServiceTypesError>,
    >;

    async fn describe_service_certificates(
        &self,
        input: DescribeServiceCertificatesRequest,
    ) -> Result<DescribeServiceCertificatesResponse, RusotoError<DescribeServiceCertificatesError>>;

    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>>;

    async fn modify_service(
        &self,
        input: ModifyServiceRequest,
    ) -> Result<ModifyServiceResponse, RusotoError<ModifyServiceError>>;

    async fn register_service(
        &self,
        input: RegisterServiceRequest,
    ) -> Result<RegisterServiceResponse, RusotoError<RegisterServiceError>>;
}
/// A client for the Eucalyptus Services API.
#[derive(Clone)]
pub struct ServicesClient {
    client: Client,
    region: region::Region,
}

impl ServicesClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServicesClient {
        ServicesClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServicesClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ServicesClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ServicesClient {
        ServicesClient { client, region }
    }
}

#[async_trait]
impl Services for ServicesClient {
    async fn deregister_service(
        &self,
        input: DeregisterServiceRequest,
    ) -> Result<DeregisterServiceResponse, RusotoError<DeregisterServiceError>> {
        let mut request = SignedRequest::new("POST", "bootstrap", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeregisterService");
        params.put("Version", "2010-01-01");
        DeregisterServiceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DeregisterServiceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DeregisterServiceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                DeregisterServiceResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    async fn describe_available_service_types(
        &self,
        input: DescribeAvailableServiceTypesRequest,
    ) -> Result<
        DescribeAvailableServiceTypesResponse,
        RusotoError<DescribeAvailableServiceTypesError>,
    > {
        let mut request = SignedRequest::new("POST", "bootstrap", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAvailableServiceTypes");
        params.put("Version", "2010-01-01");
        DescribeAvailableServiceTypesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeAvailableServiceTypesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeAvailableServiceTypesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DescribeAvailableServiceTypesResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    async fn describe_service_certificates(
        &self,
        input: DescribeServiceCertificatesRequest,
    ) -> Result<DescribeServiceCertificatesResponse, RusotoError<DescribeServiceCertificatesError>>
    {
        let mut request = SignedRequest::new("POST", "bootstrap", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeServiceCertificates");
        params.put("Version", "2010-01-01");
        DescribeServiceCertificatesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeServiceCertificatesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeServiceCertificatesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = DescribeServiceCertificatesResponseDeserializer::deserialize(
                &actual_tag_name,
                &mut stack,
            )?;
        }
        // parse non-payload
        Ok(result)
    }

    async fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> Result<DescribeServicesResponse, RusotoError<DescribeServicesError>> {
        let mut request = SignedRequest::new("POST", "bootstrap", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeServices");
        params.put("Version", "2010-01-01");
        DescribeServicesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribeServicesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribeServicesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                DescribeServicesResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    async fn modify_service(
        &self,
        input: ModifyServiceRequest,
    ) -> Result<ModifyServiceResponse, RusotoError<ModifyServiceError>> {
        let mut request = SignedRequest::new("POST", "bootstrap", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyService");
        params.put("Version", "2010-01-01");
        ModifyServiceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyServiceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyServiceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ModifyServiceResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    async fn register_service(
        &self,
        input: RegisterServiceRequest,
    ) -> Result<RegisterServiceResponse, RusotoError<RegisterServiceError>> {
        let mut request = SignedRequest::new("POST", "bootstrap", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RegisterService");
        params.put("Version", "2010-01-01");
        RegisterServiceRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(RegisterServiceError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = RegisterServiceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                RegisterServiceResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}
