# \ProvidersApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**providers_get**](ProvidersApi.md#providers_get) | **GET** /providers | Get multiple providers.
[**providers_id_get**](ProvidersApi.md#providers_id_get) | **GET** /providers/{id} | Get single provider.



## providers_get

> models::ProvidersMultiResourceDataDocument providers_get(filter_left_square_bracket_id_right_square_bracket)
Get multiple providers.

Retrieves multiple providers by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Allows to filter the collection of resources based on id attribute value |  |

### Return type

[**models::ProvidersMultiResourceDataDocument**](Providers_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers_id_get

> models::ProvidersSingleResourceDataDocument providers_id_get(id)
Get single provider.

Retrieves single provider by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Provider id | [required] |

### Return type

[**models::ProvidersSingleResourceDataDocument**](Providers_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

