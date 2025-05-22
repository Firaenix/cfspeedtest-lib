use cfspeedtest_lib::speedtest::{test_download, PayloadSize};
use cfspeedtest_lib::OutputFormat;

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("Running download test...");
    
    let client = reqwest::blocking::Client::new();
    let payload_size = PayloadSize::M10.clone() as usize;
    
    let mbits = test_download(&client, payload_size, OutputFormat::None);
    
    println!("\nDownload speed: {:.2} mbit/s", mbits);
}
