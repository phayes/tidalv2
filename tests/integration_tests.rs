use std::env;
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use tidalv2::{apis, models};
use log::{info, debug, warn};

/// Maximum total number of API requests allowed across all tests
const MAX_TOTAL_REQUESTS: usize = 100;

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
    TOTAL_REQUESTS.load(Ordering::Relaxed) < MAX_TOTAL_REQUESTS
}

/// Increment the global request counter and return the new count
fn increment_request_count() -> usize {
    TOTAL_REQUESTS.fetch_add(1, Ordering::Relaxed) + 1
}

/// Get the current total request count
fn get_request_count() -> usize {
    TOTAL_REQUESTS.load(Ordering::Relaxed)
}

/// Reset the global request counter (useful for testing)
#[allow(dead_code)]
fn reset_request_count() {
    TOTAL_REQUESTS.store(0, Ordering::Relaxed);
}

#[tokio::test]
async fn test_search_and_walk_resources() {
    // Initialize logging for HTTP request/response debugging
    tidalv2::init_logging();
    
    // Get bearer token from environment
    let bearer_token = env::var("TIDAL_BEARER_ACCESS_TOKEN")
        .expect("TIDAL_BEARER_ACCESS_TOKEN environment variable must be set");
    
    // Configure API client
    let mut config = apis::configuration::Configuration::new();
    config.bearer_access_token = Some(bearer_token);
    config.country_code = "US".to_string();
    
    info!("Starting integration test: search and walk resources");
    
    // Perform search for a popular query
    let search_query = "taylor swift";
    info!("Performing search for: {}", search_query);
    
    // Check global request limit before making search request
    if !can_make_request() {
        panic!("Cannot make search request: global request limit of {} exceeded", MAX_TOTAL_REQUESTS);
    }
    
    let request_count = increment_request_count();
    info!("Making search request ({}/{})", request_count, MAX_TOTAL_REQUESTS);
    
    let search_result = apis::search_results_api::search_results_id_get(
        &config,
        search_query,
        None, // explicit_filter
        Some(vec![
            "albums".to_string(),
            "artists".to_string(), 
            "tracks".to_string(),
            "playlists".to_string(),
            "videos".to_string(),
            "topHits".to_string(),
        ]),
    ).await;
    
    match search_result {
        Ok(search_response) => {
            info!("Search successful! Processing search result with ID: {}", search_response.data.id);
            
            if let Some(attributes) = &search_response.data.attributes {
                info!("Search tracking ID: {}", attributes.tracking_id);
                if let Some(did_you_mean) = &attributes.did_you_mean {
                    info!("Did you mean: {}", did_you_mean);
                }
            }
            
            // Walk through search result relationships
            let mut walker = ResourceWalker::new(config, 100);
            walker.walk_search_result(&search_response).await;
            
            info!("Resource walking completed. Total resources processed: {}", walker.processed_count);
            info!("Resource type breakdown:");
            for (resource_type, count) in walker.resource_type_counts.iter() {
                info!("  {}: {}", resource_type, count);
            }
        },
        Err(e) => {
            panic!("Search failed: {:?}", e);
        }
    }
}

