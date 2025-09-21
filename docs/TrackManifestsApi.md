# \TrackManifestsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_manifests_id_get**](TrackManifestsApi.md#track_manifests_id_get) | **GET** /trackManifests/{id} | Get single trackManifest.



## track_manifests_id_get

> models::TrackManifestsSingleResourceDataDocument track_manifests_id_get(id, manifest_type, formats, uri_scheme, usage, adaptive)
Get single trackManifest.

Retrieves single trackManifest by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**manifest_type** | **String** |  | [required] |
**formats** | **String** |  | [required] |
**uri_scheme** | **String** |  | [required] |
**usage** | **String** |  | [required] |
**adaptive** | **String** |  | [required] |

### Return type

[**models::TrackManifestsSingleResourceDataDocument**](TrackManifests_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

