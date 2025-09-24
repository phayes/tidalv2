use async_recursion::async_recursion;
use log::{info, trace};
use models::*;
use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
use tidalv2::{apis, models};
use ResourceType::*;

/// Ensure logging is only initialized once across all tests
static INIT_LOGGER: Once = Once::new();

/// Initialize logging exactly once, safe to call from multiple tests
fn init_logging_once() {
    INIT_LOGGER.call_once(|| {
        env_logger::init();
    });
}

/// Maximum total number of API requests allowed across all tests
const MAX_TOTAL_REQUESTS: usize = 400;

/// Global atomic counter tracking total API requests made
static TOTAL_REQUESTS: AtomicUsize = AtomicUsize::new(0);

/// Integration tests for TIDAL API
///
/// These tests require TIDAL_BEARER_ACCESS_TOKEN environment variable to be set.
/// The tests perform read-only operations to validate API parsing and resource walking.
///
/// Run with: cargo test --test integration_tests -- --nocapture

/// Check if we can make another API request without exceeding the global limit
fn can_make_request() -> bool {
    TOTAL_REQUESTS.load(Ordering::SeqCst) < MAX_TOTAL_REQUESTS
}

/// Increment the global request counter and return the new count
fn increment_request_count() {
    let total = TOTAL_REQUESTS.fetch_add(1, Ordering::SeqCst) + 1;
    info!("Total requests: {} / {}", total, MAX_TOTAL_REQUESTS);
}

/// Get the current total request count
#[allow(dead_code)]
fn get_request_count() -> usize {
    TOTAL_REQUESTS.load(Ordering::SeqCst)
}

/// Reset the global request counter (useful for testing)
fn reset_request_count() {
    TOTAL_REQUESTS.store(0, Ordering::SeqCst);
}

