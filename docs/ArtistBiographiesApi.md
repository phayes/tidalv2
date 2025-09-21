# \ArtistBiographiesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**artist_biographies_get**](ArtistBiographiesApi.md#artist_biographies_get) | **GET** /artistBiographies | Get multiple artistBiographies.
[**artist_biographies_id_get**](ArtistBiographiesApi.md#artist_biographies_id_get) | **GET** /artistBiographies/{id} | Get single artistBiographie.
[**artist_biographies_id_patch**](ArtistBiographiesApi.md#artist_biographies_id_patch) | **PATCH** /artistBiographies/{id} | Update single artistBiographie.
[**artist_biographies_id_relationships_owners_get**](ArtistBiographiesApi.md#artist_biographies_id_relationships_owners_get) | **GET** /artistBiographies/{id}/relationships/owners | Get owners relationship (\"to-many\").



## artist_biographies_get

> models::ArtistBiographiesMultiResourceDataDocument artist_biographies_get(country_code, include, filter_left_square_bracket_id_right_square_bracket)
Get multiple artistBiographies.

Retrieves multiple artistBiographies by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist id |  |

### Return type

[**models::ArtistBiographiesMultiResourceDataDocument**](ArtistBiographies_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_biographies_id_get

> models::ArtistBiographiesSingleResourceDataDocument artist_biographies_id_get(id, country_code, include)
Get single artistBiographie.

Retrieves single artistBiographie by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::ArtistBiographiesSingleResourceDataDocument**](ArtistBiographies_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_biographies_id_patch

> artist_biographies_id_patch(id, artist_biography_update_body)
Update single artistBiographie.

Updates existing artistBiographie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_biography_update_body** | Option<[**ArtistBiographyUpdateBody**](ArtistBiographyUpdateBody.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_biographies_id_relationships_owners_get

> models::ArtistBiographiesMultiRelationshipDataDocument artist_biographies_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistBiographiesMultiRelationshipDataDocument**](ArtistBiographies_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

