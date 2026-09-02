# tidalv2

[![Crates.io](https://img.shields.io/crates/v/tidalv2.svg)](https://crates.io/crates/tidalv2)
[![Documentation](https://docs.rs/tidalv2/badge.svg)](https://docs.rs/tidalv2)
[![GitHub](https://img.shields.io/badge/GitHub-phayes%2Ftidalv2-181717?logo=github)](https://github.com/phayes/tidalv2)

A Rust client library for TIDAL's v2 JSON:API.

This library provides type-safe, async access to TIDAL's music catalog and user data through their v2 API. The models and API clients are generated from TIDAL's OpenAPI specification, with hand-written client utilities for authentication and configuration.

**Note**: For TIDAL's v1 API, see [tidalrs](https://github.com/phayes/tidalrs).

## Features

- **JSON:API Support**: Full support for TIDAL's JSON:API v2 endpoints
- **Type-Safe Models**: Strongly-typed data models generated from OpenAPI spec
- **Async/Await**: Built on `reqwest` and `tokio` for async operations
- **OAuth2 Authentication**: Device-code, PKCE, client-credentials, and direct access-token flows, plus optional `tidalrs` interoperability
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

All default functionality is available without optional features. To create a
v2 client from an existing [`tidalrs`](https://crates.io/crates/tidalrs)
client, enable the `tidalrs` feature:

```toml
[dependencies]
tidalv2 = { version = "0.1", features = ["tidalrs"] }
tidalrs = "0.4"
```

## Usage

### Basic Setup

```rust
use tidalv2::client::TidalClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your TIDAL client ID
    let client = TidalClient::new("your_client_id".to_string());
    
    // Authenticate (device-code, PKCE, client-credentials, or a stored token)
    
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

Credentials are always caller-supplied. The same Bearer token is used for all
API calls, including HiRes streaming. HiRes availability depends on the OAuth
client that issued the token, not on a separate streaming credential.

### Device-code flow

```rust
use tidalv2::TidalClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = TidalClient::new("your_client_id".to_string())
    .with_client_secret("your_client_secret");

let device_auth = client.device_authorization().await?;
println!("Visit: {}", device_auth.url);
println!("Enter code: {}", device_auth.user_code);

let token = client
    .wait_for_authorization(
        &device_auth.device_code,
        device_auth.expires_in,
        device_auth.interval,
    )
    .await?;
println!("Authenticated as: {}", token.user.username);
# Ok(())
# }
```

You can still complete the flow yourself by polling `authorize(device_code, client_secret)` after the user approves.

### PKCE flow (HiRes-capable clients)

Use a client ID that TIDAL associates with HiRes playback, and the matching
redirect URI (the Android app uses `https://tidal.com/android/login/auth`).
Official developer-portal clients may not unlock HiRes.

```rust
use tidalv2::TidalClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = TidalClient::new("hires_client_id".to_string())
    .with_client_secret("hires_client_secret");

let url = client.start_pkce("https://tidal.com/android/login/auth")?;
println!("Visit: {}", url);

// After login, paste the full redirect URL (it includes ?code=...)
let token = client.finish_pkce(&redirect_url).await?;
println!("Authenticated as: {}", token.user.username);
# Ok(())
# }
```

### Client-credentials flow

```rust
use tidalv2::TidalClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = TidalClient::new("your_client_id".to_string())
    .with_client_secret("your_client_secret");
let authz = client.client_credentials().await?;
println!("App token expires at {:?}", authz.expires_timestamp);
# Ok(())
# }
```

### Direct access token

```rust
use tidalv2::{client::Authz, TidalClient};

// Token only — no automatic refresh
let client = TidalClient::new("your_client_id".to_string())
    .with_access_token("access_token");

// Persisted user session with refresh
let authz = Authz::new(
    "access_token".to_string(),
    Some("refresh_token".to_string()),
    Some(12345),
    Some("US".to_string()),
    None,
);
let client = TidalClient::new("your_client_id".to_string()).with_authz(authz);
```

With the `tidalrs` feature enabled, an existing `tidalrs::TidalClient` can
create a v2 client with the same HTTP client, country code, and current
authorization credentials:

```rust
use tidalrs::TidalClient as TidalV1Client;
use tidalv2::tidalrs::TidalV2ClientExt;

let client_id = "your_client_id";
let v1_client = TidalV1Client::new(client_id.to_string());

// Authenticate using device flow
let device_auth = v1_client.device_authorization().await?;
println!("Visit: {}", device_auth.url);
println!("Enter code: {}", device_auth.user_code);

// After the user completes authorization, poll as described in the tidalrs docs.
v1_client
    .authorize(&device_auth.device_code, "client_secret")
    .await?;

let v2_client = v1_client.tidalv2_client(client_id);
let album = v2_client.album_get("12345678", None).await?;
```

The two clients own separate copies of the authorization state after
conversion. Use the same client ID for both so the v2 client can refresh its
copy of the credentials.

## API Documentation

The library follows TIDAL's JSON:API specification. All response types implement the JSON:API resource structure with:

- `data`: The primary resource(s)
- `attributes`: Resource properties
- `relationships`: Related resources
- `included`: Embedded related resources

## Requirements

- Rust 1.70+
- Rust 1.85+ when the optional `tidalrs` feature is enabled
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
- Hand-written code is in `src/client.rs`, `src/auth.rs`, `src/error.rs`, and `src/tidalrs.rs`
- Tests use real TIDAL API endpoints and require credentials

## License

This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This library is not officially affiliated with TIDAL. Use at your own risk and ensure compliance with TIDAL's Terms of Service.

## Related Projects

- [tidalrs](https://github.com/phayes/tidalrs) - TIDAL v1 API client with authentication support
- [tidlers](https://crates.io/crates/tidlers) - Alternative v1 TIDAL client
