# \ArtistRolesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**artist_roles_get**](ArtistRolesApi.md#artist_roles_get) | **GET** /artistRoles | Get multiple artistRoles.
[**artist_roles_id_get**](ArtistRolesApi.md#artist_roles_id_get) | **GET** /artistRoles/{id} | Get single artistRole.



## artist_roles_get

> models::ArtistRolesMultiResourceDataDocument artist_roles_get(filter_left_square_bracket_id_right_square_bracket)
Get multiple artistRoles.

Retrieves multiple artistRoles by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Allows to filter the collection of resources based on id attribute value |  |

### Return type

[**models::ArtistRolesMultiResourceDataDocument**](ArtistRoles_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_roles_id_get

> models::ArtistRolesSingleResourceDataDocument artist_roles_id_get(id)
Get single artistRole.

Retrieves single artistRole by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist role id | [required] |

### Return type

[**models::ArtistRolesSingleResourceDataDocument**](ArtistRoles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

