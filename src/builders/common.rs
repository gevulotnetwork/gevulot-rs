/*!
 * # Common Builder Types
 *
 * This module provides common types and utilities used across all Gevulot builders.
 */

/// Represents units for measuring data size in the Gevulot system.
///
/// This enum provides different units of measurement for data sizes, allowing
/// for human-readable specification of memory and storage requirements.
///
/// # Units
///
/// * `Byte` - Individual bytes
/// * `Kilobyte` - 1024 bytes (KiB)
/// * `Megabyte` - 1024 kilobytes (MiB)
/// * `Gigabyte` - 1024 megabytes (GiB)
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::ByteUnit;
///
/// let unit = ByteUnit::Megabyte;
/// let bytes = unit.to_bytes(512); // Converts 512 MB to bytes (536,870,912)
/// ```
#[derive(Clone)]
pub enum ByteUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl ByteUnit {
    /// Converts a value in the given ByteUnit to bytes.
    ///
    /// This method applies the appropriate multiplication factor based on the unit
    /// to convert the value to raw bytes.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value to convert
    ///
    /// # Returns
    ///
    /// The equivalent value in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::ByteUnit;
    ///
    /// assert_eq!(ByteUnit::Byte.to_bytes(1), 1);
    /// assert_eq!(ByteUnit::Kilobyte.to_bytes(1), 1024);
    /// assert_eq!(ByteUnit::Megabyte.to_bytes(1), 1024 * 1024);
    /// assert_eq!(ByteUnit::Gigabyte.to_bytes(2), 2 * 1024 * 1024 * 1024);
    /// ```
    pub fn to_bytes(&self, value: u64) -> u64 {
        match self {
            ByteUnit::Byte => value,
            ByteUnit::Kilobyte => value * 1024,
            ByteUnit::Megabyte => value * 1024 * 1024,
            ByteUnit::Gigabyte => value * 1024 * 1024 * 1024,
        }
    }
}

/// Represents a size value with an associated unit for memory and storage specifications.
///
/// This struct combines a numeric value with a ByteUnit to create a human-readable
/// and programmatically-convenient way to specify data sizes throughout the Gevulot system.
///
/// # Fields
///
/// * `value` - The numeric quantity
/// * `unit` - The unit of measurement (Byte, Kilobyte, Megabyte, Gigabyte)
///
/// # Examples
///
/// ```
/// use gevulot_rs::builders::{ByteSize, ByteUnit};
///
/// // Create a 512 MB specification
/// let memory_size = ByteSize::new(512, ByteUnit::Megabyte);
///
/// // Convert to raw bytes for the protocol
/// let bytes = memory_size.to_bytes();
/// assert_eq!(bytes, 512 * 1024 * 1024);
///
/// // Display in a human-readable format
/// assert_eq!(memory_size.to_string(), "512 MB");
/// ```
#[derive(Clone)]
pub struct ByteSize {
    value: u64,
    unit: ByteUnit,
}

impl ByteSize {
    /// Creates a new ByteSize instance.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric quantity
    /// * `unit` - The unit of measurement
    ///
    /// # Returns
    ///
    /// A new ByteSize instance
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{ByteSize, ByteUnit};
    ///
    /// let small = ByteSize::new(100, ByteUnit::Kilobyte);
    /// let large = ByteSize::new(2, ByteUnit::Gigabyte);
    /// ```
    pub fn new(value: u64, unit: ByteUnit) -> Self {
        Self { value, unit }
    }

    /// Converts the ByteSize to raw bytes.
    ///
    /// # Returns
    ///
    /// The size in bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{ByteSize, ByteUnit};
    ///
    /// let size = ByteSize::new(2, ByteUnit::Megabyte);
    /// assert_eq!(size.to_bytes(), 2 * 1024 * 1024);
    /// ```
    pub fn to_bytes(&self) -> u64 {
        self.unit.to_bytes(self.value)
    }
}

impl std::fmt::Display for ByteSize {
    /// Formats the ByteSize for display.
    ///
    /// This implementation provides a human-readable representation of the size,
    /// including both the numeric value and the appropriate unit suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{ByteSize, ByteUnit};
    ///
    /// let size = ByteSize::new(512, ByteUnit::Megabyte);
    /// assert_eq!(size.to_string(), "512 MB");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit_str = match self.unit {
            ByteUnit::Byte => "B",
            ByteUnit::Kilobyte => "KB",
            ByteUnit::Megabyte => "MB",
            ByteUnit::Gigabyte => "GB",
        };
        write!(f, "{} {}", self.value, unit_str)
    }
}

impl From<(u64, ByteUnit)> for ByteSize {
    /// Converts a tuple of (u64, ByteUnit) to a ByteSize.
    ///
    /// This conversion implementation allows for convenient creation of ByteSize
    /// instances from a tuple.
    ///
    /// # Arguments
    ///
    /// * `value` - A tuple containing the value and unit
    ///
    /// # Returns
    ///
    /// A new ByteSize instance
    ///
    /// # Examples
    ///
    /// ```
    /// use gevulot_rs::builders::{ByteSize, ByteUnit};
    ///
    /// let size: ByteSize = (256, ByteUnit::Megabyte).into();
    /// assert_eq!(size.to_bytes(), 256 * 1024 * 1024);
    /// ```
    fn from(value: (u64, ByteUnit)) -> Self {
        Self {
            value: value.0,
            unit: value.1,
        }
    }
} 