#[tokio::test]
async fn test_search_and_walk_resources() {
    reset_request_count();
    // Initialize logging for HTTP request/response debugging
    init_logging_once();

    // Get bearer token from environment
    let bearer_token = env::var("TIDAL_BEARER_ACCESS_TOKEN")
        .expect("TIDAL_BEARER_ACCESS_TOKEN environment variable must be set");

    // Configure API client
    let mut config = apis::configuration::Configuration::new();
    config.bearer_access_token = Some(bearer_token);
    config.country_code = "US".to_string();

    // Perform search for a popular query
    let search_query = "taylor swift";
    info!("Performing search for: {}", search_query);

    // Check global request limit before making search request
    if !can_make_request() {
        return;
    }

    increment_request_count();
    let search_result = apis::search_results_api::search_result_get(
        &config,
        search_query,
        None, // explicit_filter
        Some(vec![
            Albums.to_string(),
            Artists.to_string(),
            Tracks.to_string(),
            Playlists.to_string(),
            Videos.to_string(),
            "topHits".to_string(),
        ]),
    )
    .await;

    match search_result {
        Ok(search_response) => {
            trace!(
                "Search successful! Processing search result with ID: {}",
                search_response.data.id
            );

            if let Some(attributes) = &search_response.data.attributes {
                trace!("Search tracking ID: {}", attributes.tracking_id);
                if let Some(did_you_mean) = &attributes.did_you_mean {
                    trace!("Did you mean: {}", did_you_mean);
                }
            }

            // Walk through search result relationships using simple serial approach
            walk_search_result(&config, &search_response).await;
        }
        Err(e) => {
            panic!("Search failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_search_different_queries() {
    reset_request_count();
    // Initialize logging
    init_logging_once();

    // Get bearer token from environment
    let bearer_token = env::var("TIDAL_BEARER_ACCESS_TOKEN")
        .expect("TIDAL_BEARER_ACCESS_TOKEN environment variable must be set");

    // Configure API client
    let mut config = apis::configuration::Configuration::new();
    config.bearer_access_token = Some(bearer_token);
    config.country_code = "US".to_string();

    let search_queries = vec!["the beatles", "jazz", "rock", "classical music", "hip hop"];

    for query in search_queries {
        trace!("Testing search query: {}", query);

        // Check global request limit before each search
        if !can_make_request() {
            break;
        }

        increment_request_count();
        trace!("Making search request for '{}'", query);

        let search_result = apis::search_results_api::search_result_get(
            &config,
            query,
            None,
            Some(vec![
                Albums.to_string(),
                Artists.to_string(),
                Tracks.to_string(),
            ]),
        )
        .await;

        match search_result {
            Ok(response) => {
                trace!("✓ Search '{}' successful, ID: {}", query, response.data.id);

                // Quick validation of response structure
                if let Some(_relationships) = &response.data.relationships {
                    trace!("  Found relationships for albums, artists, tracks, etc.");
                }

                if let Some(included) = &response.included {
                    trace!("  Included {} additional resources", included.len());
                }
            }
            Err(e) => {
                panic!("Search '{}' failed: {:?}", query, e);
            }
        }
    }
}

/// Simple serial resource walking
async fn walk_search_result(
    config: &apis::configuration::Configuration,
    search_response: &Resource<SearchResult>,
) {
    let mut processed_ids: HashSet<String> = HashSet::new();

    // Process relationships directly without queuing
    if let Some(relationships) = &search_response.data.relationships {
        // Process albums
        if let Some(data) = &relationships.albums.data {
            for resource_id in data {
                if !can_make_request() {
                    return;
                }
                if !processed_ids.contains(&resource_id.id) {
                    processed_ids.insert(resource_id.id.clone());
                    trace!("Processing album: {}", resource_id.id);
                    process_album(config, &resource_id.id, 1).await;
                }
            }
        }

        // Process artists
        if let Some(data) = &relationships.artists.data {
            for resource_id in data {
                if !can_make_request() {
                    return;
                }
                if !processed_ids.contains(&resource_id.id) {
                    processed_ids.insert(resource_id.id.clone());
                    trace!("Processing artist: {}", resource_id.id);
                    process_artist(config, &resource_id.id, 1).await;
                }
            }
        }

        // Process tracks
        if let Some(data) = &relationships.tracks.data {
            for resource_id in data {
                if !can_make_request() {
                    return;
                }
                if !processed_ids.contains(&resource_id.id) {
                    processed_ids.insert(resource_id.id.clone());
                    trace!("Processing track: {}", resource_id.id);
                    process_track(config, &resource_id.id, 1).await;
                }
            }
        }

        // Process playlists
        if let Some(data) = &relationships.playlists.data {
            for resource_id in data {
                if !can_make_request() {
                    info!(
                        "✓ Resource walking stopped: reached global request limit of {}",
                        MAX_TOTAL_REQUESTS
                    );
                    return;
                }
                if !processed_ids.contains(&resource_id.id) {
                    processed_ids.insert(resource_id.id.clone());
                    info!("Processing playlist: {}", resource_id.id);
                    process_playlist(config, &resource_id.id, 1).await;
                }
            }
        }

        // Process videos
        if let Some(data) = &relationships.videos.data {
            for resource_id in data {
                if !can_make_request() {
                    return;
                }
                if !processed_ids.contains(&resource_id.id) {
                    processed_ids.insert(resource_id.id.clone());
                    trace!("Processing video: {}", resource_id.id);
                    process_video(config, &resource_id.id, 1).await;
                }
            }
        }

        // Process top hits
        if let Some(data) = &relationships.top_hits.data {
            for resource_id in data {
                if !can_make_request() {
                    return;
                }
                if !processed_ids.contains(&resource_id.id) {
                    processed_ids.insert(resource_id.id.clone());
                    trace!(
                        "Processing top hit: {} (type: {})",
                        resource_id.id, resource_id.r#type
                    );
                    // Process based on type
                    match resource_id.r#type {
                        Albums => process_album(config, &resource_id.id, 2).await,
                        Artists => process_artist(config, &resource_id.id, 2).await,
                        Tracks => process_track(config, &resource_id.id, 2).await,
                        Videos => process_video(config, &resource_id.id, 2).await,
                        _ => {
                            panic!("Unknown resource type: {}", resource_id.r#type);
                        },
                    }
                }
            }
        }
    }

    info!("Simple resource walking finished");
}

#[async_recursion]
async fn process_album(config: &apis::configuration::Configuration, album_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }
    
    trace!("Loading album: {}", album_id);
    increment_request_count();

    let result = apis::albums_api::album_get(
        config,
        album_id,
        Some(vec![Artists.to_string(), "items".to_string()]),
    )
    .await;

    match result {
        Ok(album_response) => {
           if recurse > 0 {
            for resource_id in album_response.data.relationships.unwrap().items.data.unwrap() {
                match resource_id.r#type {
                    Tracks => process_track(config, &resource_id.id, recurse - 1).await,
                    Albums => process_album(config, &resource_id.id, recurse - 1).await,
                    Artists => process_artist(config, &resource_id.id, recurse - 1).await,
                    Videos => process_video(config, &resource_id.id, recurse - 1).await,
                    _ => {
                        panic!("Unknown resource type: {}", resource_id.r#type);
                    }
                }
            }
           }
        }
        Err(e) => {
            panic!("Failed to load album {}: {:?}", album_id, e);
        }
    }
}

#[async_recursion]
async fn process_artist(config: &apis::configuration::Configuration, artist_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }
    
    trace!("Loading artist: {}", artist_id);
    increment_request_count();

    let result = apis::artists_api::artist_get(
        config,
        artist_id,
        Some(vec![Albums.to_string(), Tracks.to_string()]),
        Some("FINGERPRINT".to_string()),
    )
    .await;

    match result {
        Ok(artist_response) => {
            if recurse > 0 {
                if let Some(relationships) = &artist_response.data.relationships {
                    if let Some(albums_data) = &relationships.albums.data {
                        for resource_id in albums_data {
                            if !can_make_request() {
                                return;
                            }
                            process_album(config, &resource_id.id, recurse - 1).await;
                        }
                    }
                    if let Some(tracks_data) = &relationships.tracks.data {
                        for resource_id in tracks_data {
                            if !can_make_request() {
                                return;
                            }
                            process_track(config, &resource_id.id, recurse - 1).await;
                        }
                    }
                }
            }
        }
        Err(e) => {
            panic!("Failed to load artist {}: {:?}", artist_id, e);
        }
    }
}

#[async_recursion]
async fn process_track(config: &apis::configuration::Configuration, track_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }
    
    trace!("Loading track: {}", track_id);
    increment_request_count();

    let result = apis::tracks_api::track_get(
        config,
        track_id,
        Some(vec![Artists.to_string(), Albums.to_string()]),
    )
    .await;

    match result {
        Ok(track_response) => {
            if recurse > 0 {
                if let Some(relationships) = &track_response.data.relationships {
                    if let Some(artists_data) = &relationships.artists.data {
                        for resource_id in artists_data {
                            if !can_make_request() {
                                return;
                            }
                            process_artist(config, &resource_id.id, recurse - 1).await;
                        }
                    }
                    if let Some(albums_data) = &relationships.albums.data {
                        for resource_id in albums_data {
                            if !can_make_request() {
                                return;
                            }
                            process_album(config, &resource_id.id, recurse - 1).await;
                        }
                    }
                }
            }
        }
        Err(e) => {
            panic!("Failed to load track {}: {:?}", track_id, e);
        }
    }
}

