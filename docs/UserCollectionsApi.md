# \UserCollectionsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_collections_id_get**](UserCollectionsApi.md#user_collections_id_get) | **GET** /userCollections/{id} | Get single userCollection.
[**user_collections_id_relationships_albums_delete**](UserCollectionsApi.md#user_collections_id_relationships_albums_delete) | **DELETE** /userCollections/{id}/relationships/albums | Delete from albums relationship (\"to-many\").
[**user_collections_id_relationships_albums_get**](UserCollectionsApi.md#user_collections_id_relationships_albums_get) | **GET** /userCollections/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**user_collections_id_relationships_albums_post**](UserCollectionsApi.md#user_collections_id_relationships_albums_post) | **POST** /userCollections/{id}/relationships/albums | Add to albums relationship (\"to-many\").
[**user_collections_id_relationships_artists_delete**](UserCollectionsApi.md#user_collections_id_relationships_artists_delete) | **DELETE** /userCollections/{id}/relationships/artists | Delete from artists relationship (\"to-many\").
[**user_collections_id_relationships_artists_get**](UserCollectionsApi.md#user_collections_id_relationships_artists_get) | **GET** /userCollections/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**user_collections_id_relationships_artists_post**](UserCollectionsApi.md#user_collections_id_relationships_artists_post) | **POST** /userCollections/{id}/relationships/artists | Add to artists relationship (\"to-many\").
[**user_collections_id_relationships_owners_get**](UserCollectionsApi.md#user_collections_id_relationships_owners_get) | **GET** /userCollections/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**user_collections_id_relationships_playlists_delete**](UserCollectionsApi.md#user_collections_id_relationships_playlists_delete) | **DELETE** /userCollections/{id}/relationships/playlists | Delete from playlists relationship (\"to-many\").
[**user_collections_id_relationships_playlists_get**](UserCollectionsApi.md#user_collections_id_relationships_playlists_get) | **GET** /userCollections/{id}/relationships/playlists | Get playlists relationship (\"to-many\").
[**user_collections_id_relationships_playlists_post**](UserCollectionsApi.md#user_collections_id_relationships_playlists_post) | **POST** /userCollections/{id}/relationships/playlists | Add to playlists relationship (\"to-many\").
[**user_collections_id_relationships_tracks_delete**](UserCollectionsApi.md#user_collections_id_relationships_tracks_delete) | **DELETE** /userCollections/{id}/relationships/tracks | Delete from tracks relationship (\"to-many\").
[**user_collections_id_relationships_tracks_get**](UserCollectionsApi.md#user_collections_id_relationships_tracks_get) | **GET** /userCollections/{id}/relationships/tracks | Get tracks relationship (\"to-many\").
[**user_collections_id_relationships_tracks_post**](UserCollectionsApi.md#user_collections_id_relationships_tracks_post) | **POST** /userCollections/{id}/relationships/tracks | Add to tracks relationship (\"to-many\").
[**user_collections_id_relationships_videos_delete**](UserCollectionsApi.md#user_collections_id_relationships_videos_delete) | **DELETE** /userCollections/{id}/relationships/videos | Delete from videos relationship (\"to-many\").
[**user_collections_id_relationships_videos_get**](UserCollectionsApi.md#user_collections_id_relationships_videos_get) | **GET** /userCollections/{id}/relationships/videos | Get videos relationship (\"to-many\").
[**user_collections_id_relationships_videos_post**](UserCollectionsApi.md#user_collections_id_relationships_videos_post) | **POST** /userCollections/{id}/relationships/videos | Add to videos relationship (\"to-many\").



## user_collections_id_get

> models::UserCollectionsSingleResourceDataDocument user_collections_id_get(id, locale, country_code, include)
Get single userCollection.

Retrieves single userCollection by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**locale** | **String** | BCP 47 locale | [required] |[default to en-US]
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, owners, playlists, tracks, videos |  |

### Return type

[**models::UserCollectionsSingleResourceDataDocument**](UserCollections_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_albums_delete

> user_collections_id_relationships_albums_delete(id, user_collection_albums_relationship_remove_operation_payload)
Delete from albums relationship (\"to-many\").

Deletes item(s) from albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**user_collection_albums_relationship_remove_operation_payload** | Option<[**UserCollectionAlbumsRelationshipRemoveOperationPayload**](UserCollectionAlbumsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_albums_get

> models::UserCollectionsAlbumsMultiRelationshipDataDocument user_collections_id_relationships_albums_get(id, country_code, locale, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**locale** | **String** | BCP 47 locale | [required] |[default to en-US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |

### Return type

[**models::UserCollectionsAlbumsMultiRelationshipDataDocument**](UserCollections_Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_albums_post

> user_collections_id_relationships_albums_post(id, country_code, user_collection_albums_relationship_add_operation_payload)
Add to albums relationship (\"to-many\").

Adds item(s) to albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**user_collection_albums_relationship_add_operation_payload** | Option<[**UserCollectionAlbumsRelationshipAddOperationPayload**](UserCollectionAlbumsRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_artists_delete

> user_collections_id_relationships_artists_delete(id, user_collection_artists_relationship_remove_operation_payload)
Delete from artists relationship (\"to-many\").

Deletes item(s) from artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**user_collection_artists_relationship_remove_operation_payload** | Option<[**UserCollectionArtistsRelationshipRemoveOperationPayload**](UserCollectionArtistsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_artists_get

> models::UserCollectionsArtistsMultiRelationshipDataDocument user_collections_id_relationships_artists_get(id, country_code, locale, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**locale** | **String** | BCP 47 locale | [required] |[default to en-US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |

### Return type

[**models::UserCollectionsArtistsMultiRelationshipDataDocument**](UserCollections_Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_artists_post

> user_collections_id_relationships_artists_post(id, country_code, user_collection_artists_relationship_add_operation_payload)
Add to artists relationship (\"to-many\").

Adds item(s) to artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**user_collection_artists_relationship_add_operation_payload** | Option<[**UserCollectionArtistsRelationshipAddOperationPayload**](UserCollectionArtistsRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_owners_get

> models::UserCollectionsMultiRelationshipDataDocument user_collections_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserCollectionsMultiRelationshipDataDocument**](UserCollections_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_playlists_delete

> user_collections_id_relationships_playlists_delete(id, user_collection_playlists_relationship_remove_operation_payload)
Delete from playlists relationship (\"to-many\").

Deletes item(s) from playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**user_collection_playlists_relationship_remove_operation_payload** | Option<[**UserCollectionPlaylistsRelationshipRemoveOperationPayload**](UserCollectionPlaylistsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_playlists_get

> models::UserCollectionsPlaylistsMultiRelationshipDataDocument user_collections_id_relationships_playlists_get(id, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get playlists relationship (\"to-many\").

Retrieves playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: playlists |  |

### Return type

[**models::UserCollectionsPlaylistsMultiRelationshipDataDocument**](UserCollections_Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_playlists_post

> user_collections_id_relationships_playlists_post(id, user_collection_playlists_relationship_remove_operation_payload)
Add to playlists relationship (\"to-many\").

Adds item(s) to playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**user_collection_playlists_relationship_remove_operation_payload** | Option<[**UserCollectionPlaylistsRelationshipRemoveOperationPayload**](UserCollectionPlaylistsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_tracks_delete

> user_collections_id_relationships_tracks_delete(id, user_collection_tracks_relationship_remove_operation_payload)
Delete from tracks relationship (\"to-many\").

Deletes item(s) from tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**user_collection_tracks_relationship_remove_operation_payload** | Option<[**UserCollectionTracksRelationshipRemoveOperationPayload**](UserCollectionTracksRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_tracks_get

> models::UserCollectionsTracksMultiRelationshipDataDocument user_collections_id_relationships_tracks_get(id, country_code, locale, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get tracks relationship (\"to-many\").

Retrieves tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**locale** | **String** | BCP 47 locale | [required] |[default to en-US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: tracks |  |

### Return type

[**models::UserCollectionsTracksMultiRelationshipDataDocument**](UserCollections_Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_tracks_post

> user_collections_id_relationships_tracks_post(id, country_code, user_collection_tracks_relationship_add_operation_payload)
Add to tracks relationship (\"to-many\").

Adds item(s) to tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**user_collection_tracks_relationship_add_operation_payload** | Option<[**UserCollectionTracksRelationshipAddOperationPayload**](UserCollectionTracksRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_videos_delete

> user_collections_id_relationships_videos_delete(id, user_collection_videos_relationship_remove_operation_payload)
Delete from videos relationship (\"to-many\").

Deletes item(s) from videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**user_collection_videos_relationship_remove_operation_payload** | Option<[**UserCollectionVideosRelationshipRemoveOperationPayload**](UserCollectionVideosRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_videos_get

> models::UserCollectionsVideosMultiRelationshipDataDocument user_collections_id_relationships_videos_get(id, country_code, locale, page_left_square_bracket_cursor_right_square_bracket, sort, include)
Get videos relationship (\"to-many\").

Retrieves videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**locale** | **String** | BCP 47 locale | [required] |[default to en-US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: videos |  |

### Return type

[**models::UserCollectionsVideosMultiRelationshipDataDocument**](UserCollections_Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_collections_id_relationships_videos_post

> user_collections_id_relationships_videos_post(id, country_code, user_collection_videos_relationship_add_operation_payload)
Add to videos relationship (\"to-many\").

Adds item(s) to videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**user_collection_videos_relationship_add_operation_payload** | Option<[**UserCollectionVideosRelationshipAddOperationPayload**](UserCollectionVideosRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

