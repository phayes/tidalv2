# \ArtworksApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**artworks_get**](ArtworksApi.md#artworks_get) | **GET** /artworks | Get multiple artworks.
[**artworks_id_get**](ArtworksApi.md#artworks_id_get) | **GET** /artworks/{id} | Get single artwork.
[**artworks_id_relationships_owners_get**](ArtworksApi.md#artworks_id_relationships_owners_get) | **GET** /artworks/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**artworks_post**](ArtworksApi.md#artworks_post) | **POST** /artworks | Create single artwork.



## artworks_get

> models::ArtworksMultiResourceDataDocument artworks_get(country_code, include, filter_left_square_bracket_id_right_square_bracket)
Get multiple artworks.

Retrieves multiple artworks by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artwork id |  |

### Return type

[**models::ArtworksMultiResourceDataDocument**](Artworks_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artworks_id_get

> models::ArtworksSingleResourceDataDocument artworks_id_get(id, country_code, include)
Get single artwork.

Retrieves single artwork by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artwork id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::ArtworksSingleResourceDataDocument**](Artworks_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artworks_id_relationships_owners_get

> models::ArtworksMultiRelationshipDataDocument artworks_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artwork id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtworksMultiRelationshipDataDocument**](Artworks_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artworks_post

> models::ArtworksSingleResourceDataDocument artworks_post(artwork_create_operation_payload)
Create single artwork.

Creates a new artwork.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artwork_create_operation_payload** | Option<[**ArtworkCreateOperationPayload**](ArtworkCreateOperationPayload.md)> |  |  |

### Return type

[**models::ArtworksSingleResourceDataDocument**](Artworks_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

