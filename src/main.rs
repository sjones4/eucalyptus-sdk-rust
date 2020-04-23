// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: BSD-2-Clause

use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use env_logger::Env;
use eucalyptus_sdk::{properties_describe, services_describe, services_describe_types};

const CMD_PROPERTIES: &str = "properties";
const CMD_SERVICES: &str = "services";

const SUBCMD_DESCRIBE: &str = "describe";
const SUBCMD_DESCRIBE_TYPES: &str = "describe-types";

fn usage(matches: &ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
    println!("{}", matches.usage());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
        (CMD_PROPERTIES, Some(matches)) => match matches.subcommand() {
            (SUBCMD_DESCRIBE, Some(matches)) => properties_describe(&endpoint, matches).await,
            _ => usage(&matches),
        },
        (CMD_SERVICES, Some(matches)) => match matches.subcommand() {
            (SUBCMD_DESCRIBE, Some(matches)) => services_describe(&endpoint, matches).await,
            (SUBCMD_DESCRIBE_TYPES, Some(_matches)) => {
                services_describe_types(&endpoint, matches).await
            }
            _ => usage(&matches),
        },
        _ => usage(&matches),
    };

    if let Err(error) = result {
        println!("Error: {:?}", error)
    }

    Ok(())
}
