//! This module contains VM runtime configuration definitions.
//!
//! Gevulot Network uses this configuration to setup environment inside VM before launching main
//! application.
//!
//! From VM perspective, this configuration will be mounted to `/mnt/gevulot-rt-config/config.yaml`.
//! Then VM is responsible to process it in order to execute the main application properly.
//!
//! [`follow_config`](RuntimeConfig::follow_config) allows to chain multiple configurations.
//! It contains path to the next configuration file to process after current one is finished.
//!
//! ## Processing
//!
//! The configuration SHOULD be processed in the following way:
//!
//! - Mount default filesystems (default filesystems are defined by VM itself);
//! - Setup ISA debug exit port if some (specifying multiple ports is not allowed).
//! - Mount filesystems in order of specification in [`mounts`](RuntimeConfig::mounts);
//! - Set environment variables specified in [`env`](RuntimeConfig::env);
//! - Set working directory to [`working_dir`](RuntimeConfig::working_dir);
//! - Load kernel modules in order of specification in
//!   [`kernel_modules`](RuntimeConfig::kernel_modules);
//! - Run boot commands in order of specification in [`bootcmd`](RuntimeConfig::bootcmd).
//!
//! If current configuration defines a `command` to run, it should be updated together with its
//! arguments. If there is a following configuration, it should be loaded and processed in the same
//! way.
//!
//! Finally after processing all configuration files, [`command`](RuntimeConfig::command) with
//! [`args`](RuntimeConfig::args) should be executed.
//!
//! Because loading following configuration file happens after mounting, it may be taken from
//! mounted directory.
//!
//! ## Configuration file
//!
//! Runtime configurations are expected to be serialized into and deserialized from YAML files.
//! Every Gevulot runtime configuration YAML file MUST start with `version` field.

use serde::de::Error;
use serde::{Deserialize, Serialize};

const MAJOR: u64 = 1;
const MINOR: u64 = 1;
const PATCH: u64 = 0;

const SEM_VERSION: semver::Version = semver::Version::new(MAJOR, MINOR, PATCH);

/// Version of runtime configuration.
pub const VERSION: &str = const_format::concatcp!(MAJOR, ".", MINOR, ".", PATCH);

/// Environment variable definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

/// Mount definition.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mount {
    pub source: String,
    pub target: String,
    pub fstype: Option<String>,
    pub flags: Option<u64>,
    pub data: Option<String>,
}

impl Mount {
    /// Create virtio 9p mount.
    ///
    /// This is commonly used for providing inputs and outputs to the program in VM.
    pub fn virtio9p(source: String, target: String) -> Self {
        Self {
            source,
            target,
            fstype: Some("9p".to_string()),
            flags: None,
            data: Some("trans=virtio,version=9p2000.L".to_string()),
        }
    }
}

/// Debug exit method depending on ISA.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "arch")]
pub enum DebugExit {
    /// Definition of exit code port for x86 QEMU.
    ///
    /// Defaults: iobase=0xf4,iosize=0x4
    #[serde(rename = "x86")]
    X86 {
        iobase: u16,
        /// No reason to set it to something other than 0x4,
        /// because `success_code` is `u32`.
        iosize: u16,
        /// Must be odd number greater than 1.
        /// 1 will be an error code.
        #[serde(rename = "success-code")]
        success_code: u32,
    },
}

impl DebugExit {
    pub const fn default_x86() -> Self {
        Self::X86 {
            iobase: 0xf4,
            iosize: 0x4,
            success_code: 0x3,
        }
    }
}

fn true_value() -> bool {
    true
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut version = String::deserialize(deserializer)?;
    // After deserialization, complete the version up to SemVer format: "X.Y.Z"
    let split = version.split('.').collect::<Vec<_>>();
    match split.len() {
        1 => {
            version.push_str(".0.0");
        }
        2 => {
            version.push_str(".0");
        }
        3 => {}
        _ => {
            return Err(D::Error::custom(
                "Gevulot runtime config: invalid version string",
            ));
        }
    }
    // Now compare versions in terms of SemVer
    let semversion = semver::Version::parse(&version).map_err(|err| {
        D::Error::custom(format!(
            "Gevulot runtime config: failed to parse version: {}",
            err
        ))
    })?;
    if semversion.major != SEM_VERSION.major || semversion > SEM_VERSION {
        return Err(D::Error::custom(
            "Gevulot runtime config: unsupported version",
        ));
    }
    Ok(version)
}

