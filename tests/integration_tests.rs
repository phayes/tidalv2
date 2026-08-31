use async_recursion::async_recursion;
use log::{info, trace};
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::LazyLock;
use std::sync::{Mutex, Once};
use tidalv2::models::*;
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
const MAX_TOTAL_REQUESTS: usize = 50;

/// Global atomic counter tracking total API requests made
static TOTAL_REQUESTS: AtomicUsize = AtomicUsize::new(0);

/// Static tracker of processed resources to avoid infinite loops and redundant processing
static PROCESSED_RESOURCES: LazyLock<Mutex<HashMap<ResourceType, HashSet<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Integration tests for TIDAL API
///
/// These tests require the following environment variables to be set:
/// - TIDAL_CLIENT_ID: Your Tidal API client ID
/// - TIDAL_CLIENT_SECRET: Your Tidal API client secret (optional, for some operations)
/// - TIDAL_REFRESH_TOKEN: Refresh token for automatic token renewal (required)
/// - TIDAL_ACCESS_TOKEN: Valid access token for API authentication (optional, will be generated from refresh token if not provided)
///
/// The tests perform read-only operations to validate API parsing and resource walking.
/// Note: These tests are marked with #[ignore] and require the --ignored flag to run.
///
/// Run with: cargo test --ignored -- --nocapture
/// Or run specific tests: cargo test test_search_and_walk_resources --ignored -- --nocapture

/// Check if we can make another API request without exceeding the global limit
fn can_make_request() -> bool {
    let can_make_request = TOTAL_REQUESTS.load(Ordering::SeqCst) < MAX_TOTAL_REQUESTS;
    if !can_make_request {
        // Report out all processed resources by type => count
        if let Ok(processed) = PROCESSED_RESOURCES.lock() {
            let mut total_resources = 0;
            for (resource_type, resource_set) in processed.iter() {
                let count = resource_set.len();
                total_resources += count;
                info!("  {:?}: {} resources processed", resource_type, count);
            }
            info!("📊 Total unique resources processed: {}", total_resources);
        }
    }
    can_make_request
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

/// Check if a resource has already been processed
fn is_resource_processed(resource_type: ResourceType, resource_id: &str) -> bool {
    if let Ok(processed) = PROCESSED_RESOURCES.lock() {
        if let Some(type_set) = processed.get(&resource_type) {
            return type_set.contains(resource_id);
        }
    }
    false
}

/// Mark a resource as processed
fn mark_resource_processed(resource_type: ResourceType, resource_id: &str) {
    if let Ok(mut processed) = PROCESSED_RESOURCES.lock() {
        processed
            .entry(resource_type)
            .or_insert_with(HashSet::new)
            .insert(resource_id.to_string());
    }
}

#[tokio::test]
#[ignore]
async fn test_search_and_walk_resources() {
    reset_request_count();
    // Initialize logging for HTTP request/response debugging
    init_logging_once();

    // Get required environment variables
    let client_id: String =
        env::var("TIDAL_CLIENT_ID").expect("TIDAL_CLIENT_ID environment variable must be set");

    let refresh_token = env::var("TIDAL_REFRESH_TOKEN")
        .expect("TIDAL_REFRESH_TOKEN environment variable must be set");

    // Get optional access token (client will generate one if not provided)
    let access_token = env::var("TIDAL_ACCESS_TOKEN").unwrap_or_else(|_| String::new()); // Empty string if not provided

    // Configure API client with authentication
    let authz = tidalv2::client::Authz::new(
        access_token,
        refresh_token,
        0, // user_id will be updated when we get user info
        Some("US".to_string()),
        Some(u64::MAX), // expires_timestamp - set to far future for testing
    );

    let mut client = tidalv2::client::TidalClient::new(client_id).with_authz(authz);

    client.set_country_code("US".to_string());

    // Perform search for a popular query
    let search_query = "taylor swift";
    info!("Performing search for: {}", search_query);

    // Check global request limit before making search request
    if !can_make_request() {
        return;
    }

    increment_request_count();
    let search_result = client
        .search_result_get(
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

            let attributes = &search_response.data.attributes;
            trace!("Search tracking ID: {}", attributes.tracking_id);
            if let Some(did_you_mean) = &attributes.did_you_mean {
                trace!("Did you mean: {}", did_you_mean);
            }

            // Walk through search result relationships using simple serial approach
            walk_search_result(&client, &search_response).await;
        }
        Err(e) => {
            panic!("Search failed: {:?}", e);
        }
    }
}

