# \TrackStatisticsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_statistics_get**](TrackStatisticsApi.md#track_statistics_get) | **GET** /trackStatistics | Get multiple trackStatistics.
[**track_statistics_id_get**](TrackStatisticsApi.md#track_statistics_id_get) | **GET** /trackStatistics/{id} | Get single trackStatistic.
[**track_statistics_id_relationships_owners_get**](TrackStatisticsApi.md#track_statistics_id_relationships_owners_get) | **GET** /trackStatistics/{id}/relationships/owners | Get owners relationship (\"to-many\").



## track_statistics_get

> models::TrackStatisticsMultiResourceDataDocument track_statistics_get(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple trackStatistics.

Retrieves multiple trackStatistics by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | A Tidal catalogue ID |  |

### Return type

[**models::TrackStatisticsMultiResourceDataDocument**](TrackStatistics_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_statistics_id_get

> models::TrackStatisticsSingleResourceDataDocument track_statistics_id_get(id, include)
Get single trackStatistic.

Retrieves single trackStatistic by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::TrackStatisticsSingleResourceDataDocument**](TrackStatistics_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_statistics_id_relationships_owners_get

> models::TrackStatisticsMultiRelationshipDataDocument track_statistics_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | A Tidal catalogue ID | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TrackStatisticsMultiRelationshipDataDocument**](TrackStatistics_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

