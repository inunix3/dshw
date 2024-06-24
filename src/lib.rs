// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use regex::{Captures, Regex};
use serde::Serialize;
use sysinfo::{Component, Components, Cpu, Disk, Disks, System};

/// Dead simple CLI program to query information about system and hardware.
/// Basically a CLI wrapper over the sysinfo Rust crate.
#[derive(Parser, Debug)]
#[command(
    author = "inunix3",
    version = "0.1.0",
    long_about = None,
)]
pub struct Cli {
    #[arg(short, long, default_value = "\n")]
    pub delimiter: String,
    #[command(subcommand)]
    pub cmd: Command,
    /// String with format specifiers which will be replaced by actual values. Syntax for format
    /// specifiers is `%<SPECIFIER>%`. To output the literal percent sign, write `%%`. If the specifier
    /// does not exist, a corresponding error is reported. Any supplied queries to the commands are
    /// ignored.
    #[arg(short, long, verbatim_doc_comment)]
    pub fmt: Option<String>,
}

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

impl OsQuery {
    fn exec(self, sys: &mut System) -> String {
        match self {
            OsQuery::BootTime => System::boot_time().to_string(),
            OsQuery::LoadAverage1m => format!("{:.2}", System::load_average().one),
            OsQuery::LoadAverage5m => format!("{:.2}", System::load_average().five),
            OsQuery::LoadAverage15m => format!("{:.2}", System::load_average().fifteen),
            OsQuery::Name => System::name().unwrap_or_default(),
            OsQuery::KernelVersion => System::kernel_version().unwrap_or_default(),
            OsQuery::Version => System::os_version().unwrap_or_default(),
            OsQuery::LongVersion => System::long_os_version().unwrap_or_default(),
            OsQuery::ReleaseId => System::distribution_id(),
            OsQuery::HostName => System::host_name().unwrap_or_default(),
            OsQuery::PhysicalCoreCount => {
                let count = sys
                    .physical_core_count()
                    .map(|c| c.to_string())
                    .unwrap_or_default();

                count.to_string()
            }
            OsQuery::TotalCpuUsage => {
                sys.refresh_cpu();

                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                sys.refresh_cpu();

                format!("{:.2}", sys.global_cpu_info().cpu_usage())
            }
            OsQuery::CpuArch => System::cpu_arch().unwrap_or_default(),
        }
    }
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

impl CpuQuery {
    fn exec(self, cpu: &Cpu) -> String {
        match self {
            CpuQuery::Usage => format!("{:.2}", cpu.cpu_usage()),
            CpuQuery::Frequency => cpu.frequency().to_string(),
            CpuQuery::Brand => cpu.brand().to_string(),
            CpuQuery::VendorId => cpu.vendor_id().to_string(),
        }
    }
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

impl MemoryQuery {
    fn exec(self, sys: &System) -> String {
        match self {
            MemoryQuery::Usage => sys.used_memory(),
            MemoryQuery::Total => sys.total_memory(),
            MemoryQuery::Available => sys.available_memory(),
            MemoryQuery::Free => sys.free_memory(),
        }
        .to_string()
    }
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

impl SwapQuery {
    fn exec(self, sys: &System) -> String {
        match self {
            SwapQuery::Usage => sys.used_swap(),
            SwapQuery::Total => sys.total_swap(),
            SwapQuery::Available => sys.free_swap(),
        }
        .to_string()
    }
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

impl DriveQuery {
    fn exec(self, disk: &Disk) -> String {
        let total_space = disk.total_space();
        let avail_space = disk.available_space();
        let used_space = total_space - avail_space;

        match self {
            DriveQuery::Usage => used_space.to_string(),
            DriveQuery::Fs => format!("{}", disk.file_system().to_string_lossy()),
            DriveQuery::IsRemovable => format!("{}", disk.is_removable() as i32),
            DriveQuery::Kind => disk.kind().to_string(),
            DriveQuery::MountPoint => format!("{}", disk.mount_point().to_string_lossy()),
            DriveQuery::Total => total_space.to_string(),
            DriveQuery::Available => avail_space.to_string(),
        }
    }
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

impl SensorQuery {
    fn exec(self, sensor: &Component) -> String {
        match self {
            SensorQuery::CriticalTemp => sensor
                .critical()
                .map(|t| format!("{:.2}", t))
                .unwrap_or_default(),
            SensorQuery::MaxTemp => format!("{:.2}", sensor.max()),
            SensorQuery::Temperature => format!("{:.2}", sensor.temperature()),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
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
}

impl Command {
    pub fn exec(self, output: &mut Vec<String>) -> Result<()> {
        let mut sys = System::new();

        match self {
            Command::Os { queries } => {
                for q in queries {
                    output.push(q.exec(&mut sys))
                }
            }
            Command::Cpu { name, queries } => {
                sys.refresh_cpu();

                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                sys.refresh_cpu();

                let cpus = sys.cpus();

                let cpu = cpus
                    .iter()
                    .find(|c| c.name() == name.as_str())
                    .with_context(|| format!("cpu '{}' not found", name))?;

                for q in queries {
                    output.push(q.exec(cpu))
                }
            }
            Command::Memory { queries } => {
                sys.refresh_memory();

                for q in queries {
                    output.push(q.exec(&sys))
                }
            }
            Command::Swap { queries } => {
                sys.refresh_memory();

                for q in queries {
                    output.push(q.exec(&sys))
                }
            }
            Command::Drive { name, queries } => {
                let disks = Disks::new_with_refreshed_list();

                let disk = disks
                    .list()
                    .iter()
                    .find(|d| d.name() == name.as_str())
                    .with_context(|| format!("disk '{}' not found", name))?;

                for q in queries {
                    output.push(q.exec(disk))
                }
            }
            Command::Sensor { name, queries } => {
                let components = Components::new_with_refreshed_list();

                let sensor = components
                    .iter()
                    .find(|c| c.label() == name)
                    .with_context(|| format!("sensor '{}' not found", name))?;

                for q in queries {
                    output.push(q.exec(sensor))
                }
            }
            Command::ListSensors => {
                let components = Components::new_with_refreshed_list();

                for c in &components {
                    output.push(c.label().to_string())
                }
            }
            Command::ListCpus => {
                sys.refresh_cpu();

                for c in sys.cpus() {
                    output.push(c.name().to_string())
                }
            }
        }

        Ok(())
    }
}

fn create_fmt_ctx(cmd: Command, specs: Vec<String>) -> Result<HashMap<String, String>> {
    let mut ctx: HashMap<String, String> = HashMap::new();

    // Empty specifier (%% in regex input results in empty match) should be replaced as '%'.
    ctx.insert(String::new(), "%".to_string());
    // Remove all empty specifiers from input: we're gonna use specifier names to create command
    // queries from them.
    let specs: Vec<String> = specs.iter().filter(|s| !s.is_empty()).cloned().collect();

    let mut sys = System::new();

    match cmd {
        Command::Os { .. } => {
            for s in specs {
                ctx.insert(
                    s.clone(),
                    OsQuery::from_str(&s, true)
                        .map_err(|_| anyhow!("unknown specifier `{}`", &s))?
                        .exec(&mut sys),
                );
            }
        }
        Command::Cpu { name, queries: _ } => {
            sys.refresh_cpu();

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            sys.refresh_cpu();

            let cpus = sys.cpus();

            let cpu = cpus
                .iter()
                .find(|c| c.name() == name.as_str())
                .with_context(|| format!("cpu '{}' not found", name))?;

            for s in specs {
                ctx.insert(
                    s.clone(),
                    CpuQuery::from_str(&s, true)
                        .map_err(|_| anyhow!("unknown specifier `{}`", &s))?
                        .exec(cpu),
                );
            }
        }
        Command::Memory { queries: _ } => {
            sys.refresh_memory();

            for s in specs {
                ctx.insert(
                    s.clone(),
                    MemoryQuery::from_str(&s, true)
                        .map_err(|_| anyhow!("unknown specifier `{}`", &s))?
                        .exec(&sys),
                );
            }
        }
        Command::Swap { queries: _ } => {
            sys.refresh_memory();

            for s in specs {
                ctx.insert(
                    s.clone(),
                    SwapQuery::from_str(&s, true)
                        .map_err(|_| anyhow!("unknown specifier `{}`", &s))?
                        .exec(&sys),
                );
            }
        }
        Command::Drive { name, queries: _ } => {
            let disks = Disks::new_with_refreshed_list();

            let disk = disks
                .list()
                .iter()
                .find(|d| d.name() == name.as_str())
                .with_context(|| format!("disk '{}' not found", name))?;

            for s in specs {
                ctx.insert(
                    s.clone(),
                    DriveQuery::from_str(&s, true)
                        .map_err(|_| anyhow!("unknown specifier `{}`", &s))?
                        .exec(disk),
                );
            }
        }
        Command::Sensor { name, queries: _ } => {
            let components = Components::new_with_refreshed_list();

            let sensor = components
                .iter()
                .find(|c| c.label() == name)
                .with_context(|| format!("sensor '{}' not found", name))?;

            for s in specs {
                ctx.insert(
                    s.clone(),
                    SensorQuery::from_str(&s, true)
                        .map_err(|_| anyhow!("unknown specifier `{}`", &s))?
                        .exec(sensor),
                );
            }
        }
        _ => bail!("formatting is not supported for this command"),
    }

    Ok(ctx)
}

pub fn format_string(cmd: Command, fmt: String) -> Result<String> {
    // Regex for parsing format specifiers %<SPECIFIER>%, or %% which will later yield just a percent sign.
    let re = Regex::new(r"\%(.*?)\%")?;

    let specs: Vec<String> = re
        .captures_iter(&fmt)
        .map(|c| c.extract())
        .map(|(_, [m])| m.to_string())
        .collect();

    let fmt_ctx = create_fmt_ctx(cmd, specs)?;

    Ok(re
        .replace_all(&fmt, |caps: &Captures| fmt_ctx.get(&caps[1]).unwrap())
        .to_string())
}
