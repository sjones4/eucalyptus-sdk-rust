// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: BSD-2-Clause

use std::error::Error;

use clap::ArgMatches;
use eucalyptus_sdk_euprop::{Properties, PropertiesClient, Property};
use eucalyptus_sdk_euserv::{
    DescribeServicesRequest, ServiceId, ServiceType, Services, ServicesClient,
};
use rusoto_core::Region;

const ENDPOINT_PROPERTIES: &str = "http://127.0.0.1:8773/services/Properties";
const ENDPOINT_SERVICES: &str = "http://127.0.0.1:8773/services/Empyrean";

fn region(endpoint: &str) -> Region {
    Region::Custom {
        name: String::from("eucalyptus"),
        endpoint: String::from(endpoint),
    }
}

pub async fn properties_describe(
    endpoint: &Option<&str>,
    _matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn Error>> {
    let region = region(endpoint.unwrap_or(ENDPOINT_PROPERTIES));
    let client = PropertiesClient::new(region);

    if let Some(properties_list) = client
        .describe_properties(Default::default())
        .await?
        .properties
    {
        for property in properties_list {
            if let Property {
                name: Some(name),
                value: Some(value),
                ..
            } = property
            {
                println!("PROPERTY\t{}\t{}", name, value)
            }
        }
    }

    Ok(())
}

pub async fn services_describe(
    endpoint: &Option<&str>,
    _matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn Error>> {
    let region = region(endpoint.unwrap_or(ENDPOINT_SERVICES));
    let client = ServicesClient::new(region);

    if let Some(service_list) = client
        .describe_services(DescribeServicesRequest {
            list_all: Some(true),
            ..Default::default()
        })
        .await?
        .service_statuses
    {
        for service in service_list {
            if let (
                Some(state),
                Some(ServiceId {
                    type_: Some(type_),
                    partition: Some(part),
                    name: Some(name),
                    ..
                }),
            ) = (service.local_state, service.service_id)
            {
                println!("SERVICE\t{}\t{}\t{}\t{}", type_, part, name, state)
            }
        }
    }

    Ok(())
}

pub async fn services_describe_types(
    endpoint: &Option<&str>,
    _matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn Error>> {
    let region = region(endpoint.unwrap_or(ENDPOINT_SERVICES));
    let client = ServicesClient::new(region);

    if let Some(service_types) = client
        .describe_available_service_types(Default::default())
        .await?
        .available
    {
        for service_type in service_types {
            if let ServiceType {
                component_name: Some(component_name),
                service_group_members: Some(members),
                description: Some(description),
                ..
            } = service_type
            {
                println!(
                    "SVCTYPE\t{}\t{}\t{}",
                    component_name,
                    !members.is_empty(),
                    description,
                )
            }
        }
    }

    Ok(())
}
