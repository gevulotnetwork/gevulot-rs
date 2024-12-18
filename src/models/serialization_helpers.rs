//! Serialization helpers for handling byte sizes, CPU/GPU cores, and time durations.
//! 
//! This module provides types and traits to handle parsing and serialization of:
//! - Byte sizes (e.g. "500MB", "1.5GB")
//! - CPU/GPU core counts (e.g. "2 cores", "500mcpu") 
//! - Time durations (e.g. "24h", "7d")

use std::str::FromStr;

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};

/// Trait for specifying default multiplication factors for byte units
pub trait DefaultFactor {
    const FACTOR: i64;
}

/// Default factor of 1 (no multiplication)
#[derive(Debug)]
pub struct DefaultFactorOne;
impl DefaultFactor for DefaultFactorOne {
    const FACTOR: i64 = 1;
}

/// Default factor of 1KB (1024 bytes)
#[derive(Debug)]
pub struct DefaultFactorOneKilobyte;
impl DefaultFactor for DefaultFactorOneKilobyte {
    const FACTOR: i64 = 1024;
}

/// Default factor of 1MB (1024 * 1024 bytes)
#[derive(Debug)]
pub struct DefaultFactorOneMegabyte;
impl DefaultFactor for DefaultFactorOneMegabyte {
    const FACTOR: i64 = 1024 * 1024;
}

/// Default factor of 1GB (1024 * 1024 * 1024 bytes)
#[derive(Debug)]
pub struct DefaultFactorOneGigabyte;
impl DefaultFactor for DefaultFactorOneGigabyte {
    const FACTOR: i64 = 1024 * 1024 * 1024;
}

/// Type for handling byte sizes with configurable default factors
/// 
/// Supports three formats:
/// - Raw numbers (e.g. 1024)
/// - String representations (e.g. "500MB", "1.5GB")
/// - Default factor based on type parameter
///
/// # Examples
///
/// ```rust
/// use crate::models::serialization_helpers::{ByteUnit, DefaultFactorOne};
/// 
/// // Parse from string
/// let bytes: ByteUnit = "500MB".parse().unwrap();
/// assert_eq!(bytes.bytes().unwrap(), 500 * 1024 * 1024);
///
/// // Use raw number
/// let bytes = ByteUnit::<DefaultFactorOneMegabyte>::from(1);
/// assert_eq!(bytes.bytes().unwrap(), 1 * 1024 * 1024);
/// ```
#[derive(Debug, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum ByteUnit<D: DefaultFactor = DefaultFactorOne>{
    Number(i64),
    String(String),
    #[serde(skip)]
    Factor(std::marker::PhantomData<D>),
}

impl<D: DefaultFactor> ByteUnit<D> {
    /// Convert to number of bytes
    pub fn bytes(&self) -> Result<i64, String> {
        match self {
            ByteUnit::Number(n) => Ok(*n * D::FACTOR),
            ByteUnit::String(s) => {
                if s.chars().all(|c| c.is_ascii_digit()) {
                    return Ok(s.parse::<i64>().map_err(|e| e.to_string())? * D::FACTOR);
                }
                s.parse::<ByteSize>().map(|b| b.0 as i64)
            },
            ByteUnit::Factor(_) => Ok(D::FACTOR),
        }
    }
}

impl<D: DefaultFactor> PartialEq for ByteUnit<D> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}

impl<D: DefaultFactor> FromStr for ByteUnit<D> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = s.parse::<ByteSize>()?;
        Ok(ByteUnit::String(s.to_string()))
    }
}

impl<D: DefaultFactor> From<i64> for ByteUnit<D> {
    fn from(n: i64) -> Self {
        ByteUnit::Number(n)
    }
}

/// Type for handling CPU/GPU core counts
///
/// Supports formats like:
/// - Raw numbers (interpreted as cores, e.g. 2 = 2000 millicores)
/// - String representations with units:
///   - Cores: "2 cores", "2 cpus", "2 gpus"
///   - Millicores: "500mcpu", "500mgpu", "500mcore"
///
/// # Examples
///
/// ```rust
/// use crate::models::serialization_helpers::CoreUnit;
///
/// // Parse core counts
/// let cores: CoreUnit = "2 cores".parse().unwrap();
/// assert_eq!(cores.millicores().unwrap(), 2000);
///
/// // Parse millicores
/// let cores: CoreUnit = "500mcpu".parse().unwrap();
/// assert_eq!(cores.millicores().unwrap(), 500);
/// ```
#[derive(Debug, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum CoreUnit {
    Number(i64),
    String(String),
}

