# \AppreciationsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**appreciations_post**](AppreciationsApi.md#appreciations_post) | **POST** /appreciations | Create single appreciation.



## appreciations_post

> models::AppreciationsSingleResourceDataDocument appreciations_post(appreciations_create_operation_payload)
Create single appreciation.

Creates a new appreciation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appreciations_create_operation_payload** | Option<[**AppreciationsCreateOperationPayload**](AppreciationsCreateOperationPayload.md)> |  |  |

### Return type

[**models::AppreciationsSingleResourceDataDocument**](Appreciations_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

