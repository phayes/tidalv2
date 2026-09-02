use super::{Error, client};
use crate::models::user_collection::*;
use crate::models::*;
use reqwest;
use serde::{Deserialize, Serialize};

impl client::TidalClient {
    /// Retrieves single userCollection by id.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `locale` - BCP47 locale code (e.g. "en-US")
    /// * `include` - Allows the client to customize which related resources should be returned. Available options: albums, artists, owners, playlists (e.g. "albums")
    pub async fn user_collection_get(
        &self,
        user_id: &str,
        locale: &str,
        include: Option<Vec<String>>,
    ) -> Result<Resource<UserCollection>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_locale = locale;
        let p_include = include;

        let uri_str = format!(
            "{}/userCollections/{id}",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        req_builder = req_builder.query(&[("locale", &p_locale.to_string())]);
        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        if let Some(ref param_value) = p_include {
            req_builder = req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("include".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            );
        }

        self.execute_request(req_builder).await
    }

    /// Deletes item(s) from albums relationship.
    pub async fn user_collections_id_relationships_albums_delete(
        &self,
        user_id: &str,
        albums_to_remove: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            albums_to_remove
                .into_iter()
                .map(|album_id: String| ResourceIdentifier::new(album_id, ResourceType::Albums))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/albums",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::DELETE, &uri_str);

        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Retrieves albums relationship.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `locale` - BCP 47 locale (e.g. "en-US")
    /// * `page_cursor` - Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified
    /// * `sort` - Values prefixed with "-" are sorted descending; values without it are sorted ascending
    pub async fn user_collection_albums(
        &self,
        user_id: &str,
        locale: &str,
        page_cursor: Option<&str>,
        sort: Option<Vec<String>>,
    ) -> Result<MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_locale = locale;
        let p_page_cursor = page_cursor;
        let p_sort = sort;

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/albums",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.query(&[("locale", &p_locale.to_string())]);
        if let Some(ref param_value) = p_page_cursor {
            req_builder = req_builder.query(&[("page[cursor]", &param_value.to_string())]);
        }
        if let Some(ref param_value) = p_sort {
            req_builder = req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("sort".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            );
        }
        req_builder = req_builder.query(&[("include", "albums")]);

        self.execute_request(req_builder).await
    }

    /// Adds item(s) to albums relationship.
    pub async fn user_collections_id_relationships_albums_post(
        &self,
        user_id: &str,
        albums_to_add: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            albums_to_add
                .into_iter()
                .map(|album_id: String| ResourceIdentifier::new(album_id, ResourceType::Albums))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/albums",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::POST, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Deletes item(s) from artists relationship.
    pub async fn user_collections_id_relationships_artists_delete(
        &self,
        user_id: &str,
        artists_to_remove: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            artists_to_remove
                .into_iter()
                .map(|artist_id: String| ResourceIdentifier::new(artist_id, ResourceType::Artists))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/artists",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::DELETE, &uri_str);

        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Retrieves artists relationship.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `locale` - BCP 47 locale (e.g. "en-US")
    /// * `page_cursor` - Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified
    /// * `sort` - Values prefixed with "-" are sorted descending; values without it are sorted ascending
    pub async fn user_collection_artists(
        &self,
        user_id: &str,
        locale: &str,
        page_cursor: Option<&str>,
        sort: Option<Vec<String>>,
    ) -> Result<MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_locale = locale;
        let p_page_cursor = page_cursor;
        let p_sort = sort;

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/artists",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.query(&[("locale", &p_locale.to_string())]);
        if let Some(ref param_value) = p_page_cursor {
            req_builder = req_builder.query(&[("page[cursor]", &param_value.to_string())]);
        }
        if let Some(ref param_value) = p_sort {
            req_builder = req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("sort".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            );
        }
        req_builder = req_builder.query(&[("include", "artists")]);

        self.execute_request(req_builder).await
    }

    /// Adds item(s) to artists relationship.
    pub async fn user_collections_id_relationships_artists_post(
        &self,
        user_id: &str,
        artists_to_add: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            artists_to_add
                .into_iter()
                .map(|artist_id: String| ResourceIdentifier::new(artist_id, ResourceType::Artists))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/artists",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::POST, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Retrieves owners relationship.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `page_cursor` - Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified
    pub async fn user_collection_owners(
        &self,
        user_id: &str,
        page_cursor: Option<&str>,
    ) -> Result<MultiRelationship<ResourceIdentifier>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_page_cursor = page_cursor;

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/owners",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        req_builder = req_builder.query(&[("include", "owners")]);
        if let Some(ref param_value) = p_page_cursor {
            req_builder = req_builder.query(&[("page[cursor]", &param_value.to_string())]);
        }

        self.execute_request(req_builder).await
    }

    /// Deletes item(s) from playlists relationship.
    pub async fn user_collections_id_relationships_playlists_delete(
        &self,
        user_id: &str,
        playlists_to_remove: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            playlists_to_remove
                .into_iter()
                .map(|playlist_id: String| {
                    ResourceIdentifier::new(playlist_id, ResourceType::Playlists)
                })
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/playlists",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::DELETE, &uri_str);

        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Retrieves playlists relationship.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `page_cursor` - Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified
    /// * `sort` - Values prefixed with "-" are sorted descending; values without it are sorted ascending
    pub async fn user_collection_playlists(
        &self,
        user_id: &str,
        page_cursor: Option<&str>,
        sort: Option<Vec<String>>,
    ) -> Result<MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_page_cursor = page_cursor;
        let p_sort = sort;

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/playlists",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        if let Some(ref param_value) = p_page_cursor {
            req_builder = req_builder.query(&[("page[cursor]", &param_value.to_string())]);
        }
        if let Some(ref param_value) = p_sort {
            req_builder = req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("sort".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            );
        }
        req_builder = req_builder.query(&[("include", "playlists")]);

        self.execute_request(req_builder).await
    }