impl CoreUnit {
    /// Convert to millicores (1 core = 1000 millicores)
    pub fn millicores(&self) -> Result<i64, String> {
        match self {
            CoreUnit::Number(n) => Ok(*n * 1000), // Default factor without unit is 1000
            CoreUnit::String(s) => {
                // Extract numeric part
                let numeric: String = s.chars().take_while(|c| c.is_digit(10)).collect();
                // Extract and normalize unit part
                let unit = s[numeric.len()..].to_lowercase().replace(" ", "");
                let base: i64 = numeric.parse().map_err(|e| format!("Invalid number: {}", e))?;
                
                // Convert based on unit, using 1000 millicores = 1 core
                Ok(base
                    * match unit.as_str() {
                        "cpu" | "cpus" => 1000, 
                        "gpu" | "gpus" => 1000,
                        "core" | "cores" => 1000,
                        "mcpu" | "mcpus" | "millicpu" | "millicpus" => 1,
                        "mgpu" | "mgpus" | "milligpu" | "milligpus" => 1,
                        "mcore" | "mcores" | "millicore" | "millicores" => 1,
                        "" => 1000, // Default to cores if no unit specified
                        _ => return Err(format!("Invalid unit: {}", unit)),
                    })
            }
        }
    }
}

impl PartialEq for CoreUnit {
    fn eq(&self, other: &Self) -> bool {
        self.millicores() == other.millicores()
    }
}

impl FromStr for CoreUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = CoreUnit::String(s.to_string());
        res.millicores().map_err(|e| format!("Invalid core size: {}", e))?;
        Ok(res)
    }
}

impl From<i64> for CoreUnit {
    fn from(n: i64) -> Self {
        CoreUnit::Number(n)
    }
}

/// Type for handling time durations
///
/// Supports:
/// - Raw numbers (interpreted as seconds)
/// - Human readable durations (e.g. "24h", "7d", "1y")
///
/// # Examples
///
/// ```rust
/// use crate::models::serialization_helpers::TimeUnit;
///
/// // Parse durations
/// let time: TimeUnit = "24h".parse().unwrap();
/// assert_eq!(time.seconds().unwrap(), 24 * 60 * 60);
///
/// // Use raw seconds
/// let time = TimeUnit::from(3600);
/// assert_eq!(time.seconds().unwrap(), 3600);
/// ```
#[derive(Debug, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum TimeUnit {
    Number(i64),
    String(String),
}

impl TimeUnit {
    /// Convert to seconds
    pub fn seconds(&self) -> Result<i64, String> {
        match self {
            TimeUnit::Number(n) => Ok(*n),
            TimeUnit::String(s) => {
                let duration = humantime::parse_duration(s)
                    .map_err(|e| format!("Invalid duration: {}", e))?;
                Ok(duration.as_secs() as i64)
            },
        }
    }
}

impl PartialEq for TimeUnit {
    fn eq(&self, other: &Self) -> bool {
        self.seconds() == other.seconds()
    }
}

impl FromStr for TimeUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = s.parse::<humantime::Duration>().map_err(|e| format!("Invalid duration: {}", e))?;
        Ok(TimeUnit::String(s.to_string()))
    }
}