/// Simple serial resource walking
async fn walk_search_result(
    client: &tidalv2::client::TidalClient,
    search_response: &Resource<tidalv2::models::search_result::SearchResult>,
) {
    // Process relationships directly without queuing
    if let Some(relationships) = &search_response.data.relationships {
        // Process albums
        if let Some(data) = &relationships.albums.data {
            for resource_id in data {
                process_album(client, &resource_id.id, 2).await;
            }
        }

        // Process artists
        if let Some(data) = &relationships.artists.data {
            for resource_id in data {
                process_artist(client, &resource_id.id, 2).await;
            }
        }

        // Process tracks
        if let Some(data) = &relationships.tracks.data {
            for resource_id in data {
                process_track(client, &resource_id.id, 2).await;
            }
        }

        // Process playlists
        if let Some(data) = &relationships.playlists.data {
            for resource_id in data {
                process_playlist(client, &resource_id.id, 2).await;
            }
        }

        // Process videos
        if let Some(data) = &relationships.videos.data {
            for resource_id in data {
                process_video(client, &resource_id.id, 2).await;
            }
        }

        // Process top hits
        if let Some(data) = &relationships.top_hits.data {
            for resource_id in data {
                // Process based on type
                match resource_id.r#type {
                    Albums => process_album(client, &resource_id.id, 2).await,
                    Artists => process_artist(client, &resource_id.id, 2).await,
                    Tracks => process_track(client, &resource_id.id, 2).await,
                    Videos => process_video(client, &resource_id.id, 2).await,
                    Playlists => process_playlist(client, &resource_id.id, 2).await,
                    _ => {
                        panic!("Unknown resource type: {}", resource_id.r#type);
                    }
                }
            }
        }
    }

    info!("Simple resource walking finished");
}

