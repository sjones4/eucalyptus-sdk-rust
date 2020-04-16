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
pub struct DescribePropertiesRequest {
    pub properties: Option<Vec<String>>,
}

/// Serialize `DescribePropertiesRequest` contents to a `SignedRequest`.
struct DescribePropertiesRequestSerializer;
impl DescribePropertiesRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribePropertiesRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.properties {
            PropertyPrefixListTypeSerializer::serialize(
                params,
                &format!("{}{}", prefix, "properties"),
                field_value,
            );
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribePropertiesResponse {
    pub properties: Option<Vec<Property>>,
}

struct DescribePropertiesResponseDeserializer;
impl DescribePropertiesResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribePropertiesResponse, XmlParseError> {
        deserialize_elements::<_, DescribePropertiesResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "properties" => {
                        obj.properties.get_or_insert(vec![]).extend(
                            PropertiesListDeserializer::deserialize("properties", stack)?,
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
pub struct ModifyPropertyRequest {
    pub name: String,
    pub reset: Option<bool>,
    pub value: Option<String>,
}

/// Serialize `ModifyPropertyRequest` contents to a `SignedRequest`.
struct ModifyPropertyRequestSerializer;
impl ModifyPropertyRequestSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyPropertyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.reset {
            params.put(&format!("{}{}", prefix, "Reset"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyPropertyResponse {
    pub name: Option<String>,
    pub old_value: Option<String>,
    pub value: Option<String>,
}

struct ModifyPropertyResponseDeserializer;
impl ModifyPropertyResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyPropertyResponse, XmlParseError> {
        deserialize_elements::<_, ModifyPropertyResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "name" => {
                    obj.name = Some(StringDeserializer::deserialize("name", stack)?);
                }
                "oldValue" => {
                    obj.old_value = Some(StringDeserializer::deserialize("oldValue", stack)?);
                }
                "value" => {
                    obj.value = Some(StringDeserializer::deserialize("value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct PropertiesListDeserializer;
impl PropertiesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Property>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "item" {
                obj.push(PropertyDeserializer::deserialize("item", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Property {
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub read_only: Option<bool>,
    pub value: Option<String>,
}

struct PropertyDeserializer;
impl PropertyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Property, XmlParseError> {
        deserialize_elements::<_, Property, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "defaultValue" => {
                    obj.default_value =
                        Some(StringDeserializer::deserialize("defaultValue", stack)?);
                }
                "description" => {
                    obj.description = Some(StringDeserializer::deserialize("description", stack)?);
                }
                "name" => {
                    obj.name = Some(StringDeserializer::deserialize("name", stack)?);
                }
                "readOnly" => {
                    obj.read_only = Some(BooleanDeserializer::deserialize("readOnly", stack)?);
                }
                "value" => {
                    obj.value = Some(StringDeserializer::deserialize("value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `PropertyPrefixListType` contents to a `SignedRequest`.
struct PropertyPrefixListTypeSerializer;
impl PropertyPrefixListTypeSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.{}", name, index + 1);
            params.put(&key, &obj);
        }
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
/// Errors returned by DescribeProperties
#[derive(Debug, PartialEq)]
pub enum DescribePropertiesError {
    ServiceFailure(String),
}

impl DescribePropertiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePropertiesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ServiceFailure" => {
                        return RusotoError::Service(DescribePropertiesError::ServiceFailure(
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
impl fmt::Display for DescribePropertiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePropertiesError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePropertiesError {}
/// Errors returned by ModifyPropertyValue
#[derive(Debug, PartialEq)]
pub enum ModifyPropertyValueError {
    ModifyProperty(String),

    ServiceFailure(String),
}

impl ModifyPropertyValueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyPropertyValueError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ModifyPropertyValueType" => {
                        return RusotoError::Service(ModifyPropertyValueError::ModifyProperty(
                            parsed_error.message,
                        ))
                    }
                    "ServiceFailure" => {
                        return RusotoError::Service(ModifyPropertyValueError::ServiceFailure(
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
impl fmt::Display for ModifyPropertyValueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyPropertyValueError::ModifyProperty(ref cause) => write!(f, "{}", cause),
            ModifyPropertyValueError::ServiceFailure(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyPropertyValueError {}
/// Trait representing the capabilities of the Eucalyptus Properties API. Eucalyptus Properties clients implement this trait.
#[async_trait]
pub trait Properties {
    async fn describe_properties(
        &self,
        input: DescribePropertiesRequest,
    ) -> Result<DescribePropertiesResponse, RusotoError<DescribePropertiesError>>;

    async fn modify_property_value(
        &self,
        input: ModifyPropertyRequest,
    ) -> Result<ModifyPropertyResponse, RusotoError<ModifyPropertyValueError>>;
}
/// A client for the Eucalyptus Properties API.
#[derive(Clone)]
pub struct PropertiesClient {
    client: Client,
    region: region::Region,
}

impl PropertiesClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PropertiesClient {
        PropertiesClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PropertiesClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PropertiesClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PropertiesClient {
        PropertiesClient { client, region }
    }
}

#[async_trait]
impl Properties for PropertiesClient {
    async fn describe_properties(
        &self,
        input: DescribePropertiesRequest,
    ) -> Result<DescribePropertiesResponse, RusotoError<DescribePropertiesError>> {
        let mut request = SignedRequest::new("POST", "properties", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeProperties");
        params.put("Version", "2010-01-01");
        DescribePropertiesRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(DescribePropertiesError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = DescribePropertiesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result =
                DescribePropertiesResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }

    async fn modify_property_value(
        &self,
        input: ModifyPropertyRequest,
    ) -> Result<ModifyPropertyResponse, RusotoError<ModifyPropertyValueError>> {
        let mut request = SignedRequest::new("POST", "properties", &self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyPropertyValue");
        params.put("Version", "2010-01-01");
        ModifyPropertyRequestSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(ModifyPropertyValueError::from_response(response));
        }

        let xml_response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        let result;

        if xml_response.body.is_empty() {
            result = ModifyPropertyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                xml_response.body.as_ref(),
                ParserConfig::new().trim_whitespace(false),
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = peek_at_name(&mut stack)?;
            result = ModifyPropertyResponseDeserializer::deserialize(&actual_tag_name, &mut stack)?;
        }
        // parse non-payload
        Ok(result)
    }
}
