// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use anyhow::{Context, Result};
use clap::Parser;
use dshw::Cli;
use unescaper::unescape;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let delimiter = unescape(&cli.delimiter)
        .with_context(|| "invalid delimiter; are there any invalid escape sequences?")?;

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        eprintln!("Warning: this OS is not supported; some stats might be inaccurate/invalid.")
    }

    if let Some(fmt) = cli.fmt {
        println!("{}", dshw::format_string(cli.cmd, fmt)?);
    } else {
        let mut data: Vec<String> = vec![];
        cli.cmd.exec(&mut data)?;

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