#[tokio::test] 
async fn test_search_different_queries() {
    // Initialize logging
    tidalv2::init_logging();
    
    // Get bearer token from environment
    let bearer_token = env::var("TIDAL_BEARER_ACCESS_TOKEN")
        .expect("TIDAL_BEARER_ACCESS_TOKEN environment variable must be set");
    
    // Configure API client
    let mut config = apis::configuration::Configuration::new();
    config.bearer_access_token = Some(bearer_token);
    config.country_code = "US".to_string();
    
    let search_queries = vec![
        "the beatles",
        "jazz",
        "rock",
        "classical music",
        "hip hop"
    ];
    
    for query in search_queries {
        info!("Testing search query: {}", query);
        
        // Check global request limit before each search
        if !can_make_request() {
            info!("Stopping test: global request limit of {} exceeded", MAX_TOTAL_REQUESTS);
            break;
        }
        
        let request_count = increment_request_count();
        info!("Making search request for '{}' ({}/{})", query, request_count, MAX_TOTAL_REQUESTS);
        
        let search_result = apis::search_results_api::search_results_id_get(
            &config,
            query,
            None,
            Some(vec!["albums".to_string(), "artists".to_string(), "tracks".to_string()]),
        ).await;
        
        match search_result {
            Ok(response) => {
                info!("✓ Search '{}' successful, ID: {}", query, response.data.id);
                
                // Quick validation of response structure
                if let Some(_relationships) = &response.data.relationships {
                    debug!("  Found relationships for albums, artists, tracks, etc.");
                }
                
                if let Some(included) = &response.included {
                    info!("  Included {} additional resources", included.len());
                }
            },
            Err(e) => {
                warn!("✗ Search '{}' failed: {:?}", query, e);
            }
        }
    }
    
    info!("Multiple search queries test completed. Total API requests made: {}/{}", get_request_count(), MAX_TOTAL_REQUESTS);
}

/// Resource walker that traverses search results and loads related resources
struct ResourceWalker {
    config: apis::configuration::Configuration,
    processed_count: usize,
    max_resources: usize,
    processed_ids: HashSet<String>,
    resource_queue: VecDeque<ResourceRef>,
    resource_type_counts: std::collections::HashMap<String, usize>,
}

#[derive(Debug, Clone)]
struct ResourceRef {
    id: String,
    resource_type: String,
}

impl ResourceWalker {
    fn new(config: apis::configuration::Configuration, max_resources: usize) -> Self {
        Self {
            config,
            processed_count: 0,
            max_resources,
            processed_ids: HashSet::new(),
            resource_queue: VecDeque::new(),
            resource_type_counts: std::collections::HashMap::new(),
        }
    }
    
    async fn walk_search_result(&mut self, search_response: &models::Resource<models::SearchResult>) {
        info!("Starting resource walking from search result");
        
        // Process included resources first
        if let Some(included) = &search_response.included {
            for included_resource in included {
                self.queue_resource_from_included(included_resource);
            }
        }
        
        // Add relationships to queue
        if let Some(relationships) = &search_response.data.relationships {
            self.queue_relationships(relationships);
        }
        
        // Process queue
        while !self.resource_queue.is_empty() && self.processed_count < self.max_resources {
            // Check global request limit before processing each resource
            if !can_make_request() {
                info!("Stopping resource walking: global request limit of {} exceeded", MAX_TOTAL_REQUESTS);
                break;
            }
            
            if let Some(resource_ref) = self.resource_queue.pop_front() {
                self.process_resource(resource_ref).await;
            }
        }
        
        info!("Resource walking finished. Processed {} resources", self.processed_count);
    }
    
    fn queue_resource_from_included(&mut self, included: &models::IncludedInner) {
        // Extract resource info from included resource
        // Note: IncludedInner is an enum, we need to handle different variants
        debug!("Processing included resource: {:?}", included);
        
        // For now, we'll extract what we can from the included resources
        // The actual implementation would depend on the IncludedInner enum structure
    }
    
    fn queue_relationships(&mut self, relationships: &models::SearchResultsRelationships) {
        // Queue albums
        self.queue_multi_relationship_resources(&relationships.albums, "albums");
        
        // Queue artists  
        self.queue_multi_relationship_resources(&relationships.artists, "artists");
        
        // Queue tracks
        self.queue_multi_relationship_resources(&relationships.tracks, "tracks");
        
        // Queue playlists
        self.queue_multi_relationship_resources(&relationships.playlists, "playlists");
        
        // Queue videos
        self.queue_multi_relationship_resources(&relationships.videos, "videos");
        
        // Queue top hits
        self.queue_multi_relationship_resources(&relationships.top_hits, "topHits");
    }
    
