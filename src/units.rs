// Copyright (c) 2024 inunix3
//
// This file is licensed under the MIT License (see LICENSE.md).

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, ValueEnum, Clone, Copy, Serialize, strum_macros::Display)]
#[serde(rename_all = "lowercase")]
pub enum DataUnit {
    #[strum(serialize = "bits")]
    Bits,
    #[strum(serialize = "bytes")]
    Bytes,
    #[strum(serialize = "kb")]
    Kb,
    #[strum(serialize = "kib")]
    Kib,
    #[strum(serialize = "mb")]
    Mb,
    #[strum(serialize = "mib")]
    Mib,
    #[strum(serialize = "gb")]
    Gb,
    #[strum(serialize = "gib")]
    Gib,
    #[strum(serialize = "tb")]
    Tb,
    #[strum(serialize = "tib")]
    Tib,
}

pub struct DataValue {
    value: f64,
    unit: DataUnit,
}

impl DataValue {
    pub fn new(value: f64, unit: DataUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_bytes(value: f64, unit: DataUnit) -> Self {
        let factor = match unit {
            DataUnit::Bits => 1.0 / 8.0,
            DataUnit::Bytes => 1.0,
            DataUnit::Kb => 1000.0,
            DataUnit::Kib => 1024.0,
            DataUnit::Mb => 1_000_000.0,
            DataUnit::Mib => 1024.0 * 1024.0,
            DataUnit::Gb => 1_000_000_000.0,
            DataUnit::Gib => 1024.0 * 1024.0 * 1024.0,
            DataUnit::Tb => 1_000_000_000_000.0,
            DataUnit::Tib => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        };

        Self {
            value: value / factor,
            unit,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn value_str(&self) -> String {
        match self.unit {
            DataUnit::Bits | DataUnit::Bytes => self.value.to_string(),
            _ => format!("{:.2}", self.value),
        }
    }

    pub fn unit(&self) -> DataUnit {
        self.unit
    }
}
