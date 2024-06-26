// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use crate::{app::Application, query::*};

use sysinfo::{Component, Cpu, Disk, NetworkData, System};

pub trait Command {
    fn exec(&mut self, q: Query) -> Vec<String>;
}

pub struct OsCommand<'a> {
    app: &'a mut Application,
}

impl Command for OsCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let s = if let Query::Os(q) = q {
            match q {
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
                    let count = self
                        .app
                        .sys
                        .physical_core_count()
                        .map(|c| c.to_string())
                        .unwrap_or_default();

                    count.to_string()
                }
                OsQuery::TotalCpuUsage => {
                    self.app.sys.refresh_cpu();

                    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                    self.app.sys.refresh_cpu();

                    format!("{:.2}", self.app.sys.global_cpu_info().cpu_usage())
                }
                OsQuery::CpuArch => System::cpu_arch().unwrap_or_default(),
            }
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> OsCommand<'a> {
    pub fn new(app: &'a mut Application) -> Self {
        Self { app }
    }
}

pub struct CpuCommand<'a> {
    cpu: &'a Cpu,
}

impl Command for CpuCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let s = if let Query::Cpu(q) = q {
            match q {
                CpuQuery::Usage => format!("{:.2}", self.cpu.cpu_usage()),
                CpuQuery::Frequency => self.cpu.frequency().to_string(),
                CpuQuery::Brand => self.cpu.brand().to_string(),
                CpuQuery::VendorId => self.cpu.vendor_id().to_string(),
            }
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> CpuCommand<'a> {
    pub fn new(cpu: &'a Cpu) -> Self {
        Self { cpu }
    }
}

pub struct MemoryCommand<'a> {
    app: &'a mut Application,
}

impl Command for MemoryCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let s = if let Query::Memory(q) = q {
            match q {
                MemoryQuery::Usage => self.app.sys.used_memory(),
                MemoryQuery::Total => self.app.sys.total_memory(),
                MemoryQuery::Available => self.app.sys.available_memory(),
                MemoryQuery::Free => self.app.sys.free_memory(),
            }
            .to_string()
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> MemoryCommand<'a> {
    pub fn new(app: &'a mut Application) -> Self {
        Self { app }
    }
}

pub struct SwapCommand<'a> {
    app: &'a mut Application,
}

impl Command for SwapCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let s = if let Query::Swap(q) = q {
            match q {
                SwapQuery::Usage => self.app.sys.used_swap(),
                SwapQuery::Total => self.app.sys.total_swap(),
                SwapQuery::Available => self.app.sys.free_swap(),
            }
            .to_string()
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> SwapCommand<'a> {
    pub fn new(app: &'a mut Application) -> Self {
        Self { app }
    }
}

pub struct DriveCommand<'a> {
    drive: &'a Disk,
}

impl Command for DriveCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let total_space = self.drive.total_space();
        let avail_space = self.drive.available_space();
        let used_space = total_space - avail_space;

        let s = if let Query::Drive(q) = q {
            match q {
                DriveQuery::Usage => used_space.to_string(),
                DriveQuery::Fs => self.drive.file_system().to_string_lossy().to_string(),
                DriveQuery::IsRemovable => (self.drive.is_removable() as i32).to_string(),
                DriveQuery::Kind => self.drive.kind().to_string(),
                DriveQuery::MountPoint => self.drive.mount_point().to_string_lossy().to_string(),
                DriveQuery::Total => total_space.to_string(),
                DriveQuery::Available => avail_space.to_string(),
            }
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> DriveCommand<'a> {
    pub fn new(drive: &'a Disk) -> Self {
        Self { drive }
    }
}

pub struct SensorCommand<'a> {
    sensor: &'a Component,
}

impl Command for SensorCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let s = if let Query::Sensor(q) = q {
            match q {
                SensorQuery::CriticalTemp => self
                    .sensor
                    .critical()
                    .map(|t| format!("{:.2}", t))
                    .unwrap_or_default(),
                SensorQuery::MaxTemp => format!("{:.2}", self.sensor.max()),
                SensorQuery::Temperature => format!("{:.2}", self.sensor.temperature()),
            }
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> SensorCommand<'a> {
    pub fn new(sensor: &'a Component) -> Self {
        Self { sensor }
    }
}

pub struct NetworkCommand<'a> {
    network: &'a NetworkData,
}

impl Command for NetworkCommand<'_> {
    fn exec(&mut self, q: Query) -> Vec<String> {
        if let Query::None = q {
            return vec![];
        };

        let s = if let Query::Network(q) = q {
            match q {
                NetworkQuery::MacAddress => self.network.mac_address().to_string(),
                NetworkQuery::TotalIncomingErrors => {
                    self.network.total_errors_on_received().to_string()
                }
                NetworkQuery::TotalOutcomingErrors => {
                    self.network.total_errors_on_transmitted().to_string()
                }
                NetworkQuery::TotalReceivedBytes => self.network.total_received().to_string(),
                NetworkQuery::TotalTransmittedBytes => self.network.total_transmitted().to_string(),
                NetworkQuery::TotalReceivedPackets => {
                    self.network.total_packets_received().to_string()
                }
                NetworkQuery::TotalTransmittedPackets => {
                    self.network.total_packets_transmitted().to_string()
                }
            }
        } else {
            unreachable!()
        };

        vec![s]
    }
}

impl<'a> NetworkCommand<'a> {
    pub fn new(network: &'a NetworkData) -> Self {
        Self { network }
    }
}

pub struct ListCpusCommand<'a> {
    app: &'a mut Application,
}

impl Command for ListCpusCommand<'_> {
    fn exec(&mut self, _q: Query) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        self.app.sys.refresh_cpu();

        for c in self.app.sys.cpus() {
            output.push(c.name().to_string())
        }

        output
    }
}

impl<'a> ListCpusCommand<'a> {
    pub fn new(app: &'a mut Application) -> Self {
        Self { app }
    }
}

pub struct ListSensorsCommand<'a> {
    app: &'a mut Application,
}

impl Command for ListSensorsCommand<'_> {
    fn exec(&mut self, _q: Query) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        for c in &*self.app.sensors {
            output.push(c.label().to_string())
        }

        output
    }
}

impl<'a> ListSensorsCommand<'a> {
    pub fn new(app: &'a mut Application) -> Self {
        Self { app }
    }
}

pub struct ListNetworksCommand<'a> {
    app: &'a mut Application,
}

impl Command for ListNetworksCommand<'_> {
    fn exec(&mut self, _q: Query) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        for (interface_name, _) in &*self.app.networks {
            output.push(interface_name.to_string())
        }

        output
    }
}

impl<'a> ListNetworksCommand<'a> {
    pub fn new(app: &'a mut Application) -> Self {
        Self { app }
    }
}
