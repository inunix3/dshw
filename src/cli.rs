// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use crate::{app::Application, query::*};

use anyhow::Result;
pub use clap::{Parser, Subcommand};

/// Dead simple CLI program to query information about system and hardware.
/// Basically a CLI wrapper over the sysinfo Rust crate.
#[derive(Parser, Debug)]
#[command(
    author = "inunix3",
    version = "0.2.0",
    long_about = None,
)]
pub struct Cli {
    /// Interval between commands, ignored if --run_times is 1. For format see https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html.
    #[arg(short = 'I', long)]
    pub interval: Option<humantime::Duration>,
    /// How many times to run the command. Specifying 0 will cause commands to run infinitely until
    /// the user manually terminates the program.
    #[arg(short = 'n', long, default_value_t = 1, verbatim_doc_comment)]
    pub run_times: u32,
    /// Delimiter used for separating responses. Also used by `list-cpus` and `list-sensors` commands.
    #[arg(short, long, default_value = "\n")]
    pub delimiter: String,
    #[command(subcommand)]
    pub cmd: CliCommand,
    /// String with format specifiers which will be replaced by actual values. Syntax for format
    /// specifiers is `%<SPECIFIER>%`. To output the literal percent sign, write `%%`. If the specifier
    /// does not exist, a corresponding error is reported. Any supplied queries to the commands are
    /// ignored. The case does not matter (`%MAC-AddREss%` = `%mac-address%`).
    #[arg(short, long, verbatim_doc_comment)]
    pub fmt: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    Os {
        queries: Vec<OsQuery>,
    },
    Cpu {
        #[clap(value_parser)]
        name: String,
        queries: Vec<CpuQuery>,
    },
    Memory {
        queries: Vec<MemoryQuery>,
    },
    Swap {
        queries: Vec<SwapQuery>,
    },
    Drive {
        name: String,
        queries: Vec<DriveQuery>,
    },
    Sensor {
        name: String,
        queries: Vec<SensorQuery>,
    },
    Network {
        name: String,
        queries: Vec<NetworkQuery>,
    },
    /// List all available sensors.
    ListSensors,
    /// List all available CPUs.
    ListCpus,
    /// List all available network interfaces.
    ListNetworks,
}

impl CliCommand {
    pub fn exec(&self) -> Result<Vec<String>> {
        let mut output: Vec<String> = vec![];
        let mut app = Application::new();

        let (mut cmd, queries) = app.command_from_cli(self)?;

        if !queries.is_empty() {
            for q in queries {
                output.extend(cmd.exec(q));
            }
        } else {
            output.extend(cmd.exec(Query::None));
        }

        Ok(output)
    }
}
