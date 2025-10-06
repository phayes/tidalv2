use log::trace;
use std::collections::HashMap;
use std::fs;
use tidalv2::models::ResourceType::*;

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
    let search_result =
        serde_json::from_str::<models::Resource<models::search_result::SearchResult>>(&json_content);
    assert!(
        search_result.is_ok(),
        "Failed to deserialize search result: {:?}",
        search_result.err()
    );

    let search_response = search_result.unwrap();

    // Verify basic structure
    assert_eq!(search_response.data.id, "taylor+swift");
    // Note: The type field is consumed by the enum discriminator, so it will be empty
    // assert_eq!(search_response.data.r#type, "searchResults");

    // Verify attributes exist
    let attributes = &search_response.data.attributes;
    assert_eq!(attributes.did_you_mean.as_ref().unwrap(), "taylor swift");

    // Verify relationships exist
    assert!(search_response.data.relationships.is_some());
    let relationships = search_response.data.relationships.as_ref().unwrap();

    // Verify that albums relationship has data
    if let Some(albums_data) = &relationships.albums.data {
        assert!(!albums_data.is_empty(), "Albums data should not be empty");
        trace!("Found {} albums in relationships", albums_data.len());
    }

    // Verify that artists relationship has data
    if let Some(artists_data) = &relationships.artists.data {
        assert!(!artists_data.is_empty(), "Artists data should not be empty");
        trace!("Found {} artists in relationships", artists_data.len());
    }

    // Most importantly, verify that the included array deserializes correctly
    assert!(
        !search_response.included.is_empty(),
        "Included array should not be empty"
    );
    let included = &search_response.included;

    trace!(
        "Successfully deserialized search result with {} included resources",
        included.len()
    );

    // Count different types of included resources
    let mut type_counts = HashMap::new();
    for item in included {
        *type_counts.entry(item.resource_type()).or_insert(0) += 1;
    }

    trace!("Included resource type breakdown:");
    for (resource_type, count) in &type_counts {
        trace!("  {}: {}", resource_type, count);
    }

    // Verify we have the expected resource types
    assert!(type_counts.contains_key(&Tracks), "Should contain tracks");
    assert!(type_counts.contains_key(&Artists), "Should contain artists");
    assert!(type_counts.contains_key(&Albums), "Should contain albums");
    assert!(
        type_counts.contains_key(&Playlists),
        "Should contain playlists"
    );
    assert!(type_counts.contains_key(&Videos), "Should contain videos");
}

