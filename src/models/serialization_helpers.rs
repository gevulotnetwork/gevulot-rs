use std::str::FromStr;

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};

pub trait DefaultFactor {
    const FACTOR: i64;
}

#[derive(Debug)]
pub struct DefaultFactorOne;
impl DefaultFactor for DefaultFactorOne {
    const FACTOR: i64 = 1;
}

#[derive(Debug)]
pub struct DefaultFactorOneMegabyte;
impl DefaultFactor for DefaultFactorOneMegabyte {
    const FACTOR: i64 = 1024 * 1024;
}

#[derive(Debug)]
pub struct DefaultFactorOneGigabyte;
impl DefaultFactor for DefaultFactorOneGigabyte {
    const FACTOR: i64 = 1024 * 1024 * 1024;
}

#[derive(Debug, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum ByteUnit<D: DefaultFactor = DefaultFactorOne>{
    Number(i64),
    String(String),
    #[serde(skip)]
    Factor(std::marker::PhantomData<D>),
}

impl<D: DefaultFactor> ByteUnit<D> {
    pub fn bytes(&self) -> Result<i64, String> {
        match self {
            ByteUnit::Number(n) => Ok(*n),
            ByteUnit::String(s) => s.parse::<ByteSize>().map(|b| b.0 as i64),
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

#[derive(Debug, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum CoreUnit {
    Number(i64),
    String(String),
}

impl CoreUnit {
    pub fn millicores(&self) -> Result<i64, String> {
        match self {
            CoreUnit::Number(n) => Ok(*n * 1000), // Attention! default factor without unit is 1000, i.e. cpus: 1 results in 1000mcpu
            CoreUnit::String(s) => {
                let numeric: String = s.chars().take_while(|c| c.is_digit(10)).collect();
                let unit = s[numeric.len()..].to_lowercase().replace(" ", "");
                let base: i64 = numeric.parse().map_err(|e| format!("Invalid number: {}", e))?;
                Ok(base
                    * match unit.as_str() {
                        "cpu" | "cpus" => 1000, 
                        "gpu" | "gpus" => 1000,
                        "core" | "cores" => 1000,
                        "mcpu" | "mcpus" | "millicpu" | "millicpus" => 1,
                        "mgpu" | "mgpus" | "milligpu" | "milligpus" => 1,
                        "mcore" | "mcores" | "millicore" | "millicores" => 1,
                        "" => 1000, // Attention! default factor is 1000, i.e. cpus: 1 results in 1000mcpu
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

#[derive(Debug, Serialize, Deserialize, Eq)]
#[serde(untagged)]
pub enum TimeUnit {
    Number(i64),
    String(String),
}

impl TimeUnit {
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