#[async_recursion]
async fn process_playlist(config: &apis::configuration::Configuration, playlist_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }
    
    trace!("Loading playlist: {}", playlist_id);
    increment_request_count();

    let result =
        apis::playlists_api::playlist_get(config, playlist_id, Some(vec!["items".to_string()]))
            .await;

    match result {
        Ok(playlist_response) => {
            if recurse > 0 {
                if let Some(relationships) = &playlist_response.data.relationships {
                    if let Some(items_data) = &relationships.items.data {
                        for resource_id in items_data {
                            if !can_make_request() {
                                return;
                            }
                            match resource_id.r#type {
                                Tracks => process_track(config, &resource_id.id, recurse - 1).await,
                                Albums => process_album(config, &resource_id.id, recurse - 1).await,
                                Artists => process_artist(config, &resource_id.id, recurse - 1).await,
                                Videos => process_video(config, &resource_id.id, recurse - 1).await,
                                _ => {
                                    panic!("Unknown resource type: {}", resource_id.r#type);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            panic!("Failed to load playlist {}: {:?}", playlist_id, e);
        }
    }
}

#[async_recursion]
async fn process_video(config: &apis::configuration::Configuration, video_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }
    
    trace!("Loading video: {}", video_id);
    increment_request_count();
    let result = apis::videos_api::video_get(
        config,
        video_id,
        Some(vec![Artists.to_string(), Albums.to_string()]),
    )
    .await;

    match result {
        Ok(video_response) => {
            if recurse > 0 {
                if let Some(relationships) = &video_response.data.relationships {
                    if let Some(artists_data) = &relationships.artists.data {
                        for resource_id in artists_data {
                            if !can_make_request() {
                                return;
                            }
                            process_artist(config, &resource_id.id, recurse - 1).await;
                        }
                    }
                    if let Some(albums_data) = &relationships.albums.data {
                        for resource_id in albums_data {
                            if !can_make_request() {
                                return;
                            }
                            process_album(config, &resource_id.id, recurse - 1).await;
                        }
                    }
                }
            }
        }
        Err(e) => {
            panic!("Failed to load video {}: {:?}", video_id, e);
        }
    }
}
