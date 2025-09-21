# \VideosApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**videos_get**](VideosApi.md#videos_get) | **GET** /videos | Get multiple videos.
[**videos_id_get**](VideosApi.md#videos_id_get) | **GET** /videos/{id} | Get single video.
[**videos_id_relationships_albums_get**](VideosApi.md#videos_id_relationships_albums_get) | **GET** /videos/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**videos_id_relationships_artists_get**](VideosApi.md#videos_id_relationships_artists_get) | **GET** /videos/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**videos_id_relationships_providers_get**](VideosApi.md#videos_id_relationships_providers_get) | **GET** /videos/{id}/relationships/providers | Get providers relationship (\"to-many\").
[**videos_id_relationships_thumbnail_art_get**](VideosApi.md#videos_id_relationships_thumbnail_art_get) | **GET** /videos/{id}/relationships/thumbnailArt | Get thumbnailArt relationship (\"to-many\").



## videos_get

> models::VideosMultiResourceDataDocument videos_get(country_code, include, filter_left_square_bracket_isrc_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple videos.

Retrieves multiple videos by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, providers, thumbnailArt |  |
**filter_left_square_bracket_isrc_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Allows to filter the collection of resources based on isrc attribute value |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Allows to filter the collection of resources based on id attribute value |  |

### Return type

[**models::VideosMultiResourceDataDocument**](Videos_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos_id_get

> models::VideosSingleResourceDataDocument videos_id_get(id, country_code, include)
Get single video.

Retrieves single video by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, artists, providers, thumbnailArt |  |

### Return type

[**models::VideosSingleResourceDataDocument**](Videos_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos_id_relationships_albums_get

> models::VideosMultiRelationshipDataDocument videos_id_relationships_albums_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos_id_relationships_artists_get

> models::VideosMultiRelationshipDataDocument videos_id_relationships_artists_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos_id_relationships_providers_get

> models::VideosMultiRelationshipDataDocument videos_id_relationships_providers_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get providers relationship (\"to-many\").

Retrieves providers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: providers |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## videos_id_relationships_thumbnail_art_get

> models::VideosMultiRelationshipDataDocument videos_id_relationships_thumbnail_art_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get thumbnailArt relationship (\"to-many\").

Retrieves thumbnailArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Video id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: thumbnailArt |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::VideosMultiRelationshipDataDocument**](Videos_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