/// Gevulot VM runtime configuration.
///
/// See [module-level documentation](self) for more information.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct RuntimeConfig {
    /// Version of the config.
    #[serde(deserialize_with = "deserialize_version")]
    pub version: String,

    /// Program to execute.
    pub command: Option<String>,

    /// Arguments to the command.
    #[serde(default)]
    pub args: Vec<String>,

    /// Environment variables.
    #[serde(default)]
    pub env: Vec<EnvVar>,

    /// Working directory.
    pub working_dir: Option<String>,

    /// Mounts.
    #[serde(default)]
    pub mounts: Vec<Mount>,

    /// Default filesystems to mount.
    ///
    /// These filesystems are defined by VM itself. Typically these are `/proc`, `/sys` etc.
    ///
    /// When (de-)serlializing, defaults to `true`.
    #[serde(default = "true_value")]
    pub default_mounts: bool,

    /// Kernel modules.
    #[serde(default)]
    pub kernel_modules: Vec<String>,

    /// Debug exit (e.g. for QEMU `isa-debug-exit` device).
    ///
    /// If none specified, a simple shutdown is expected.
    pub debug_exit: Option<DebugExit>,

    /// Boot commands.
    ///
    /// Arbitrary commands to execute at initialization time.
    #[serde(default)]
    pub bootcmd: Vec<Vec<String>>,

    /// Path to another runtime configuration file to process after current one.
    pub follow_config: Option<String>,
}

// TODO: Implement strict version check to get proper error messages.
//       Deserializer needs to ensure that version field goes first (as it is described in docs
//       above) and decline going further if version is not correct. Otherwise such file:
//         abracadabra: xxxyyyzzz
//         version: 123
//       will report error "unknown field `abracadabra`" instead of version error
//       (like "no `version` at the beginning").

#[cfg(test)]
mod tests {
    use super::{DebugExit, EnvVar, RuntimeConfig};

    #[test]
    fn test_deserialize_version_ok() {
        let source = "
        version: 1
        command: echo
        ";
        let result = serde_yaml::from_str::<RuntimeConfig>(source);
        result.expect("deserialization should succeed");
    }

    #[test]
    fn test_deserialize_version_fail_1() {
        let source = "
        version: 0
        commands: echo
        ";
        let result = serde_yaml::from_str::<RuntimeConfig>(source);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(
            err.to_string(),
            "Gevulot runtime config: unsupported version at line 2 column 9".to_string()
        );
    }

    #[test]
    fn test_deserialize_version_fail_2() {
        let source = "
        abracadabra: 0
        version: 123
        ";
        let result = serde_yaml::from_str::<RuntimeConfig>(source);
        assert!(result.is_err());
        // TODO: check error message. Can be done only after completing TODO above.
    }

    const EXAMPLE_CONFIG: &str = "
    version: 1
    working-dir: /
    command: prover
    args: [--log, info]
    env:
      - key: TMPDIR
        value: /tmp
    mounts:
      - source: input-1
        target: /input/1
    default-mounts: true
    kernel-modules:
      - nvidia
    debug-exit:
      arch: x86
      iobase: 0xf4
      iosize: 0x4
      success-code: 0x3
    bootcmd:
      - [echo, booting]
    follow-config: /my/local/config.yaml
    ";

    #[test]
    fn test_deserialization_example_config() {
        let result = serde_yaml::from_str::<RuntimeConfig>(EXAMPLE_CONFIG)
            .expect("deserialization should succeed");
        assert_eq!(
            &result.command.expect("command should be present"),
            "prover"
        );
        assert_eq!(result.args, vec!["--log".to_string(), "info".to_string()]);
        assert_eq!(result.env.len(), 1);
        assert_eq!(
            result.env[0],
            EnvVar {
                key: "TMPDIR".to_string(),
                value: "/tmp".to_string()
            }
        );
        assert_eq!(
            &result.working_dir.expect("working dir should be present"),
            "/"
        );
        assert_eq!(result.mounts.len(), 1);
        assert_eq!(result.mounts[0].source, "input-1".to_string());
        assert_eq!(result.mounts[0].target, "/input/1".to_string());
        assert_eq!(result.mounts[0].fstype, None);
        assert_eq!(result.mounts[0].flags, None);
        assert_eq!(result.mounts[0].data, None);
        assert!(result.default_mounts);
        assert_eq!(result.kernel_modules, vec!["nvidia".to_string()]);
        assert_eq!(result.debug_exit, Some(DebugExit::default_x86()));
        assert_eq!(result.bootcmd, vec![vec!["echo", "booting"]]);
        assert_eq!(
            &result
                .follow_config
                .expect("follow config should be present"),
            "/my/local/config.yaml"
        );
    }
}
