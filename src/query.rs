// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use crate::cli::CliCommand;

use anyhow::{anyhow, bail, Result};
use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OsQuery {
    /// Time when the system booted since UNIX epoch (seconds).
    BootTime,
    /// A load average within 1 minute expressed as percentage, 2 decimal places.
    /// On Windows, returns nothing.
    #[clap(verbatim_doc_comment)]
    LoadAverage1m,
    /// A load average within 5 minutes expressed as percentage, 2 decimal places.
    /// On Windows, returns nothing.
    #[clap(verbatim_doc_comment)]
    LoadAverage5m,
    /// A load average within 15 minutes expressed as percentage, 2 decimal places.
    /// On Windows, returns nothing.
    #[clap(verbatim_doc_comment)]
    LoadAverage15m,
    /// The name of the OS. Returns nothing if not available.
    Name,
    /// The kernel version. Returns nothing if not available.
    KernelVersion,
    /// The OS version. Returns nothing if not available.
    Version,
    /// The long OS version (e.g. "MacOS 11.2 BigSur"). Returns nothing if not available.
    LongVersion,
    /// The os-release ID.
    ReleaseId,
    /// Host name based off DNS. Returns nothing if not available.
    HostName,
    /// Count of physical cores. If not available, returns nothing. In case there are multiple CPUs,
    /// it will combine the physical core count of all the CPUs.
    #[clap(verbatim_doc_comment)]
    PhysicalCoreCount,
    /// Total CPU usage (percentage, 2 decimal places).
    TotalCpuUsage,
    /// CPU Architecture (e.g. x86, amd64, aarch64, ...). Returns nothing if not available.
    CpuArch,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CpuQuery {
    /// CPU usage (percentage, 2 decimal places).
    Usage,
    /// The frequency of the CPU (the unit is not defined; can be MHz, GHz, etc).
    Frequency,
    /// The brand of the CPU (e.g. "Intel(R) Core(TM) i9-9900K CPU @ 3.60GHz").
    Brand,
    /// ID of CPU's vendor (e.g. "GenuineIntel").
    VendorId,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MemoryQuery {
    /// Total memory usage (bytes).
    Usage,
    /// Total memory capacity (bytes).
    Total,
    /// Reusable memory (bytes). On FreeBSD, it's the same as `free`.
    Available,
    /// Unallocated memory (bytes). On Windows, it's the same as `available`.
    Free,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SwapQuery {
    /// Total swap usage (bytes).
    Usage,
    /// Total swap capacity (bytes).
    Total,
    /// Available swap memory (bytes).
    Available,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DriveQuery {
    /// Total used drive space (bytes).
    Usage,
    /// Drive's filesystem name.
    Fs,
    /// Determine if the drive is removable (boolean, 1 or 0).
    IsRemovable,
    /// The kind of the drive (Should be HDD or SSD, otherwise returns "Unknown").
    Kind,
    /// The path where the drive is mounted.
    MountPoint,
    /// Total space (bytes).
    Total,
    /// Total available space (bytes).
    Available,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SensorQuery {
    /// Sensor's critical temperature (Celsius, 2 decimal places). If
    /// not available, returns nothing
    #[clap(verbatim_doc_comment)]
    CriticalTemp,
    /// Sensor's maximal temperature (Celsius, 2 decimal places).
    MaxTemp,
    /// Current sensor's temperature (Celsius, 2 decimal places).
    Temperature,
}

#[derive(Debug)]
pub enum Query {
    None,
    Os(OsQuery),
    Cpu(CpuQuery),
    Memory(MemoryQuery),
    Swap(SwapQuery),
    Drive(DriveQuery),
    Sensor(SensorQuery),
}

impl Query {
    pub fn from_str(cmd: &CliCommand, s: &str) -> Result<Self> {
        const IGNORE_CASE: bool = true;

        let q = match cmd {
            CliCommand::Os { queries: _ } => Self::Os(
                OsQuery::from_str(s, IGNORE_CASE)
                    .map_err(|_| anyhow!("invalid os query `{}`", s))?,
            ),
            CliCommand::Cpu {
                name: _,
                queries: _,
            } => Self::Cpu(
                CpuQuery::from_str(s, IGNORE_CASE)
                    .map_err(|_| anyhow!("invalid cpu query `{}`", s))?,
            ),
            CliCommand::Memory { queries: _ } => Self::Memory(
                MemoryQuery::from_str(s, IGNORE_CASE)
                    .map_err(|_| anyhow!("invalid memory query `{}`", s))?,
            ),
            CliCommand::Swap { queries: _ } => Self::Swap(
                SwapQuery::from_str(s, IGNORE_CASE)
                    .map_err(|_| anyhow!("invalid swap query `{}`", s))?,
            ),
            CliCommand::Drive {
                name: _,
                queries: _,
            } => Self::Drive(
                DriveQuery::from_str(s, IGNORE_CASE)
                    .map_err(|_| anyhow!("invalid drive query `{}`", s))?,
            ),
            CliCommand::Sensor {
                name: _,
                queries: _,
            } => Self::Sensor(
                SensorQuery::from_str(s, IGNORE_CASE)
                    .map_err(|_| anyhow!("invalid sensor query `{}`", s))?,
            ),
            _ => bail!("this command does not take any arguments (queries)"),
        };

        Ok(q)
    }
}