    /// Adds item(s) to playlists relationship.
    pub async fn user_collections_id_relationships_playlists_post(
        &self,
        user_id: &str,
        playlists_to_add: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            playlists_to_add
                .into_iter()
                .map(|playlist_id: String| {
                    ResourceIdentifier::new(playlist_id, ResourceType::Playlists)
                })
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/playlists",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::POST, &uri_str);

        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Deletes item(s) from tracks relationship.
    pub async fn user_collections_id_relationships_tracks_delete(
        &self,
        user_id: &str,
        tracks_to_remove: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            tracks_to_remove
                .into_iter()
                .map(|track_id: String| ResourceIdentifier::new(track_id, ResourceType::Tracks))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/tracks",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::DELETE, &uri_str);

        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Retrieves tracks relationship.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `locale` - BCP 47 locale (e.g. "en-US")
    /// * `page_cursor` - Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified
    /// * `sort` - Values prefixed with "-" are sorted descending; values without it are sorted ascending
    pub async fn user_collection_tracks(
        &self,
        user_id: &str,
        locale: &str,
        page_cursor: Option<&str>,
        sort: Option<Vec<String>>,
    ) -> Result<MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_locale = locale;
        let p_page_cursor = page_cursor;
        let p_sort = sort;

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/tracks",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.query(&[("locale", &p_locale.to_string())]);
        if let Some(ref param_value) = p_page_cursor {
            req_builder = req_builder.query(&[("page[cursor]", &param_value.to_string())]);
        }
        if let Some(ref param_value) = p_sort {
            req_builder = req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("sort".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            );
        }
        req_builder = req_builder.query(&[("include", "tracks")]);

        self.execute_request(req_builder).await
    }

    /// Adds item(s) to tracks relationship.
    pub async fn user_collections_id_relationships_tracks_post(
        &self,
        user_id: &str,
        tracks_to_add: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            tracks_to_add
                .into_iter()
                .map(|track_id: String| ResourceIdentifier::new(track_id, ResourceType::Tracks))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/tracks",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::POST, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Deletes item(s) from videos relationship.
    pub async fn user_collections_id_relationships_videos_delete(
        &self,
        user_id: &str,
        videos_to_remove: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            videos_to_remove
                .into_iter()
                .map(|video_id: String| ResourceIdentifier::new(video_id, ResourceType::Videos))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/videos",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::DELETE, &uri_str);

        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }

    /// Retrieves videos relationship.
    ///
    /// # Parameters
    /// * `user_id` - User id (e.g. "123456")
    /// * `locale` - BCP 47 locale (e.g. "en-US")
    /// * `page_cursor` - Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified
    /// * `sort` - Values prefixed with "-" are sorted descending; values without it are sorted ascending
    pub async fn user_collection_videos(
        &self,
        user_id: &str,
        locale: &str,
        page_cursor: Option<&str>,
        sort: Option<Vec<String>>,
    ) -> Result<MultiRelationship<ResourceIdentifier<UserCollectionsResourceMeta>>, Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let p_locale = locale;
        let p_page_cursor = page_cursor;
        let p_sort = sort;

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/videos",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.query(&[("locale", &p_locale.to_string())]);
        if let Some(ref param_value) = p_page_cursor {
            req_builder = req_builder.query(&[("page[cursor]", &param_value.to_string())]);
        }
        if let Some(ref param_value) = p_sort {
            req_builder = req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("sort".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            );
        }
        req_builder = req_builder.query(&[("include", "videos")]);

        self.execute_request(req_builder).await
    }

    /// Adds item(s) to videos relationship.
    pub async fn user_collections_id_relationships_videos_post(
        &self,
        user_id: &str,
        videos_to_add: Vec<String>,
    ) -> Result<(), Error> {
        // add a prefix to parameters to efficiently prevent name collisions
        let p_id = user_id;
        let payload = DataWrap::new(
            videos_to_add
                .into_iter()
                .map(|video_id: String| ResourceIdentifier::new(video_id, ResourceType::Videos))
                .collect::<Vec<ResourceIdentifier>>(),
        );

        let uri_str = format!(
            "{}/userCollections/{id}/relationships/videos",
            self.base_path_api,
            id = crate::apis::urlencode(p_id)
        );
        let mut req_builder = self.client.request(reqwest::Method::POST, &uri_str);

        if let Some(country_code) = &self.country_code {
            req_builder = req_builder.query(&[("countryCode", country_code.clone())]);
        }
        req_builder = req_builder.json(&payload);

        self.execute_request(req_builder).await
    }
}
