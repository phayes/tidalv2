# \TrackFilesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_files_id_get**](TrackFilesApi.md#track_files_id_get) | **GET** /trackFiles/{id} | Get single trackFile.



## track_files_id_get

> models::TrackFilesSingleResourceDataDocument track_files_id_get(id, formats, usage)
Get single trackFile.

Retrieves single trackFile by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**formats** | **String** |  | [required] |
**usage** | **String** |  | [required] |

### Return type

[**models::TrackFilesSingleResourceDataDocument**](TrackFiles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

