# \UserRecommendationsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_recommendations_id_get**](UserRecommendationsApi.md#user_recommendations_id_get) | **GET** /userRecommendations/{id} | Get single userRecommendation.
[**user_recommendations_id_relationships_discovery_mixes_get**](UserRecommendationsApi.md#user_recommendations_id_relationships_discovery_mixes_get) | **GET** /userRecommendations/{id}/relationships/discoveryMixes | Get discoveryMixes relationship (\"to-many\").
[**user_recommendations_id_relationships_my_mixes_get**](UserRecommendationsApi.md#user_recommendations_id_relationships_my_mixes_get) | **GET** /userRecommendations/{id}/relationships/myMixes | Get myMixes relationship (\"to-many\").
[**user_recommendations_id_relationships_new_arrival_mixes_get**](UserRecommendationsApi.md#user_recommendations_id_relationships_new_arrival_mixes_get) | **GET** /userRecommendations/{id}/relationships/newArrivalMixes | Get newArrivalMixes relationship (\"to-many\").



## user_recommendations_id_get

> models::UserRecommendationsSingleResourceDataDocument user_recommendations_id_get(id, country_code, locale, include)
Get single userRecommendation.

Retrieves single userRecommendation by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**locale** | **String** | BCP47 locale code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: discoveryMixes, myMixes, newArrivalMixes |  |

### Return type

[**models::UserRecommendationsSingleResourceDataDocument**](UserRecommendations_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_recommendations_id_relationships_discovery_mixes_get

> models::UserRecommendationsMultiRelationshipDataDocument user_recommendations_id_relationships_discovery_mixes_get(id, country_code, locale, include, page_left_square_bracket_cursor_right_square_bracket)
Get discoveryMixes relationship (\"to-many\").

Retrieves discoveryMixes relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**locale** | **String** | BCP47 locale code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: discoveryMixes |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserRecommendationsMultiRelationshipDataDocument**](UserRecommendations_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_recommendations_id_relationships_my_mixes_get

> models::UserRecommendationsMultiRelationshipDataDocument user_recommendations_id_relationships_my_mixes_get(id, country_code, locale, include, page_left_square_bracket_cursor_right_square_bracket)
Get myMixes relationship (\"to-many\").

Retrieves myMixes relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**locale** | **String** | BCP47 locale code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: myMixes |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserRecommendationsMultiRelationshipDataDocument**](UserRecommendations_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_recommendations_id_relationships_new_arrival_mixes_get

> models::UserRecommendationsMultiRelationshipDataDocument user_recommendations_id_relationships_new_arrival_mixes_get(id, country_code, locale, include, page_left_square_bracket_cursor_right_square_bracket)
Get newArrivalMixes relationship (\"to-many\").

Retrieves newArrivalMixes relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | User id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**locale** | **String** | BCP47 locale code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: newArrivalMixes |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::UserRecommendationsMultiRelationshipDataDocument**](UserRecommendations_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

