use cfspeedtest_lib::speedtest::speed_test;
use cfspeedtest_lib::speedtest::PayloadSize;
use cfspeedtest_lib::OutputFormat;
use cfspeedtest_lib::SpeedTestOptions;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // define speedtest options
    let options = SpeedTestOptions {
        output_format: OutputFormat::None, // don't write to stdout
        ipv4: None,                        // don't force ipv4 usage
        ipv6: None,                        // don't force ipv6 usage
        verbose: false,
        upload_only: false,
        download_only: false,
        nr_tests: 5,
        nr_latency_tests: 20,
        max_payload_size: PayloadSize::M10,
        disable_dynamic_max_payload_size: false,
    };

    println!("Running speed test...");
    let measurements = speed_test(reqwest::blocking::Client::new(), options);
    
    println!("\nResults:");
    measurements
        .iter()
        .for_each(|measurement| println!("{measurement}"));
}
