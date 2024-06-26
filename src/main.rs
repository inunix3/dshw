// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use anyhow::Result;
use dshw::{
    app::Application,
    cli::{Cli, Parser},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        eprintln!("Warning: this OS is not supported; some stats might be inaccurate/invalid.")
    }

    let app = Application::new();
    app.run(cli)
}
