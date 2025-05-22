pub mod boxplot;
pub mod measurements;
pub mod progress;
pub mod speedtest;
use std::fmt;
use std::fmt::Display;

use speedtest::PayloadSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Csv,
    Json,
    JsonPretty,
    StdOut,
    None,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl OutputFormat {
    pub fn from(output_format_string: String) -> Result<Self, String> {
        match output_format_string.to_lowercase().as_str() {
            "csv" => Ok(Self::Csv),
            "json" => Ok(Self::Json),
            "json_pretty" | "json-pretty" => Ok(Self::JsonPretty),
            "stdout" => Ok(Self::StdOut),
            _ => Err("Value needs to be one of csv, json or json-pretty".to_string()),
        }
    }
}

/// Configuration options for the speed test
#[derive(Debug, Clone)]
pub struct SpeedTestOptions {
    /// Number of test runs per payload size.
    pub nr_tests: u32,

    /// Number of latency tests to run
    pub nr_latency_tests: u32,

    /// The max payload size in bytes to use
    pub max_payload_size: PayloadSize,

    /// Set the output format
    pub output_format: OutputFormat,

    /// Enable verbose output (only affects StdOut output format)
    pub verbose: bool,

    /// Force IPv4 with provided source IPv4 address or the default IPv4 address bound to the main interface
    pub ipv4: Option<String>,

    /// Force IPv6 with provided source IPv6 address or the default IPv6 address bound to the main interface
    pub ipv6: Option<String>,

    /// Disables dynamically skipping tests with larger payload sizes if the tests for the previous payload
    /// size took longer than 5 seconds
    pub disable_dynamic_max_payload_size: bool,

    /// Test download speed only
    pub download_only: bool,

    /// Test upload speed only
    pub upload_only: bool,
}

impl Default for SpeedTestOptions {
    fn default() -> Self {
        Self {
            nr_tests: 10,
            nr_latency_tests: 25,
            max_payload_size: PayloadSize::M25,
            output_format: OutputFormat::None,
            verbose: false,
            ipv4: None,
            ipv6: None,
            disable_dynamic_max_payload_size: false,
            download_only: false,
            upload_only: false,
        }
    }
}

impl SpeedTestOptions {
    /// Returns whether download tests should be performed
    pub fn should_download(&self) -> bool {
        self.download_only || !self.upload_only
    }

    /// Returns whether upload tests should be performed
    pub fn should_upload(&self) -> bool {
        self.upload_only || !self.download_only
    }
}

// Backward compatibility for examples - will be deprecated
pub use SpeedTestOptions as SpeedTestCLIOptions;