impl From<i64> for TimeUnit {
    fn from(n: i64) -> Self {
        TimeUnit::Number(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_byte_unit() {
        let bytes: ByteUnit = "500MiB".parse().unwrap();
        assert_eq!(bytes.bytes().unwrap(), 500 * 1024 * 1024);
    }

    #[test]
    fn test_byte_unit_default_factor() {
        let bytes: ByteUnit = 1.into();
        assert_eq!(bytes.bytes(), Ok(1), "Default factor should be 1 (no multiplication)");
        let bytes: ByteUnit<DefaultFactorOneKilobyte> = 1.into();
        assert_eq!(bytes.bytes(), Ok(1 * 1024), "1 with KB factor should be 1024 bytes");
        let bytes: ByteUnit<DefaultFactorOneMegabyte> = 1.into();
        assert_eq!(bytes.bytes(), Ok(1 * 1024 * 1024), "1 with MB factor should be 1024*1024 bytes");
        let bytes: ByteUnit<DefaultFactorOneGigabyte> = 1.into();
        assert_eq!(bytes.bytes(), Ok(1 * 1024 * 1024 * 1024), "1 with GB factor should be 1024*1024*1024 bytes");

        let bytes: ByteUnit = "1".parse().unwrap();
        assert_eq!(bytes.bytes(), Ok(1), "String '1' with default factor should be 1 byte");
        let bytes: ByteUnit<DefaultFactorOneKilobyte> = "1".parse().unwrap();
        assert_eq!(bytes.bytes(), Ok(1 * 1024), "String '1' with KB factor should be 1024 bytes");
        let bytes: ByteUnit<DefaultFactorOneMegabyte> = "1".parse().unwrap();
        assert_eq!(bytes.bytes(), Ok(1 * 1024 * 1024), "String '1' with MB factor should be 1024*1024 bytes");
        let bytes: ByteUnit<DefaultFactorOneGigabyte> = "1".parse().unwrap();
        assert_eq!(bytes.bytes(), Ok(1 * 1024 * 1024 * 1024), "String '1' with GB factor should be 1024*1024*1024 bytes");

        let bytes: ByteUnit<DefaultFactorOneKilobyte> = "1MiB".parse().unwrap();
        assert_eq!(bytes.bytes(), Ok(1 * 1024 * 1024), "Explicit unit (1MiB) should override default factor (KB)");
    }

    #[test]
    fn test_time_unit() {
        let time: TimeUnit = "24h".parse().unwrap();
        assert_eq!(time.seconds().unwrap(), 24 * 60 * 60);

        let time: TimeUnit = "7d".parse().unwrap();
        assert_eq!(time.seconds().unwrap(), 7 * 24 * 60 * 60);

        let time: TimeUnit = "30m".parse().unwrap();
        assert_eq!(time.seconds().unwrap(), 30 * 60);

        let time: TimeUnit = "90s".parse().unwrap();
        assert_eq!(time.seconds().unwrap(), 90);

        let time: TimeUnit = 3600.into();
        assert_eq!(time.seconds().unwrap(), 3600);
    }

    #[test]
    fn test_core_unit() {
        let cores: CoreUnit = "2 cores".parse().unwrap();
        assert_eq!(cores.millicores().unwrap(), 2000);

        let cores: CoreUnit = "500mcpu".parse().unwrap();
        assert_eq!(cores.millicores().unwrap(), 500);

        let cores: CoreUnit = "1.5 cpus".parse().unwrap();
        assert_eq!(cores.millicores().unwrap(), 1500);

        let cores: CoreUnit = "2 gpus".parse().unwrap();
        assert_eq!(cores.millicores().unwrap(), 2000);

        let cores: CoreUnit = "750mgpu".parse().unwrap();
        assert_eq!(cores.millicores().unwrap(), 750);

        let cores: CoreUnit = 2.into();
        assert_eq!(cores.millicores().unwrap(), 2000);
    }

    #[test]
    fn test_invalid_formats() {
        assert!("invalid".parse::<ByteUnit>().is_err());
        assert!("invalid".parse::<TimeUnit>().is_err());
        assert!("invalid".parse::<CoreUnit>().is_err());

        assert!("-1GB".parse::<ByteUnit>().is_err());
        assert!("-24h".parse::<TimeUnit>().is_err());
        assert!("-2cores".parse::<CoreUnit>().is_err());
    }

    #[test]
    fn test_json_serialization() {
        // Test ByteUnit serialization
        let bytes: ByteUnit = "500MiB".parse().unwrap();
        let json = serde_json::to_string(&bytes).unwrap();
        assert_eq!(json, "\"500MiB\"");

        let bytes: ByteUnit = 1024.into();
        let json = serde_json::to_string(&bytes).unwrap();
        assert_eq!(json, "1024");

        // Test TimeUnit serialization
        let time: TimeUnit = "24h".parse().unwrap();
        let json = serde_json::to_string(&time).unwrap();
        assert_eq!(json, "\"24h\"");

        let time: TimeUnit = 3600.into();
        let json = serde_json::to_string(&time).unwrap();
        assert_eq!(json, "3600");

        // Test CoreUnit serialization
        let cores: CoreUnit = "2 cores".parse().unwrap();
        let json = serde_json::to_string(&cores).unwrap();
        assert_eq!(json, "\"2 cores\"");

        let cores: CoreUnit = 2.into();
        let json = serde_json::to_string(&cores).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_json_deserialization() {
        // Test ByteUnit deserialization
        let bytes: ByteUnit = serde_json::from_str("\"500MiB\"").unwrap();
        assert_eq!(bytes.bytes().unwrap(), 500 * 1024 * 1024);

        let bytes: ByteUnit = serde_json::from_str("1024").unwrap();
        assert_eq!(bytes.bytes().unwrap(), 1024);

        // Test TimeUnit deserialization
        let time: TimeUnit = serde_json::from_str("\"24h\"").unwrap();
        assert_eq!(time.seconds().unwrap(), 24 * 60 * 60);

        let time: TimeUnit = serde_json::from_str("3600").unwrap();
        assert_eq!(time.seconds().unwrap(), 3600);

        // Test CoreUnit deserialization
        let cores: CoreUnit = serde_json::from_str("\"2 cores\"").unwrap();
        assert_eq!(cores.millicores().unwrap(), 2000);

        let cores: CoreUnit = serde_json::from_str("2").unwrap();
        assert_eq!(cores.millicores().unwrap(), 2000);
    }
}