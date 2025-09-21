# \UserSharesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_shares_get**](UserSharesApi.md#user_shares_get) | **GET** /userShares | Get multiple userShares.
[**user_shares_id_get**](UserSharesApi.md#user_shares_id_get) | **GET** /userShares/{id} | Get single userShare.
[**user_shares_id_relationships_owners_get**](UserSharesApi.md#user_shares_id_relationships_owners_get) | **GET** /userShares/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**user_shares_id_relationships_shared_resources_get**](UserSharesApi.md#user_shares_id_relationships_shared_resources_get) | **GET** /userShares/{id}/relationships/sharedResources | Get sharedResources relationship (\"to-many\").
[**user_shares_post**](UserSharesApi.md#user_shares_post) | **POST** /userShares | Create single userShare.



## user_shares_get

> models::UserSharesMultiResourceDataDocument user_shares_get(include, filter_left_square_bracket_code_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple userShares.

Retrieves multiple userShares by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, sharedResources |  |
**filter_left_square_bracket_code_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Share code |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User share id |  |

### Return type

[**models::UserSharesMultiResourceDataDocument**](UserShares_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_shares_id_get

> models::UserSharesSingleResourceDataDocument user_shares_id_get(id, include)
Get single userShare.

Retrieves single userShare by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User share id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners, sharedResources |  |

### Return type

[**models::UserSharesSingleResourceDataDocument**](UserShares_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_shares_id_relationships_owners_get

> models::UserSharesMultiRelationshipDataDocument user_shares_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User share id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserSharesMultiRelationshipDataDocument**](UserShares_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_shares_id_relationships_shared_resources_get

> models::UserSharesMultiRelationshipDataDocument user_shares_id_relationships_shared_resources_get(id, page_left_square_bracket_cursor_right_square_bracket, include)
Get sharedResources relationship (\"to-many\").

Retrieves sharedResources relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User share id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: sharedResources |  |

### Return type

[**models::UserSharesMultiRelationshipDataDocument**](UserShares_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_shares_post

> models::UserSharesSingleResourceDataDocument user_shares_post(user_shares_create_operation_payload)
Create single userShare.

Creates a new userShare.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_shares_create_operation_payload** | Option<[**UserSharesCreateOperationPayload**](UserSharesCreateOperationPayload.md)> |  |  |

### Return type

[**models::UserSharesSingleResourceDataDocument**](UserShares_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

