use tidalv2::{apis::configuration::Configuration, apis::providers_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging - set RUST_LOG=debug to see detailed headers
    // or RUST_LOG=info to see basic request/response info
    env_logger::init();

    println!("TIDAL API Logging Demo");
    println!("======================");
    println!("Set RUST_LOG=info to see basic request/response logging");
    println!("Set RUST_LOG=debug to see detailed headers including bearer token");
    println!();

    // Create a configuration with a bearer token for testing
    let mut config = Configuration::default();
    config.bearer_access_token = Some("test-bearer-token-12345".to_string());

    // Make a simple API call - this will demonstrate the logging
    println!("Making API call to get providers...");

    match providers_api::provider_list(&config, None).await {
        Ok(_response) => {
            println!("✅ API call successful!");
            println!("Check the logs above to see the HTTP request details:");
            println!("- Method: GET");
            println!("- URL: https://openapi.tidal.com/v2/providers");
            println!("- Response code should be visible in logs");
            println!("- Bearer token should be visible in Authorization header with debug logging");
        }
        Err(e) => {
            println!("❌ API call failed: {}", e);
            println!("This is expected if you don't have proper authentication,");
            println!("but you should still see the logging output above!");
            println!();
            println!("🔍 Look for 'authorization': Sensitive in the debug headers above.");
            println!("This indicates the bearer token is being sent correctly!");
        }
    }

    Ok(())
}
