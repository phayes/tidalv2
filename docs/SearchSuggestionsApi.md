# \SearchSuggestionsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_suggestions_id_get**](SearchSuggestionsApi.md#search_suggestions_id_get) | **GET** /searchSuggestions/{id} | Get single searchSuggestion.
[**search_suggestions_id_relationships_direct_hits_get**](SearchSuggestionsApi.md#search_suggestions_id_relationships_direct_hits_get) | **GET** /searchSuggestions/{id}/relationships/directHits | Get directHits relationship (\"to-many\").



## search_suggestions_id_get

> models::SearchSuggestionsSingleResourceDataDocument search_suggestions_id_get(id, country_code, explicit_filter, include)
Get single searchSuggestion.

Retrieves single searchSuggestion by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: directHits |  |

### Return type

[**models::SearchSuggestionsSingleResourceDataDocument**](SearchSuggestions_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_suggestions_id_relationships_direct_hits_get

> models::SearchSuggestionsMultiRelationshipDataDocument search_suggestions_id_relationships_direct_hits_get(id, country_code, explicit_filter, include, page_left_square_bracket_cursor_right_square_bracket)
Get directHits relationship (\"to-many\").

Retrieves directHits relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**explicit_filter** | Option<**String**> | Explicit filter |  |[default to INCLUDE]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: directHits |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::SearchSuggestionsMultiRelationshipDataDocument**](SearchSuggestions_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

