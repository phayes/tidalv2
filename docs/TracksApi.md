# \TracksApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tracks_get**](TracksApi.md#tracks_get) | **GET** /tracks | Get multiple tracks.
[**tracks_id_delete**](TracksApi.md#tracks_id_delete) | **DELETE** /tracks/{id} | Delete single track.
[**tracks_id_get**](TracksApi.md#tracks_id_get) | **GET** /tracks/{id} | Get single track.
[**tracks_id_patch**](TracksApi.md#tracks_id_patch) | **PATCH** /tracks/{id} | Update single track.
[**tracks_id_relationships_albums_get**](TracksApi.md#tracks_id_relationships_albums_get) | **GET** /tracks/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**tracks_id_relationships_artists_get**](TracksApi.md#tracks_id_relationships_artists_get) | **GET** /tracks/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**tracks_id_relationships_genres_get**](TracksApi.md#tracks_id_relationships_genres_get) | **GET** /tracks/{id}/relationships/genres | Get genres relationship (\"to-many\").
[**tracks_id_relationships_lyrics_get**](TracksApi.md#tracks_id_relationships_lyrics_get) | **GET** /tracks/{id}/relationships/lyrics | Get lyrics relationship (\"to-many\").
[**tracks_id_relationships_owners_get**](TracksApi.md#tracks_id_relationships_owners_get) | **GET** /tracks/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**tracks_id_relationships_providers_get**](TracksApi.md#tracks_id_relationships_providers_get) | **GET** /tracks/{id}/relationships/providers | Get providers relationship (\"to-many\").
[**tracks_id_relationships_radio_get**](TracksApi.md#tracks_id_relationships_radio_get) | **GET** /tracks/{id}/relationships/radio | Get radio relationship (\"to-many\").
[**tracks_id_relationships_similar_tracks_get**](TracksApi.md#tracks_id_relationships_similar_tracks_get) | **GET** /tracks/{id}/relationships/similarTracks | Get similarTracks relationship (\"to-many\").
[**tracks_id_relationships_source_file_get**](TracksApi.md#tracks_id_relationships_source_file_get) | **GET** /tracks/{id}/relationships/sourceFile | Get sourceFile relationship (\"to-one\").
[**tracks_id_relationships_track_statistics_get**](TracksApi.md#tracks_id_relationships_track_statistics_get) | **GET** /tracks/{id}/relationships/trackStatistics | Get trackStatistics relationship (\"to-one\").
[**tracks_post**](TracksApi.md#tracks_post) | **POST** /tracks | Create single track.



## tracks_get

> models::TracksMultiResourceDataDocument tracks_get(country_code, page_left_square_bracket_cursor_right_square_bracket, include, filter_left_square_bracket_owners_period_id_right_square_bracket, filter_left_square_bracket_isrc_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple tracks.

Retrieves multiple tracks by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, genres, lyrics, owners, providers, radio, similarTracks, sourceFile, trackStatistics |  |
**filter_left_square_bracket_owners_period_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |
**filter_left_square_bracket_isrc_right_square_bracket** | Option<[**Vec<String>**](String.md)> | International Standard Recording Code (ISRC) |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | A Tidal catalogue ID |  |

### Return type

[**models::TracksMultiResourceDataDocument**](Tracks_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_delete

> tracks_id_delete(id)
Delete single track.

Deletes existing track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_get

> models::TracksSingleResourceDataDocument tracks_id_get(id, country_code, include)
Get single track.

Retrieves single track by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, genres, lyrics, owners, providers, radio, similarTracks, sourceFile, trackStatistics |  |

### Return type

[**models::TracksSingleResourceDataDocument**](Tracks_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_patch

> tracks_id_patch(id, track_update_operation_payload)
Update single track.

Updates existing track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**track_update_operation_payload** | Option<[**TrackUpdateOperationPayload**](TrackUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_albums_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_albums_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_artists_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_artists_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_genres_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_genres_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get genres relationship (\"to-many\").

Retrieves genres relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: genres |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_lyrics_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_lyrics_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get lyrics relationship (\"to-many\").

Retrieves lyrics relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: lyrics |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_owners_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_providers_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_providers_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get providers relationship (\"to-many\").

Retrieves providers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: providers |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_radio_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_radio_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get radio relationship (\"to-many\").

Retrieves radio relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: radio |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_similar_tracks_get

> models::TracksMultiRelationshipDataDocument tracks_id_relationships_similar_tracks_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get similarTracks relationship (\"to-many\").

Retrieves similarTracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: similarTracks |  |

### Return type

[**models::TracksMultiRelationshipDataDocument**](Tracks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_source_file_get

> models::TracksSingleRelationshipDataDocument tracks_id_relationships_source_file_get(id, include)
Get sourceFile relationship (\"to-one\").

Retrieves sourceFile relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: sourceFile |  |

### Return type

[**models::TracksSingleRelationshipDataDocument**](Tracks_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_id_relationships_track_statistics_get

> models::TracksSingleRelationshipDataDocument tracks_id_relationships_track_statistics_get(id, include)
Get trackStatistics relationship (\"to-one\").

Retrieves trackStatistics relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: trackStatistics |  |

### Return type

[**models::TracksSingleRelationshipDataDocument**](Tracks_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tracks_post

> models::TracksSingleResourceDataDocument tracks_post(track_create_operation_payload)
Create single track.

Creates a new track.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_create_operation_payload** | Option<[**TrackCreateOperationPayload**](TrackCreateOperationPayload.md)> |  |  |

### Return type

[**models::TracksSingleResourceDataDocument**](Tracks_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

