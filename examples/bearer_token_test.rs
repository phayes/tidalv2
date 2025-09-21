use tidalv2::{init_logging, apis::configuration::Configuration, apis::genres_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    
    println!("Bearer Token Priority Test");
    println!("=========================");
    println!();
    
    // Test 1: Only bearer_access_token
    println!("Test 1: Only bearer_access_token set");
    let mut config1 = Configuration::default();
    config1.bearer_access_token = Some("bearer-token-123".to_string());
    
    match genres_api::genres_get(&config1, None, None).await {
        Ok(_) => println!("✅ Request sent successfully"),
        Err(_) => println!("❌ Request failed (expected due to network/auth)"),
    }
    println!();
    
    // Test 2: Only oauth_access_token
    println!("Test 2: Only oauth_access_token set");
    let mut config2 = Configuration::default();
    config2.oauth_access_token = Some("oauth-token-456".to_string());
    
    match genres_api::genres_get(&config2, None, None).await {
        Ok(_) => println!("✅ Request sent successfully"),
        Err(_) => println!("❌ Request failed (expected due to network/auth)"),
    }
    println!();
    
    // Test 3: Both tokens (bearer should take priority)
    println!("Test 3: Both tokens set (bearer_access_token should take priority)");
    let mut config3 = Configuration::default();
    config3.bearer_access_token = Some("bearer-token-priority".to_string());
    config3.oauth_access_token = Some("oauth-token-ignored".to_string());
    
    match genres_api::genres_get(&config3, None, None).await {
        Ok(_) => println!("✅ Request sent successfully"),
        Err(_) => println!("❌ Request failed (expected due to network/auth)"),
    }
    
    println!();
    println!("Check the debug logs above to verify the correct Authorization headers!");
    println!("All should show 'authorization': Sensitive in the headers.");
    
    Ok(())
}
