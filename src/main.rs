use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use sysinfo::{Component, Components, Cpu, Disk, Disks, System};
use unescaper::unescape;

/// Dead simple CLI program to query information about system and hardware.
/// Basically a CLI wrapper over the sysinfo Rust crate.
#[derive(Parser, Debug)]
#[command(
    author = "inunix3",
    version = "0.1.0",
    long_about = None,
)]
struct Cli {
    #[arg(short, long, default_value = "\n")]
    delimiter: String,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum OsQuery {
    /// Time since the boot in the UNIX epoch format. If you want a readable format, see the `format`
    /// command.
    #[clap(verbatim_doc_comment)]
    BootTime,
    /// A load average within past 1 minute expressed as percents with 2 decimal places.
    LoadAverage1m,
    /// A load average within past 5 minutes expressed as percents with 2 decimal places.
    LoadAverage5m,
    /// A load average within past 15 minutes expressed as percents with 2 decimal places.
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
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum CpuQuery {
    /// CPU usage (percentage with 2 decimal places).
    Usage,
    /// The frequency of the CPU (the unit is not defined; can be MHz, GHz, etc).
    Frequency,
    /// The brand of the CPU.
    Brand,
    /// ID of CPU's vendor.
    VendorId,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum MemoryQuery {
    /// Total memory usage (bytes).
    Usage,
    /// Total memory capacity (bytes).
    Total,
    /// Unallocated memory (bytes).
    Available,
    /// Memory which can be reused (bytes).
    Reusable,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SwapQuery {
    /// Total swap usage (bytes).
    Usage,
    /// Total swap capacity (bytes).
    Total,
    /// Available swap memory (bytes).
    Available,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum DriveQuery {
    /// Total used drive space (bytes).
    Usage,
    /// Drive's filesystem name.
    Fs,
    /// Determine if the drive is removable (boolean).
    IsRemovable,
    /// The kind of the drive (Should be HDD or SSD)
    Kind,
    /// The mount point.
    MountPoint,
    /// Total space (bytes).
    Total,
    /// Total available space (bytes).
    Available,
}

#[derive(Debug, ValueEnum, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
enum SensorQuery {
    /// Sensor's critical temperature (Celsius, floating-point number with 2 decimal places). If
    /// not available, returns nothing
    #[clap(verbatim_doc_comment)]
    CriticalTemp,
    /// Sensor's maximal temperature (Celsius, floating-point number with 2 decimal places).
    MaxTemp,
    /// Current sensor's temperature (Celsius, floating-point number with 2 decimal places).
    Temperature,
}

#[derive(Subcommand, Debug)]
enum Command {
    Os {
        #[clap(required = true)]
        queries: Vec<OsQuery>,
    },
    Cpu {
        #[clap(value_parser)]
        name: String,
        #[clap(required = true)]
        queries: Vec<CpuQuery>,
    },
    Memory {
        #[clap(required = true)]
        queries: Vec<MemoryQuery>,
    },
    Swap {
        #[clap(required = true)]
        queries: Vec<SwapQuery>,
    },
    Drive {
        name: String,
        #[clap(required = true)]
        queries: Vec<DriveQuery>,
    },
    Sensor {
        name: String,
        #[clap(required = true)]
        queries: Vec<SensorQuery>,
    },
    /// List all available sensors.
    ListSensors,
    /// List all available CPUs.
    ListCpus,
    /// Count of physical cores. If not available, prints nothing. In case there are multiple CPUs,
    /// it will combine the physical core count of all the CPUs.
    #[clap(verbatim_doc_comment)]
    PhysicalCoreCount,
    /// Total CPU usage (percentage with 2 decimal places).
    TotalCpuUsage,
}

fn handle_os(queries: Vec<OsQuery>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for q in queries {
        let line = match q {
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
        };

        output.push(line)
    }

    output
}

fn handle_cpu(cpu: &Cpu, queries: Vec<CpuQuery>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for q in queries {
        let line = match q {
            CpuQuery::Usage => format!("{:.2}", cpu.cpu_usage()),
            CpuQuery::Frequency => cpu.frequency().to_string(),
            CpuQuery::Brand => cpu.brand().to_string(),
            CpuQuery::VendorId => cpu.vendor_id().to_string(),
        };

        output.push(line)
    }

    output
}

fn handle_memory(sys: System, queries: Vec<MemoryQuery>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for q in queries {
        let line = match q {
            MemoryQuery::Usage => sys.used_memory(),
            MemoryQuery::Total => sys.total_memory(),
            MemoryQuery::Available => sys.available_memory(),
            MemoryQuery::Reusable => sys.free_memory(),
        }
        .to_string();

        output.push(line)
    }

    output
}

fn handle_swap(sys: System, queries: Vec<SwapQuery>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for q in queries {
        let line = match q {
            SwapQuery::Usage => sys.used_swap(),
            SwapQuery::Total => sys.total_swap(),
            SwapQuery::Available => sys.free_swap(),
        }
        .to_string();

        output.push(line)
    }

    output
}

fn handle_drive(disk: &Disk, queries: Vec<DriveQuery>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    let total_space = disk.total_space();
    let avail_space = disk.available_space();
    let used_space = total_space - avail_space;

    for q in queries {
        let line = match q {
            DriveQuery::Usage => used_space.to_string(),
            DriveQuery::Fs => format!("{}", disk.file_system().to_string_lossy()),
            DriveQuery::IsRemovable => format!("{}", disk.is_removable() as i32),
            DriveQuery::Kind => disk.kind().to_string(),
            DriveQuery::MountPoint => format!("{}", disk.mount_point().to_string_lossy()),
            DriveQuery::Total => total_space.to_string(),
            DriveQuery::Available => avail_space.to_string(),
        };

        output.push(line)
    }

    output
}

fn handle_sensor(sensor: &Component, queries: Vec<SensorQuery>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for q in queries {
        // If the critical temperature cannot be queried, an empty string will be printed.
        let critical_temp = sensor
            .critical()
            .map(|t| format!("{:.2}", t))
            .unwrap_or_default();

        let line = match q {
            SensorQuery::CriticalTemp => critical_temp,
            SensorQuery::MaxTemp => format!("{:.2}", sensor.max()),
            SensorQuery::Temperature => format!("{:.2}", sensor.temperature()),
        };

        output.push(line);
    }

    output
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let delimiter = unescape(&cli.delimiter)
        .with_context(|| "invalid delimiter (are there invalid escape sequences?)")?;

    if !sysinfo::IS_SUPPORTED_SYSTEM {
        eprintln!("Warning: this OS is not supported; some stats might be inaccurate/invalid.")
    }

    let mut sys = System::new();

    let lines = match cli.cmd {
        Command::Os { queries } => handle_os(queries),
        Command::Cpu { name, queries } => {
            sys.refresh_cpu();

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            sys.refresh_cpu();

            let cpus = sys.cpus();

            let cpu = cpus
                .iter()
                .find(|c| c.name() == name.as_str())
                .with_context(|| format!("cpu '{}' not found", name))?;

            handle_cpu(cpu, queries)
        }
        Command::Memory { queries } => {
            sys.refresh_memory();

            handle_memory(sys, queries)
        }
        Command::Swap { queries } => {
            sys.refresh_memory();

            handle_swap(sys, queries)
        }
        Command::Drive { name, queries } => {
            let disks = Disks::new_with_refreshed_list();

            let disk = disks
                .list()
                .iter()
                .find(|d| d.name() == name.as_str())
                .with_context(|| format!("disk '{}' not found", name))?;

            handle_drive(disk, queries)
        }
        Command::Sensor { name, queries } => {
            let components = Components::new_with_refreshed_list();

            let sensor = components
                .iter()
                .find(|c| c.label() == name)
                .with_context(|| format!("sensor '{}' not found", name))?;

            handle_sensor(sensor, queries)
        }
        Command::ListSensors => Components::new_with_refreshed_list()
            .iter()
            .map(|c| c.label().to_string())
            .collect(),
        Command::ListCpus => {
            sys.refresh_cpu();

            sys.cpus().iter().map(|c| c.name().to_string()).collect()
        }
        // If the physical core count cannot be queried, an empty string is printed.
        Command::PhysicalCoreCount => {
            let count = sys
                .physical_core_count()
                .map(|c| c.to_string())
                .unwrap_or_default();

            vec![count.to_string()]
        }
        Command::TotalCpuUsage => {
            sys.refresh_cpu();

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            sys.refresh_cpu();

            vec![format!("{:.2}", sys.global_cpu_info().cpu_usage())]
        }
    };

    for (i, l) in lines.iter().enumerate() {
        if i < lines.len() - 1 {
            print!("{}{}", l, delimiter)
        } else {
            println!("{}", l)
        }
    }

    Ok(())
}
