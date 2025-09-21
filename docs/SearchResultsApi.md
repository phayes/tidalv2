# \SearchResultsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_results_id_get**](SearchResultsApi.md#search_results_id_get) | **GET** /searchResults/{id} | Get single searchResult.
[**search_results_id_relationships_albums_get**](SearchResultsApi.md#search_results_id_relationships_albums_get) | **GET** /searchResults/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**search_results_id_relationships_artists_get**](SearchResultsApi.md#search_results_id_relationships_artists_get) | **GET** /searchResults/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**search_results_id_relationships_playlists_get**](SearchResultsApi.md#search_results_id_relationships_playlists_get) | **GET** /searchResults/{id}/relationships/playlists | Get playlists relationship (\"to-many\").
[**search_results_id_relationships_top_hits_get**](SearchResultsApi.md#search_results_id_relationships_top_hits_get) | **GET** /searchResults/{id}/relationships/topHits | Get topHits relationship (\"to-many\").
[**search_results_id_relationships_tracks_get**](SearchResultsApi.md#search_results_id_relationships_tracks_get) | **GET** /searchResults/{id}/relationships/tracks | Get tracks relationship (\"to-many\").
[**search_results_id_relationships_videos_get**](SearchResultsApi.md#search_results_id_relationships_videos_get) | **GET** /searchResults/{id}/relationships/videos | Get videos relationship (\"to-many\").



## search_results_id_get

> models::SearchResultsSingleResourceDataDocument search_results_id_get(id, country_code, explicit_filter, include)
Get single searchResult.

Retrieves single searchResult by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, playlists, topHits, tracks, videos |  |

### Return type

[**models::SearchResultsSingleResourceDataDocument**](SearchResults_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_results_id_relationships_albums_get

> models::SearchResultsMultiRelationshipDataDocument search_results_id_relationships_albums_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_results_id_relationships_artists_get

> models::SearchResultsMultiRelationshipDataDocument search_results_id_relationships_artists_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_results_id_relationships_playlists_get

> models::SearchResultsMultiRelationshipDataDocument search_results_id_relationships_playlists_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get playlists relationship (\"to-many\").

Retrieves playlists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: playlists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_results_id_relationships_top_hits_get

> models::SearchResultsMultiRelationshipDataDocument search_results_id_relationships_top_hits_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get topHits relationship (\"to-many\").

Retrieves topHits relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: topHits |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_results_id_relationships_tracks_get

> models::SearchResultsMultiRelationshipDataDocument search_results_id_relationships_tracks_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get tracks relationship (\"to-many\").

Retrieves tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: tracks |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_results_id_relationships_videos_get

> models::SearchResultsMultiRelationshipDataDocument search_results_id_relationships_videos_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get videos relationship (\"to-many\").

Retrieves videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Search query | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**explicit_filter** | Option<**String**> | Explicit filter |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: videos |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchResultsMultiRelationshipDataDocument**](SearchResults_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

