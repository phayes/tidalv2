# \LyricsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lyrics_get**](LyricsApi.md#lyrics_get) | **GET** /lyrics | Get multiple lyrics.
[**lyrics_id_get**](LyricsApi.md#lyrics_id_get) | **GET** /lyrics/{id} | Get single lyric.
[**lyrics_id_patch**](LyricsApi.md#lyrics_id_patch) | **PATCH** /lyrics/{id} | Update single lyric.
[**lyrics_id_relationships_owners_get**](LyricsApi.md#lyrics_id_relationships_owners_get) | **GET** /lyrics/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**lyrics_id_relationships_track_get**](LyricsApi.md#lyrics_id_relationships_track_get) | **GET** /lyrics/{id}/relationships/track | Get track relationship (\"to-one\").
[**lyrics_post**](LyricsApi.md#lyrics_post) | **POST** /lyrics | Create single lyric.



## lyrics_get

> models::LyricsMultiResourceDataDocument lyrics_get(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple lyrics.

Retrieves multiple lyrics by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, track |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Lyrics Id |  |

### Return type

[**models::LyricsMultiResourceDataDocument**](Lyrics_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lyrics_id_get

> models::LyricsSingleResourceDataDocument lyrics_id_get(id, include)
Get single lyric.

Retrieves single lyric by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, track |  |

### Return type

[**models::LyricsSingleResourceDataDocument**](Lyrics_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lyrics_id_patch

> lyrics_id_patch(id, lyrics_update_operation_payload)
Update single lyric.

Updates existing lyric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**lyrics_update_operation_payload** | Option<[**LyricsUpdateOperationPayload**](LyricsUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lyrics_id_relationships_owners_get

> models::LyricsMultiRelationshipDataDocument lyrics_id_relationships_owners_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::LyricsMultiRelationshipDataDocument**](Lyrics_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lyrics_id_relationships_track_get

> models::LyricsSingleRelationshipDataDocument lyrics_id_relationships_track_get(id, country_code, include)
Get track relationship (\"to-one\").

Retrieves track relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Lyrics Id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: track |  |

### Return type

[**models::LyricsSingleRelationshipDataDocument**](Lyrics_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lyrics_post

> models::LyricsSingleResourceDataDocument lyrics_post(lyrics_create_operation_payload)
Create single lyric.

Creates a new lyric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lyrics_create_operation_payload** | Option<[**LyricsCreateOperationPayload**](LyricsCreateOperationPayload.md)> |  |  |

### Return type

[**models::LyricsSingleResourceDataDocument**](Lyrics_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

