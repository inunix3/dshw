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

    let mut output: Vec<String> = vec![];
    cli.cmd.exec(&mut output)?;

    for (i, l) in output.iter().enumerate() {
        if i < output.len() - 1 {
            print!("{}{}", l, delimiter)
        } else {
            println!("{}", l)
        }
    }

    Ok(())
}
