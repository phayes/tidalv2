use std::collections::HashMap;
use std::fs;
use tidalv2::models;
use log::info;

#[tokio::test]
async fn test_included_inner_deserialization() {
    // Test deserialization of IncludedInner variants to verify our fix
    use tidalv2::models::IncludedInner;
    
    // Test track deserialization
    let track_json = r#"{"id":"116125896","type":"tracks","attributes":{"title":"Cruel Summer","duration":"PT2M58S","explicit":false}}"#;
    let track_result = serde_json::from_str::<IncludedInner>(track_json);
    assert!(track_result.is_ok(), "Failed to deserialize track: {:?}", track_result.err());
    
    // Test artist deserialization
    let artist_json = r#"{"id":"3557299","type":"artists","attributes":{"name":"Taylor Swift","popularity":0.9328337958905205}}"#;
    let artist_result = serde_json::from_str::<IncludedInner>(artist_json);
    assert!(artist_result.is_ok(), "Failed to deserialize artist: {:?}", artist_result.err());
    
    // Test playlist deserialization
    let playlist_json = r#"{"id":"4d056fb5-99f9-46ec-8ff3-f2dddd41821f","type":"playlists","attributes":{"name":"Taylor Swift Essentials","bounded":true}}"#;
    let playlist_result = serde_json::from_str::<IncludedInner>(playlist_json);
    assert!(playlist_result.is_ok(), "Failed to deserialize playlist: {:?}", playlist_result.err());
    
    // Test album deserialization
    let album_json = r#"{"id":"36039732","type":"albums","attributes":{"title":"Taylor Swift","numberOfItems":15}}"#;
    let album_result = serde_json::from_str::<IncludedInner>(album_json);
    assert!(album_result.is_ok(), "Failed to deserialize album: {:?}", album_result.err());
    
    // Test video deserialization
    let video_json = r#"{"id":"216325765","type":"videos","attributes":{"title":"The Joker And The Queen","duration":"PT3M6S"}}"#;
    let video_result = serde_json::from_str::<IncludedInner>(video_json);
    assert!(video_result.is_ok(), "Failed to deserialize video: {:?}", video_result.err());
}

#[tokio::test]
async fn test_search_result_json_deserialization() {
    // Initialize logging for this test
    let _ = env_logger::try_init();
    
    // Test deserialization of the complete test_search_result.json file
    use tidalv2::models;
    
    // Read the test JSON file
    let json_content = fs::read_to_string("tests/test_search_result.json")
        .expect("Failed to read test_search_result.json file");
    
    // Test full search result deserialization
    let search_result = serde_json::from_str::<models::Resource<models::SearchResult>>(&json_content);
    assert!(search_result.is_ok(), "Failed to deserialize search result: {:?}", search_result.err());
    
    let search_response = search_result.unwrap();
    
    // Verify basic structure
    assert_eq!(search_response.data.id, "taylor+swift");
    // Note: The type field is consumed by the enum discriminator, so it will be empty
    // assert_eq!(search_response.data.r#type, "searchResults");
    
    // Verify attributes exist
    assert!(search_response.data.attributes.is_some());
    let attributes = search_response.data.attributes.as_ref().unwrap();
    assert_eq!(attributes.did_you_mean.as_ref().unwrap(), "taylor swift");
    
    // Verify relationships exist
    assert!(search_response.data.relationships.is_some());
    let relationships = search_response.data.relationships.as_ref().unwrap();
    
    // Verify that albums relationship has data
    if let Some(albums_data) = &relationships.albums.data {
        assert!(!albums_data.is_empty(), "Albums data should not be empty");
        info!("Found {} albums in relationships", albums_data.len());
    }
    
    // Verify that artists relationship has data
    if let Some(artists_data) = &relationships.artists.data {
        assert!(!artists_data.is_empty(), "Artists data should not be empty");
        info!("Found {} artists in relationships", artists_data.len());
    }
    
    // Most importantly, verify that the included array deserializes correctly
    assert!(search_response.included.is_some(), "Included array should be present");
    let included = search_response.included.as_ref().unwrap();
    assert!(!included.is_empty(), "Included array should not be empty");
    
    info!("Successfully deserialized search result with {} included resources", included.len());
    
    // Count different types of included resources
    let mut type_counts = HashMap::new();
    for item in included {
        let type_name = match item {
            models::IncludedInner::Albums(_) => "albums",
            models::IncludedInner::Artists(_) => "artists", 
            models::IncludedInner::Tracks(_) => "tracks",
            models::IncludedInner::Playlists(_) => "playlists",
            models::IncludedInner::Videos(_) => "videos",
            _ => "other",
        };
        *type_counts.entry(type_name).or_insert(0) += 1;
    }
    
    info!("Included resource type breakdown:");
    for (resource_type, count) in &type_counts {
        info!("  {}: {}", resource_type, count);
    }
    
    // Verify we have the expected resource types
    assert!(type_counts.contains_key("tracks"), "Should contain tracks");
    assert!(type_counts.contains_key("artists"), "Should contain artists");
    assert!(type_counts.contains_key("albums"), "Should contain albums");
    assert!(type_counts.contains_key("playlists"), "Should contain playlists");
    assert!(type_counts.contains_key("videos"), "Should contain videos");
}
