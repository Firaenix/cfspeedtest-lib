# cfspeedtest-lib - Cloudflare Speed Test Library

A Rust library for testing internet speed using Cloudflare's speed test service.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cfspeedtest-lib = "0.1.0"
```

## Usage

Here's a simple example of how to use the library:

```rust
use cfspeedtest_lib::speedtest::speed_test;
use cfspeedtest_lib::speedtest::PayloadSize;
use cfspeedtest_lib::OutputFormat;
use cfspeedtest_lib::SpeedTestOptions;

fn main() {
    // Initialize tracing for logging (optional)
    tracing_subscriber::fmt::init();

    // Define speedtest options
    let options = SpeedTestOptions {
        output_format: OutputFormat::None,
        ipv4: None,
        ipv6: None,
        verbose: false,
        upload_only: false,
        download_only: false,
        nr_tests: 5,
        nr_latency_tests: 20,
        max_payload_size: PayloadSize::M10,
        disable_dynamic_max_payload_size: false,
    };

    // Run the speed test
    let measurements = speed_test(reqwest::blocking::Client::new(), options);
    
    // Process the results
    for measurement in measurements {
        println!("{measurement}");
    }
}
```

### Running Specific Tests

You can run just latency or download/upload tests:

```rust
// For latency testing
let (measurements, avg) = run_latency_test(&client, 20, OutputFormat::None);

// For download testing
let mbits = test_download(&client, PayloadSize::M10.clone() as usize, OutputFormat::None);

// For upload testing
let mbits = test_upload(&client, PayloadSize::M10.clone() as usize, OutputFormat::None);
```

## Examples

Check the `examples/` directory for more usage examples:

- `simple_speedtest.rs` - A complete speed test example
- `latency_test.rs` - Example showing only latency testing
- `download_test.rs` - Example showing only download testing

Run examples with:

```sh
cargo run --example simple_speedtest
```

## Features

- Download and upload speed testing
- Latency measurements
- Configurable test sizes (100KB to 100MB)
- Multiple output formats (CSV, JSON, Pretty JSON)
- IPv4/IPv6 support
- Dynamic payload sizing to prevent excessive test times

## Development

### Logging

The library uses `tracing` for logging. To see debug logs:

```rust
// Initialize a tracing subscriber in your application
tracing_subscriber::fmt::init();
```

## License

MIT