    fn queue_multi_relationship_resources(&mut self, multi_rel: &models::MultiRelationship<models::ResourceIdentifier>, relationship_type: &str) {
        if let Some(data) = &multi_rel.data {
            for resource_id in data {
                let resource_ref = ResourceRef {
                    id: resource_id.id.clone(),
                    resource_type: resource_id.r#type.clone(),
                };
                
                if !self.processed_ids.contains(&resource_ref.id) {
                    debug!("Queuing {} resource: {} (type: {})", relationship_type, resource_ref.id, resource_ref.resource_type);
                    self.resource_queue.push_back(resource_ref);
                }
            }
        }
    }
    
    async fn process_resource(&mut self, resource_ref: ResourceRef) {
        if self.processed_ids.contains(&resource_ref.id) {
            return; // Already processed
        }
        
        if self.processed_count >= self.max_resources {
            return; // Hit limit
        }
        
        self.processed_ids.insert(resource_ref.id.clone());
        self.processed_count += 1;
        
        // Update resource type counts
        *self.resource_type_counts.entry(resource_ref.resource_type.clone()).or_insert(0) += 1;
        
        info!("Processing resource {}/{}: {} (type: {})", 
              self.processed_count, self.max_resources, 
              resource_ref.id, resource_ref.resource_type);
        
        // Load the actual resource based on type
        match resource_ref.resource_type.as_str() {
            "albums" => self.process_album(&resource_ref.id).await,
            "artists" => self.process_artist(&resource_ref.id).await,
            "tracks" => self.process_track(&resource_ref.id).await,
            "playlists" => self.process_playlist(&resource_ref.id).await,
            "videos" => self.process_video(&resource_ref.id).await,
            _ => {
                debug!("Unknown resource type: {}, skipping detailed processing", resource_ref.resource_type);
            }
        }
    }
    
    async fn process_album(&mut self, album_id: &str) {
        debug!("Loading album: {}", album_id);
        
        let request_count = increment_request_count();
        debug!("Making album request ({}/{})", request_count, MAX_TOTAL_REQUESTS);
        
        let result = apis::albums_api::albums_id_get(
            &self.config,
            album_id,
            Some(vec!["artists".to_string(), "items".to_string()]), // include related
        ).await;
        
        match result {
            Ok(album_response) => {
                debug!("✓ Album loaded: {}", album_id);
                
                // Queue related resources from relationships
                if let Some(relationships) = &album_response.data.relationships {
                    self.queue_multi_relationship_resources(&relationships.artists, "artists");
                    // Queue album items (tracks/videos) - need special handling for different type
                    self.queue_album_items_relationship(&relationships.items, "items");
                }
            },
            Err(e) => {
                warn!("✗ Failed to load album {}: {:?}", album_id, e);
            }
        }
    }
    
    async fn process_artist(&mut self, artist_id: &str) {
        debug!("Loading artist: {}", artist_id);
        
        let request_count = increment_request_count();
        debug!("Making artist request ({}/{})", request_count, MAX_TOTAL_REQUESTS);
        
        let result = apis::artists_api::artists_id_get(
            &self.config,
            artist_id,
            Some(vec!["albums".to_string(), "tracks".to_string()]), // include related
        ).await;
        
        match result {
            Ok(artist_response) => {
                debug!("✓ Artist loaded: {}", artist_id);
                
                // Queue related resources from relationships
                if let Some(relationships) = &artist_response.data.relationships {
                    self.queue_multi_relationship_resources(&relationships.albums, "albums");
                    self.queue_multi_relationship_resources(&relationships.tracks, "tracks");
                }
            },
            Err(e) => {
                warn!("✗ Failed to load artist {}: {:?}", artist_id, e);
            }
        }
    }
    
