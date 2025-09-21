# \UserEntitlementsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_entitlements_id_get**](UserEntitlementsApi.md#user_entitlements_id_get) | **GET** /userEntitlements/{id} | Get single userEntitlement.



## user_entitlements_id_get

> models::UserEntitlementsSingleResourceDataDocument user_entitlements_id_get(id)
Get single userEntitlement.

Retrieves single userEntitlement by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |

### Return type

[**models::UserEntitlementsSingleResourceDataDocument**](UserEntitlements_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