#[async_recursion]
async fn process_album(client: &tidalv2::client::TidalClient, album_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this album
    if is_resource_processed(Albums, album_id) {
        trace!("Skipping already processed album: {}", album_id);
        return;
    }

    // Mark this album as being processed
    mark_resource_processed(Albums, album_id);

    trace!("Loading album: {}", album_id);
    increment_request_count();

    let result = client
        .album_get(
            album_id,
            Some(vec![Artists.to_string(), "items".to_string()]),
        )
        .await;

    match result {
        Ok(album_response) => {
            if recurse > 0 {
                for resource_id in album_response
                    .data
                    .relationships
                    .unwrap()
                    .items
                    .data
                    .unwrap()
                {
                    match resource_id.r#type {
                        Tracks => process_track(client, &resource_id.id, recurse - 1).await,
                        Albums => process_album(client, &resource_id.id, recurse - 1).await,
                        Artists => process_artist(client, &resource_id.id, recurse - 1).await,
                        Videos => process_video(client, &resource_id.id, recurse - 1).await,
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
async fn process_artist(client: &tidalv2::client::TidalClient, artist_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this artist
    if is_resource_processed(Artists, artist_id) {
        trace!("Skipping already processed artist: {}", artist_id);
        return;
    }

    // Mark this artist as being processed
    mark_resource_processed(Artists, artist_id);

    trace!("Loading artist: {}", artist_id);
    increment_request_count();

    let result = client
        .artist_get(
            artist_id,
            Some(vec![Albums.to_string(), Tracks.to_string()]),
            Some("FINGERPRINT".to_string()),
        )
        .await;

    match result {
        Ok(artist_response) => {
            if recurse > 0 {
                if let Some(relationships) = &artist_response.data.relationships {
                    // Process albums
                    if let Some(albums_data) = &relationships.albums.data {
                        for resource_id in albums_data {
                            process_album(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process tracks
                    if let Some(tracks_data) = &relationships.tracks.data {
                        for resource_id in tracks_data {
                            process_track(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process videos
                    if let Some(videos_data) = &relationships.videos.data {
                        for resource_id in videos_data {
                            process_video(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process similar artists
                    if let Some(similar_artists_data) = &relationships.similar_artists.data {
                        for resource_id in similar_artists_data {
                            process_artist(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process owners (other artists that own this artist's content)
                    if let Some(owners_data) = &relationships.owners.data {
                        for resource_id in owners_data {
                            match resource_id.r#type {
                                Artists => {
                                    process_artist(client, &resource_id.id, recurse - 1).await
                                }
                                _ => trace!("Skipping owner resource type: {}", resource_id.r#type),
                            }
                        }
                    }

                    // Process profile art (artwork resources)
                    if let Some(profile_art_data) = &relationships.profile_art.data {
                        for resource_id in profile_art_data {
                            process_artwork(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process radio (radio station resources)
                    if let Some(radio_data) = &relationships.radio.data {
                        for resource_id in radio_data {
                            process_radio(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process roles (role resources)
                    if let Some(roles_data) = &relationships.roles.data {
                        for resource_id in roles_data {
                            process_role(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process track providers
                    if let Some(track_providers_data) = &relationships.track_providers.data {
                        for resource_id in track_providers_data {
                            process_provider(client, &resource_id.id, recurse - 1).await;
                        }
                    }

                    // Process biography (single Relationship)
                    if let Some(_biography_data) = &relationships.biography.data {
                        // Note: Biography API takes artist_id, not biography resource id
                        // We'll use the current artist_id instead of biography_data.id
                        process_biography(client, artist_id, recurse - 1).await;
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
async fn process_track(client: &tidalv2::client::TidalClient, track_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this track
    if is_resource_processed(Tracks, track_id) {
        trace!("Skipping already processed track: {}", track_id);
        return;
    }

    // Mark this track as being processed
    mark_resource_processed(Tracks, track_id);

    trace!("Loading track: {}", track_id);
    increment_request_count();

    let result = client
        .track_get(
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
                            process_artist(client, &resource_id.id, recurse - 1).await;
                        }
                    }
                    if let Some(albums_data) = &relationships.albums.data {
                        for resource_id in albums_data {
                            process_album(client, &resource_id.id, recurse - 1).await;
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
async fn process_playlist(
    client: &tidalv2::client::TidalClient,
    playlist_id: &str,
    recurse: usize,
) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this playlist
    if is_resource_processed(Playlists, playlist_id) {
        trace!("Skipping already processed playlist: {}", playlist_id);
        return;
    }

    // Mark this playlist as being processed
    mark_resource_processed(Playlists, playlist_id);

    trace!("Loading playlist: {}", playlist_id);
    increment_request_count();

    let result = client
        .playlist_get(playlist_id, Some(vec!["items".to_string()]))
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
                                Tracks => process_track(client, &resource_id.id, recurse - 1).await,
                                Albums => process_album(client, &resource_id.id, recurse - 1).await,
                                Artists => {
                                    process_artist(client, &resource_id.id, recurse - 1).await
                                }
                                Videos => process_video(client, &resource_id.id, recurse - 1).await,
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
async fn process_video(client: &tidalv2::client::TidalClient, video_id: &str, recurse: usize) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this video
    if is_resource_processed(Videos, video_id) {
        trace!("Skipping already processed video: {}", video_id);
        return;
    }

    // Mark this video as being processed
    mark_resource_processed(Videos, video_id);

    trace!("Loading video: {}", video_id);
    increment_request_count();
    let result = client
        .video_get(
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
                            process_artist(client, &resource_id.id, recurse - 1).await;
                        }
                    }
                    if let Some(albums_data) = &relationships.albums.data {
                        for resource_id in albums_data {
                            process_album(client, &resource_id.id, recurse - 1).await;
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

#[async_recursion]
async fn process_artwork(client: &tidalv2::client::TidalClient, artwork_id: &str, _recurse: usize) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this artwork
    if is_resource_processed(Artworks, artwork_id) {
        trace!("Skipping already processed artwork: {}", artwork_id);
        return;
    }

    // Mark this artwork as being processed
    mark_resource_processed(Artworks, artwork_id);

    trace!("Loading artwork: {}", artwork_id);
    increment_request_count();

    let result = client.artwork_get(artwork_id, None).await;

    match result {
        Ok(_artwork_response) => {
            trace!("✓ Artwork loaded: {}", artwork_id);
            // Artworks typically don't have deep relationships to recurse into
        }
        Err(e) => {
            panic!("Failed to load artwork {}: {:?}", artwork_id, e);
        }
    }
}

#[async_recursion]
async fn process_biography(
    client: &tidalv2::client::TidalClient,
    artist_id: &str,
    _recurse: usize,
) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this artist's biography
    if is_resource_processed(ArtistBiographies, artist_id) {
        trace!(
            "Skipping already processed biography for artist: {}",
            artist_id
        );
        return;
    }

    // Mark this artist's biography as being processed
    mark_resource_processed(ArtistBiographies, artist_id);

    trace!("Loading biography for artist: {}", artist_id);
    increment_request_count();

    let result = client.artist_biography(artist_id).await;

    match result {
        Ok(_biography_response) => {
            trace!("✓ Biography loaded for artist: {}", artist_id);
            // Biographies typically don't have relationships to recurse into
        }
        Err(e) => {
            panic!("Failed to load biography for artist {}: {:?}", artist_id, e);
        }
    }
}

#[async_recursion]
async fn process_role(client: &tidalv2::client::TidalClient, role_id: &str, _recurse: usize) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this role
    if is_resource_processed(ArtistRoles, role_id) {
        trace!("Skipping already processed role: {}", role_id);
        return;
    }

    // Mark this role as being processed
    mark_resource_processed(ArtistRoles, role_id);

    trace!("Loading role: {}", role_id);
    increment_request_count();

    let result = client.artist_role_get(role_id).await;

    match result {
        Ok(_role_response) => {
            trace!("✓ Role loaded: {}", role_id);
            // Roles typically don't have deep relationships to recurse into
        }
        Err(e) => {
            panic!("Failed to load role {}: {:?}", role_id, e);
        }
    }
}

#[async_recursion]
async fn process_provider(
    client: &tidalv2::client::TidalClient,
    provider_id: &str,
    _recurse: usize,
) {
    if !can_make_request() {
        return;
    }

    // Check if we've already processed this provider
    if is_resource_processed(Providers, provider_id) {
        trace!("Skipping already processed provider: {}", provider_id);
        return;
    }

    // Mark this provider as being processed
    mark_resource_processed(Providers, provider_id);

    trace!("Loading provider: {}", provider_id);
    increment_request_count();

    let result = client.provider_get(provider_id).await;

    match result {
        Ok(_provider_response) => {
            trace!("✓ Provider loaded: {}", provider_id);
            // Providers typically don't have deep relationships to recurse into
        }
        Err(e) => {
            panic!("Failed to load provider {}: {:?}", provider_id, e);
        }
    }
}

#[async_recursion]
async fn process_radio(_client: &tidalv2::client::TidalClient, radio_id: &str, _recurse: usize) {
    if !can_make_request() {
        return;
    }

    // For radio resources, we'll use a generic tracking approach since there's no specific ResourceType
    // We'll use SearchResults as a placeholder ResourceType for radio resources
    if is_resource_processed(SearchResults, &format!("radio_{}", radio_id)) {
        trace!("Skipping already processed radio: {}", radio_id);
        return;
    }

    mark_resource_processed(SearchResults, &format!("radio_{}", radio_id));

    trace!("Loading radio: {}", radio_id);
    // Note: There doesn't seem to be a dedicated radio API, so we'll just log for now
    // This could be expanded if a radio API becomes available
    trace!(
        "✓ Radio resource noted: {} (no specific API available)",
        radio_id
    );
}

#[tokio::test]
#[ignore]
async fn test_user_collections_and_walk() {
    reset_request_count();
    // Initialize logging for HTTP request/response debugging
    init_logging_once();

    // Get required environment variables
    let client_id =
        env::var("TIDAL_CLIENT_ID").expect("TIDAL_CLIENT_ID environment variable must be set");

    let refresh_token = env::var("TIDAL_REFRESH_TOKEN")
        .expect("TIDAL_REFRESH_TOKEN environment variable must be set");

    // Get optional access token (client will generate one if not provided)
    let access_token = env::var("TIDAL_ACCESS_TOKEN").unwrap_or_else(|_| String::new()); // Empty string if not provided

    // Configure API client with authentication
    let authz = tidalv2::client::Authz::new(
        access_token,
        refresh_token,
        0, // user_id will be updated when we get user info
        Some("US".to_string()),
        Some(u64::MAX), // expires_timestamp - set to far future for testing
    );

    let mut client = tidalv2::client::TidalClient::new(client_id).with_authz(authz);

    client.set_country_code("US".to_string());

    info!("Starting user collections integration test");

    // First, get the current user to obtain user ID
    if !can_make_request() {
        return;
    }

    increment_request_count();
    let user_result = client.user_me().await;

    match user_result {
        Ok(user_response) => {
            let user_id = &user_response.data.id;
            info!("✓ Current user ID: {}", user_id);

            // Walk user collections for different resource types
            walk_user_collections(&client, user_id).await;
        }
        Err(e) => {
            panic!("Failed to get current user: {:?}", e);
        }
    }
}

/// Walk through user collections for different resource types
async fn walk_user_collections(client: &tidalv2::client::TidalClient, user_id: &str) {
    info!("Starting user collections walking for user: {}", user_id);

    // Process user's playlist collection
    if can_make_request() {
        info!("Processing user playlist collection...");
        increment_request_count();

        let playlists_result = client
            .user_collection_playlists(
                user_id, None, // page_cursor
                None, // sort
            )
            .await;

        match playlists_result {
            Ok(playlists_response) => {
                if let Some(ref playlists_data) = playlists_response.data {
                    info!(
                        "✓ Found {} playlists in user collection",
                        playlists_data.len()
                    );

                    // Walk through each playlist
                    for playlist_resource in playlists_data {
                        process_playlist(client, &playlist_resource.id, 2).await;
                    }
                } else {
                    info!("✓ No playlists found in user collection");
                }
            }
            Err(e) => {
                panic!("Could not fetch user playlist collection: {:?}", e);
            }
        }
    }

    // Process user's album collection
    if can_make_request() {
        info!("Processing user album collection...");
        increment_request_count();

        let albums_result = client
            .user_collection_albums(
                user_id, "US", // locale
                None, // page_cursor
                None, // sort
            )
            .await;

        match albums_result {
            Ok(albums_response) => {
                if let Some(ref albums_data) = albums_response.data {
                    info!("✓ Found {} albums in user collection", albums_data.len());

                    // Walk through each album
                    for album_resource in albums_data {
                        process_album(client, &album_resource.id, 2).await;
                    }
                } else {
                    info!("✓ No albums found in user collection");
                }
            }
            Err(e) => {
                panic!("Could not fetch user album collection: {:?}", e);
            }
        }
    }

    // Process user's artist collection
    if can_make_request() {
        info!("Processing user artist collection...");
        increment_request_count();

        let artists_result = client
            .user_collection_artists(
                user_id, "US", // locale
                None, // page_cursor
                None, // sort
            )
            .await;

        match artists_result {
            Ok(artists_response) => {
                if let Some(ref artists_data) = artists_response.data {
                    info!("✓ Found {} artists in user collection", artists_data.len());

                    // Walk through each artist
                    for artist_resource in artists_data {
                        process_artist(client, &artist_resource.id, 2).await;
                    }
                } else {
                    info!("✓ No artists found in user collection");
                }
            }
            Err(e) => {
                panic!("Could not fetch user artist collection: {:?}", e);
            }
        }
    }

    // Process user's track collection
    if can_make_request() {
        info!("Processing user track collection...");
        increment_request_count();

        let tracks_result = client
            .user_collection_tracks(
                user_id, "US", // locale
                None, // page_cursor
                None, // sort
            )
            .await;

        match tracks_result {
            Ok(tracks_response) => {
                if let Some(ref tracks_data) = tracks_response.data {
                    info!("✓ Found {} tracks in user collection", tracks_data.len());

                    // Walk through each track
                    for track_resource in tracks_data {
                        process_track(client, &track_resource.id, 2).await;
                    }
                } else {
                    info!("✓ No tracks found in user collection");
                }
            }
            Err(e) => {
                panic!("Could not fetch user track collection: {:?}", e);
            }
        }
    }

    // Process user's video collection
    if can_make_request() {
        info!("Processing user video collection...");
        increment_request_count();

        let videos_result = client
            .user_collection_videos(
                user_id, "US", // locale
                None, // page_cursor
                None, // sort
            )
            .await;

        match videos_result {
            Ok(videos_response) => {
                if let Some(ref videos_data) = videos_response.data {
                    info!("✓ Found {} videos in user collection", videos_data.len());

                    // Walk through each video
                    for video_resource in videos_data {
                        process_video(client, &video_resource.id, 2).await;
                    }
                } else {
                    info!("✓ No videos found in user collection");
                }
            }
            Err(e) => {
                panic!("Could not fetch user video collection: {:?}", e);
            }
        }
    }

    info!("User collections walking completed");
}
