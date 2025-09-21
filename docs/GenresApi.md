# \GenresApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**genres_get**](GenresApi.md#genres_get) | **GET** /genres | Get multiple genres.
[**genres_id_get**](GenresApi.md#genres_id_get) | **GET** /genres/{id} | Get single genre.



## genres_get

> models::GenresMultiResourceDataDocument genres_get(page_left_square_bracket_cursor_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple genres.

Retrieves multiple genres by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Allows filtering by genre id(s). USER_SELECTABLE is special value used to return specific genres which users can select from |  |

### Return type

[**models::GenresMultiResourceDataDocument**](Genres_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genres_id_get

> models::GenresSingleResourceDataDocument genres_id_get(id)
Get single genre.

Retrieves single genre by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Genre id | [required] |

### Return type

[**models::GenresSingleResourceDataDocument**](Genres_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