#[tokio::test]
async fn test_album_json_deserialization() {
    // Initialize logging for this test
    let _ = env_logger::try_init();

    // Test deserialization of the test_album.json file
    use tidalv2::models::*;

    // Read the test JSON file
    let json_content =
        fs::read_to_string("tests/test_album.json").expect("Failed to read test_album.json file");

    // Test album deserialization
    let album_result = serde_json::from_str::<album::Album>(&json_content);
    assert!(
        album_result.is_ok(),
        "Failed to deserialize album: {:?}",
        album_result.err()
    );

    let album = album_result.unwrap();

    // Verify basic structure
    assert_eq!(album.id, "12345");
    assert_eq!(album.r#type, "albums");

    // Verify attributes exist and have expected values
    let attributes = &album.attributes;
    assert_eq!(attributes.title, "4:44");
    assert_eq!(attributes.barcode_id, "00854242007552");
    assert_eq!(attributes.duration, "PT46M17S");
    assert!(attributes.explicit);
    assert_eq!(attributes.number_of_items, 13);
    assert_eq!(attributes.number_of_volumes, 1);
    assert_eq!(attributes.popularity, 0.56);
    assert_eq!(attributes.release_date.as_ref().unwrap(), "2017-06-30");
    assert_eq!(attributes.version.as_ref().unwrap(), "remix");

    // Verify availability
    assert!(attributes.availability.is_some());
    let availability = attributes.availability.as_ref().unwrap();
    assert_eq!(availability.len(), 1);

    // Verify media tags
    assert_eq!(attributes.media_tags.len(), 1);
    assert_eq!(attributes.media_tags[0], "HIRES_LOSSLESS, LOSSLESS");

    // Verify external links
    assert!(attributes.external_links.is_some());
    let external_links = attributes.external_links.as_ref().unwrap();
    assert_eq!(external_links.len(), 1);

    // Verify relationships exist
    assert!(album.relationships.is_some());
    let relationships = album.relationships.as_ref().unwrap();

    // Verify that various relationships have data
    if let Some(artists_data) = &relationships.artists.data {
        assert!(!artists_data.is_empty(), "Artists data should not be empty");
        trace!(
            "Found {} artists in album relationships",
            artists_data.len()
        );
    }

    if let Some(items_data) = &relationships.items.data {
        assert!(!items_data.is_empty(), "Items data should not be empty");
        trace!("Found {} items in album relationships", items_data.len());

        // Verify the first item has meta information
        if let Some(meta) = &items_data[0].meta {
            // Check that we can access track and volume numbers
            trace!("First item meta: {:?}", meta);
        }
    }

    trace!("Successfully deserialized album: {}", attributes.title);
}

#[tokio::test]
async fn test_artist_json_deserialization() {
    // Initialize logging for this test
    let _ = env_logger::try_init();

    // Test deserialization of the test_artist.json file
    use tidalv2::models::*;

    // Read the test JSON file
    let json_content =
        fs::read_to_string("tests/test_artist.json").expect("Failed to read test_artist.json file");

    // Test artist deserialization
    let artist_result = serde_json::from_str::<artist::Artist>(&json_content);
    assert!(
        artist_result.is_ok(),
        "Failed to deserialize artist: {:?}",
        artist_result.err()
    );

    let artist = artist_result.unwrap();

    // Verify basic structure
    assert_eq!(artist.id, "12345");
    assert_eq!(artist.r#type, "artists");

    // Verify attributes exist and have expected values
    let attributes = &artist.attributes;
    assert_eq!(attributes.name, "JAY Z");
    assert_eq!(attributes.handle.as_ref().unwrap(), "jayz");
    assert_eq!(attributes.popularity, 0.56);
    assert!(attributes.contributions_enabled.unwrap());
    assert!(attributes.spotlighted.unwrap());
    assert_eq!(
        attributes.contributions_sales_pitch.as_ref().unwrap(),
        "Help me be a full time artist"
    );

    // Verify external links
    assert!(attributes.external_links.is_some());
    let external_links = attributes.external_links.as_ref().unwrap();
    assert_eq!(external_links.len(), 1);

    // Verify relationships exist
    assert!(artist.relationships.is_some());
    let relationships = artist.relationships.as_ref().unwrap();

    // Verify that various relationships have data
    if let Some(albums_data) = &relationships.albums.data {
        assert!(!albums_data.is_empty(), "Albums data should not be empty");
        trace!("Found {} albums in artist relationships", albums_data.len());
    }

    if let Some(tracks_data) = &relationships.tracks.data {
        assert!(!tracks_data.is_empty(), "Tracks data should not be empty");
        trace!("Found {} tracks in artist relationships", tracks_data.len());
    }

    // Verify similar artists relationship
    if let Some(similar_artists_data) = &relationships.similar_artists.data {
        assert!(
            !similar_artists_data.is_empty(),
            "Similar artists data should not be empty"
        );
        trace!(
            "Found {} similar artists in artist relationships",
            similar_artists_data.len()
        );

        // Verify the first similar artist has basic information
        trace!("First similar artist id: {}", similar_artists_data[0].id);
        trace!(
            "First similar artist type: {}",
            similar_artists_data[0].r#type
        );
    }

    // Verify track providers relationship with meta information
    if let Some(track_providers_data) = &relationships.track_providers.data {
        assert!(
            !track_providers_data.is_empty(),
            "Track providers data should not be empty"
        );
        trace!(
            "Found {} track providers in artist relationships",
            track_providers_data.len()
        );

        // Verify the first track provider has meta information
        if let Some(meta) = &track_providers_data[0].meta {
            trace!("First track provider meta: {:?}", meta);
        }
    }

    trace!("Successfully deserialized artist: {}", attributes.name);
}

#[test]
fn test_included_vec_serialization() {
    use tidalv2::models::{Links, Resource};
    
    // Test with empty included vec - should not serialize "included" field
    let resource_empty = Resource {
        data: "test".to_string(),
        included: Vec::new(),
        links: Links::new("http://example.com/self".to_string()),
    };
    
    let json_empty = serde_json::to_string(&resource_empty).unwrap();
    assert!(!json_empty.contains("included"), "Empty included vec should not be serialized");
    
    // Test deserialization without "included" field - should default to empty vec
    let json_no_included = r#"{"data":"test","links":{"self":"http://example.com/self"}}"#;
    let deserialized: Resource<String> = serde_json::from_str(json_no_included).unwrap();
    assert!(deserialized.included.is_empty(), "Missing included field should deserialize to empty vec");
    assert_eq!(deserialized.data, "test");
    
    // Test deserialization with explicit empty array
    let json_empty_array = r#"{"data":"test","included":[],"links":{"self":"http://example.com/self"}}"#;
    let deserialized2: Resource<String> = serde_json::from_str(json_empty_array).unwrap();
    assert!(deserialized2.included.is_empty(), "Empty included array should deserialize to empty vec");
}