    async fn process_track(&mut self, track_id: &str) {
        debug!("Loading track: {}", track_id);
        
        let request_count = increment_request_count();
        debug!("Making track request ({}/{})", request_count, MAX_TOTAL_REQUESTS);
        
        let result = apis::tracks_api::tracks_id_get(
            &self.config,
            track_id,
            Some(vec!["artists".to_string(), "albums".to_string()]), // include related
        ).await;
        
        match result {
            Ok(track_response) => {
                debug!("✓ Track loaded: {}", track_id);
                
                // Queue related resources from relationships  
                if let Some(relationships) = &track_response.data.relationships {
                    self.queue_multi_relationship_resources(&relationships.artists, "artists");
                    self.queue_multi_relationship_resources(&relationships.albums, "albums");
                }
            },
            Err(e) => {
                warn!("✗ Failed to load track {}: {:?}", track_id, e);
            }
        }
    }
    
    async fn process_playlist(&mut self, playlist_id: &str) {
        debug!("Loading playlist: {}", playlist_id);
        
        let request_count = increment_request_count();
        debug!("Making playlist request ({}/{})", request_count, MAX_TOTAL_REQUESTS);
        
        let result = apis::playlists_api::playlists_id_get(
            &self.config,
            playlist_id,
            Some(vec!["items".to_string()]), // include items
        ).await;
        
        match result {
            Ok(playlist_response) => {
                debug!("✓ Playlist loaded: {}", playlist_id);
                
                // Queue related resources from relationships
                if let Some(relationships) = &playlist_response.data.relationships {
                    self.queue_playlist_items_relationship(&relationships.items, "items");
                }
            },
            Err(e) => {
                warn!("✗ Failed to load playlist {}: {:?}", playlist_id, e);
            }
        }
    }
    
    async fn process_video(&mut self, video_id: &str) {
        debug!("Loading video: {}", video_id);
        
        let request_count = increment_request_count();
        debug!("Making video request ({}/{})", request_count, MAX_TOTAL_REQUESTS);
        
        let result = apis::videos_api::videos_id_get(
            &self.config,
            video_id,
            Some(vec!["artists".to_string(), "albums".to_string()]), // include related
        ).await;
        
        match result {
            Ok(video_response) => {
                debug!("✓ Video loaded: {}", video_id);
                
                // Queue related resources from relationships
                if let Some(relationships) = &video_response.data.relationships {
                    self.queue_multi_relationship_resources(&relationships.artists, "artists");
                    self.queue_multi_relationship_resources(&relationships.albums, "albums");
                }
            },
            Err(e) => {
                warn!("✗ Failed to load video {}: {:?}", video_id, e);
            }
        }
    }
    
    fn queue_album_items_relationship(&mut self, items_rel: &models::MultiRelationship<models::AlbumsItemsResourceIdentifier>, relationship_type: &str) {
        if let Some(data) = &items_rel.data {
            for resource_id in data {
                let resource_ref = ResourceRef {
                    id: resource_id.id.clone(),
                    resource_type: resource_id.r#type.clone(),
                };
                
                if !self.processed_ids.contains(&resource_ref.id) {
                    debug!("Queuing {} resource: {} (type: {})", relationship_type, resource_ref.id, resource_ref.resource_type);
                    self.resource_queue.push_back(resource_ref);
                }
            }
        }
    }
    
    fn queue_playlist_items_relationship(&mut self, items_rel: &models::MultiRelationship<models::PlaylistsItemsResourceIdentifier>, relationship_type: &str) {
        if let Some(data) = &items_rel.data {
            for resource_id in data {
                let resource_ref = ResourceRef {
                    id: resource_id.id.clone(),
                    resource_type: resource_id.r#type.clone(),
                };
                
                if !self.processed_ids.contains(&resource_ref.id) {
                    debug!("Queuing {} resource: {} (type: {})", relationship_type, resource_ref.id, resource_ref.resource_type);
                    self.resource_queue.push_back(resource_ref);
                }
            }
        }
    }
}
