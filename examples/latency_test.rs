use cfspeedtest_lib::speedtest::run_latency_test;
use cfspeedtest_lib::OutputFormat;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("Running latency test...");
    
    let client = reqwest::blocking::Client::new();
    let (measurements, avg) = run_latency_test(&client, 20, OutputFormat::None);
    
    println!("\nLatency Results:");
    println!("Average latency: {:.2} ms", avg);
    println!("All measurements (ms):");
    for (i, measurement) in measurements.iter().enumerate() {
        println!("Test {}: {:.2} ms", i + 1, measurement);
    }
}
