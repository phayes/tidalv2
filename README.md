# tidalv2

A Rust client library for TIDAL's v2 JSON:API.

This library provides type-safe, async access to TIDAL's music catalog and user data through their v2 API. The models and API clients are generated from TIDAL's OpenAPI specification, with hand-written client utilities for authentication and configuration.

## Features

- **JSON:API Support**: Full support for TIDAL's JSON:API v2 endpoints
- **Type-Safe Models**: Strongly-typed data models generated from OpenAPI spec
- **Async/Await**: Built on `reqwest` and `tokio` for async operations
- **OAuth2 Authentication**: Device flow authentication with automatic token refresh via `tidalrs` integration
- **Comprehensive API Coverage**:
  - Albums, artists, tracks, videos
  - Playlists and user collections
  - Search and recommendations
  - Track manifests and streaming
  - User management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tidalv2 = "0.1"
```

## Usage

### Basic Setup

```rust
use tidalv2::client::TidalClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your TIDAL client ID
    let client = TidalClient::new("your_client_id".to_string());
    
    // Perform authentication (see tidalrs documentation)
    // ...
    
    Ok(())
}
```

### Searching for Content

```rust
use tidalv2::models::ResourceType;

let search_result = client
    .search_result_get(
        "taylor swift",
        None, // explicit_filter
        Some(vec![
            ResourceType::Albums.to_string(),
            ResourceType::Artists.to_string(),
            ResourceType::Tracks.to_string(),
        ]),
    )
    .await?;
```

### Getting Album Details

```rust
let album = client
    .album_get(
        "12345678",
        Some(vec!["artists".to_string(), "items".to_string()]),
    )
    .await?;

println!("Album: {}", album.data.attributes.title);
```

### Managing User Collections

```rust
let user_id = "user_id_here";

// Get user's playlists
let playlists = client
    .user_collection_playlists(user_id, None, None)
    .await?;

// Get user's favorite albums
let albums = client
    .user_collection_albums(user_id, "US", None, None)
    .await?;
```

## Authentication

This library can be used with the [`tidalrs`](https://crates.io/crates/tidalrs) crate for OAuth2 authentication. To use both together, add them as separate dependencies:

```toml
[dependencies]
tidalv2 = "0.1"
tidalrs = "0.4"
```

Then use `TidalClient` from `tidalrs` which provides authentication, and import the tidalv2 configuration trait:

```rust
use tidalrs::TidalClient;

// Note: The Configurator trait for tidalv2 integration is only available
// when both crates are used together. You'll need to implement a small
// extension trait or use the generated APIs directly with your own Configuration.

let client = TidalClient::new("your_client_id".to_string());

// Authenticate using device flow
let device_auth = client.device_authorization().await?;
println!("Visit: {}", device_auth.url);
println!("Enter code: {}", device_auth.user_code);

// Complete authentication
let authz_token = client.authorize(&device_auth.device_code, "client_secret").await?;
```

For direct API usage without tidalrs, create a `Configuration` and use the generated API clients:

```rust
use tidalv2::apis::configuration::Configuration;
use tidalv2::apis::albums_api;

let config = Configuration {
    base_path: "https://openapi.tidal.com/v2".to_string(),
    bearer_access_token: Some("your_access_token".to_string()),
    country_code: "US".to_string(),
    ..Default::default()
};

let album = albums_api::album_get(&config, "12345678", None).await?;
```

## API Documentation

The library follows TIDAL's JSON:API specification. All response types implement the JSON:API resource structure with:

- `data`: The primary resource(s)
- `attributes`: Resource properties
- `relationships`: Related resources
- `included`: Embedded related resources

## Requirements

- Rust 1.70+
- Valid TIDAL API client credentials
- For streaming features, appropriate TIDAL subscription

## Testing

The library includes comprehensive integration tests that validate API parsing and resource walking. Tests require TIDAL API credentials:

```bash
export TIDAL_CLIENT_ID="your_client_id"
export TIDAL_REFRESH_TOKEN="your_refresh_token"
export TIDAL_ACCESS_TOKEN="your_access_token"

cargo test --ignored
```

## Contributing

Contributions are welcome! Please note:

- Models and API clients in `src/models/` and `src/apis/` are OpenAPI-generated
- Hand-written code is in `src/client.rs`, `src/error.rs`, and `src/tidalrs.rs`
- Tests use real TIDAL API endpoints and require credentials

## License

This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This library is not officially affiliated with TIDAL. Use at your own risk and ensure compliance with TIDAL's Terms of Service.

## Related Projects

- [tidalrs](https://github.com/phayes/tidalrs) - TIDAL v1 API client with authentication support
- [tidlers](https://crates.io/crates/tidlers) - Alternative v1 TIDAL client
