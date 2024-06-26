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
    #[arg(short, long, default_value = "\n")]
    /// Delimiter used for separating responses. Also used by `list-cpus` and `list-sensors` commands.
    pub delimiter: String,
    #[command(subcommand)]
    pub cmd: CliCommand,
    /// String with format specifiers which will be replaced by actual values. Syntax for format
    /// specifiers is `%<SPECIFIER>%`. To output the literal percent sign, write `%%`. If the specifier
    /// does not exist, a corresponding error is reported. Any supplied queries to the commands are
    /// ignored.
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
