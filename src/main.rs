// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: BSD-2-Clause

use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use env_logger::Env;
use rusoto_core::Region;
use eucalyptus_sdk_euprop::{DescribePropertiesRequest, Properties, PropertiesClient, Property};
use eucalyptus_sdk_euserv::{
    DescribeAvailableServiceTypesRequest, DescribeServicesRequest, ServiceId, ServiceType,
    Services, ServicesClient,
};

fn region(endpoint: &str) -> Region {
    Region::Custom {
        name: "eucalyptus".to_owned(),
        endpoint: endpoint.to_owned(),
    }
}

fn usage(matches: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    println!("{}", matches.usage());
    Ok(())
}

async fn properties_describe(
    region: Region,
    _matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn Error>> {
    let client = PropertiesClient::new(region);
    let describe_properties: DescribePropertiesRequest = Default::default();

    match client.describe_properties(describe_properties).await {
        Ok(response) => match response.properties {
            Some(properties_list) => {
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
                Ok(())
            }
            _ => Ok(()),
        },
        Err(error) => Err(Box::<dyn Error>::from(error)),
    }
}

async fn services_describe(
    region: Region,
    _matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn Error>> {
    let client = ServicesClient::new(region);
    let mut describe_services: DescribeServicesRequest = Default::default();
    describe_services.list_all = Some(true);

    match client.describe_services(describe_services).await {
        Ok(response) => match response.service_statuses {
            Some(service_list) => {
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
                Ok(())
            }
            _ => Ok(()),
        },
        Err(error) => Err(Box::<dyn Error>::from(error)),
    }
}

async fn services_describe_types(
    region: Region,
    _matches: &ArgMatches<'_>,
) -> Result<(), Box<dyn Error>> {
    let client = ServicesClient::new(region);
    let describe_service_types: DescribeAvailableServiceTypesRequest = Default::default();

    match client
        .describe_available_service_types(describe_service_types)
        .await
    {
        Ok(response) => match response.available {
            Some(service_types) => {
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
                Ok(())
            }
            _ => Ok(()),
        },
        Err(error) => Err(Box::<dyn Error>::from(error)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const CMD_PROPERTIES: &str = "properties";
    const CMD_SERVICES: &str = "services";

    const SUBCMD_DESCRIBE: &str = "describe";
    const SUBCMD_DESCRIBE_TYPES: &str = "describe-types";

    let matches = App::new("eucalyptus-sdk-rust")
        .version("0.1")
        .about("Example Eucalyptus CLI demonstrating some SDK functionality")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("show request/repsonse debug output")
                .global(true),
        )
        .arg(
            Arg::with_name("endpoint")
                .short("e")
                .long("endpoint")
                .takes_value(true)
                .value_name("URL")
                .help("endpoint url")
                .global(true),
        )
        .subcommand(
            SubCommand::with_name(CMD_PROPERTIES)
                .about("Properties service commands")
                .subcommand(SubCommand::with_name(SUBCMD_DESCRIBE)),
        )
        .subcommand(
            SubCommand::with_name(CMD_SERVICES)
                .about("Service management service commands")
                .subcommand(SubCommand::with_name(SUBCMD_DESCRIBE))
                .subcommand(SubCommand::with_name(SUBCMD_DESCRIBE_TYPES)),
        )
        .get_matches();

    let endpoint = matches.value_of("endpoint");
    if matches.is_present("debug") {
        let _ = env_logger::try_init_from_env(Env::default().default_filter_or("rusoto"));
    }
    let result = match matches.subcommand() {
        (CMD_PROPERTIES, Some(matches)) => {
            let region = region(endpoint.unwrap_or("http://127.0.0.1:8773/services/Properties"));
            match matches.subcommand() {
                (SUBCMD_DESCRIBE, Some(matches)) => properties_describe(region, matches).await,
                _ => usage(&matches),
            }
        }
        (CMD_SERVICES, Some(matches)) => {
            let region = region(endpoint.unwrap_or("http://127.0.0.1:8773/services/Empyrean"));
            match matches.subcommand() {
                (SUBCMD_DESCRIBE, Some(matches)) => services_describe(region, matches).await,
                (SUBCMD_DESCRIBE_TYPES, Some(_matches)) => {
                    services_describe_types(region, matches).await
                }
                _ => usage(&matches),
            }
        }
        _ => usage(&matches),
    };

    if let Err(error) = result {
        println!("Error: {:?}", error)
    }

    Ok(())
}
