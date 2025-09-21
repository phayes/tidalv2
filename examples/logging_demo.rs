use tidalv2::{init_logging, apis::configuration::Configuration, apis::genres_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging - set RUST_LOG=debug to see detailed headers
    // or RUST_LOG=info to see basic request/response info
    init_logging();
    
    println!("TIDAL API Logging Demo");
    println!("======================");
    println!("Set RUST_LOG=info to see basic request/response logging");
    println!("Set RUST_LOG=debug to see detailed headers");
    println!();
    
    // Create a basic configuration
    let config = Configuration::default();
    
    // Make a simple API call - this will demonstrate the logging
    println!("Making API call to get genres...");
    
    match genres_api::genres_get(&config, None, None).await {
        Ok(_response) => {
            println!("✅ API call successful!");
            println!("Check the logs above to see the HTTP request details:");
            println!("- Method: GET");
            println!("- URL: https://openapi.tidal.com/v2/genres");
            println!("- Response code should be visible in logs");
            println!("- Headers should be visible with debug logging");
        }
        Err(e) => {
            println!("❌ API call failed: {}", e);
            println!("This is expected if you don't have proper authentication,");
            println!("but you should still see the logging output above!");
        }
    }
    
    Ok(())
}
