// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use crate::{
    cli::{Cli, CliCommand},
    cmd::*,
    query::Query,
};

use anyhow::{Context, Result};
use once_cell::unsync::Lazy;
use regex::{Captures, Regex};
use sysinfo::{Components, Disks, Networks, System};
use unescaper::unescape;

use std::{collections::HashMap, thread};

type FmtContext = HashMap<String, String>;

#[derive(Debug)]
pub struct Application {
    pub sys: System,
    pub drives: Lazy<Disks>,
    pub sensors: Lazy<Components>,
    pub networks: Lazy<Networks>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            sys: System::new(),
            drives: Lazy::new(Disks::new_with_refreshed_list),
            sensors: Lazy::new(Components::new_with_refreshed_list),
            networks: Lazy::new(Networks::new_with_refreshed_list),
        }
    }
}

impl Application {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(mut self, cli: Cli) -> Result<()> {
        if cli.run_times == 1 {
            return self.exec_cmd(&cli);
        }

        let mut cnt = 0u64;
        loop {
            if cli.run_times > 0 && cnt >= cli.run_times {
                break;
            }

            self.exec_cmd(&cli)?;

            if let Some(i) = cli.interval {
                thread::sleep(*i);
            }

            if cli.run_times != 0 {
                cnt += 1;
            }
        }

        Ok(())
    }

    pub fn command_from_cli<'a>(
        &'a mut self,
        cli_cmd: &CliCommand,
    ) -> Result<(Box<dyn Command + 'a>, Vec<Query>)> {
        match cli_cmd {
            CliCommand::Os { queries } => Ok((
                Box::new(OsCommand::new(self)),
                queries.iter().map(|q| Query::Os(q.clone())).collect(),
            )),
            CliCommand::Cpu { name, queries } => {
                self.refresh_cpus();

                let cpu = self
                    .sys
                    .cpus()
                    .iter()
                    .find(|c| c.name() == name)
                    .with_context(|| format!("cpu `{}` not found", name))?;

                Ok((
                    Box::new(CpuCommand::new(cpu)),
                    queries.iter().map(|q| Query::Cpu(q.clone())).collect(),
                ))
            }
            CliCommand::Memory { queries } => {
                self.sys.refresh_memory();

                Ok((
                    Box::new(MemoryCommand::new(self)),
                    queries.iter().map(|q| Query::Memory(q.clone())).collect(),
                ))
            }
            CliCommand::Swap { queries } => {
                self.sys.refresh_memory();

                Ok((
                    Box::new(SwapCommand::new(self)),
                    queries.iter().map(|q| Query::Swap(q.clone())).collect(),
                ))
            }
            CliCommand::Drive { name, queries } => {
                let drive = self
                    .drives
                    .list()
                    .iter()
                    .find(|d| d.name() == name.as_str())
                    .with_context(|| format!("drive '{}' not found", name))?;

                Ok((
                    Box::new(DriveCommand::new(drive)),
                    queries.iter().map(|q| Query::Drive(q.clone())).collect(),
                ))
            }
            CliCommand::Sensor { name, queries } => {
                let sensor = self
                    .sensors
                    .iter()
                    .find(|c| c.label() == name)
                    .with_context(|| format!("sensor '{}' not found", name))?;

                Ok((
                    Box::new(SensorCommand::new(sensor)),
                    queries.iter().map(|q| Query::Sensor(q.clone())).collect(),
                ))
            }
            CliCommand::Network { name, queries } => {
                let network = self
                    .networks
                    .get(name)
                    .with_context(|| format!("network `{}` not found", name))?;

                Ok((
                    Box::new(NetworkCommand::new(network)),
                    queries.iter().map(|q| Query::Network(q.clone())).collect(),
                ))
            }
            CliCommand::ListSensors => Ok((Box::new(ListSensorsCommand::new(self)), vec![])),
            CliCommand::ListCpus => Ok((Box::new(ListCpusCommand::new(self)), vec![])),
            CliCommand::ListNetworks => Ok((Box::new(ListNetworksCommand::new(self)), vec![])),
        }
    }

    pub fn refresh_cpus(&mut self) {
        self.sys.refresh_cpu();

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_cpu();
    }

    fn exec_cmd(&mut self, cli: &Cli) -> Result<()> {
        let delimiter = unescape(&cli.delimiter)
            .with_context(|| "invalid delimiter; are there any invalid escape sequences?")?;

        if let Some(fmt) = &cli.fmt {
            println!("{}", self.format_string(&cli.cmd, fmt)?);
        } else {
            let data = cli.cmd.exec()?;

            for (i, d) in data.iter().enumerate() {
                if i < data.len() - 1 {
                    print!("{}{}", d, delimiter)
                } else {
                    println!("{}", d)
                }
            }
        }

        Ok(())
    }

    fn format_string(&mut self, cmd: &CliCommand, fmt: &str) -> Result<String> {
        // Regex for parsing format specifiers %<SPECIFIER>%, or %% which yields just a percent sign.
        let re = Regex::new(r"\%(.*?)\%")?;

        let specs: Vec<String> = re
            .captures_iter(fmt)
            .map(|c| c.extract())
            .map(|(_, [r#match])| r#match.to_string())
            .collect();

        let fmt_ctx = self.create_fmt_ctx(cmd, specs)?;

        Ok(re
            .replace_all(fmt, |caps: &Captures| fmt_ctx.get(&caps[1]).unwrap())
            .to_string())
    }

    fn create_fmt_ctx(&mut self, cli_cmd: &CliCommand, specs: Vec<String>) -> Result<FmtContext> {
        let mut ctx: FmtContext = HashMap::new();

        // Empty specifier (%% in regex input results in empty match) should be replaced as '%'.
        ctx.insert(String::new(), "%".to_string());
        // Remove all empty specifiers from input: we're gonna use specifier names to create command
        // queries from them.
        let specs: Vec<String> = specs.iter().filter(|s| !s.is_empty()).cloned().collect();

        let mut queries: Vec<Query> = vec![];

        for s in &specs {
            queries.push(Query::from_str(cli_cmd, s)?)
        }

        let (mut cmd, _) = self.command_from_cli(cli_cmd)?;

        queries.into_iter().zip(specs).for_each(|(q, s)| {
            ctx.insert(s.to_string(), cmd.exec(q).first().unwrap().to_string());
        });

        Ok(ctx)
    }
}
