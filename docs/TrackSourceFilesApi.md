# \TrackSourceFilesApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_source_files_get**](TrackSourceFilesApi.md#track_source_files_get) | **GET** /trackSourceFiles | Get multiple trackSourceFiles.
[**track_source_files_id_get**](TrackSourceFilesApi.md#track_source_files_id_get) | **GET** /trackSourceFiles/{id} | Get single trackSourceFile.
[**track_source_files_id_relationships_owners_get**](TrackSourceFilesApi.md#track_source_files_id_relationships_owners_get) | **GET** /trackSourceFiles/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**track_source_files_post**](TrackSourceFilesApi.md#track_source_files_post) | **POST** /trackSourceFiles | Create single trackSourceFile.



## track_source_files_get

> models::TrackSourceFilesMultiResourceDataDocument track_source_files_get(include, filter_left_square_bracket_id_right_square_bracket)
Get multiple trackSourceFiles.

Retrieves multiple trackSourceFiles by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Track source file id |  |

### Return type

[**models::TrackSourceFilesMultiResourceDataDocument**](TrackSourceFiles_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_source_files_id_get

> models::TrackSourceFilesSingleResourceDataDocument track_source_files_id_get(id, include)
Get single trackSourceFile.

Retrieves single trackSourceFile by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track source file id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |

### Return type

[**models::TrackSourceFilesSingleResourceDataDocument**](TrackSourceFiles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_source_files_id_relationships_owners_get

> models::TrackSourceFilesMultiRelationshipDataDocument track_source_files_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Track source file id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::TrackSourceFilesMultiRelationshipDataDocument**](TrackSourceFiles_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_source_files_post

> models::TrackSourceFilesSingleResourceDataDocument track_source_files_post(track_source_file_create_operation_payload)
Create single trackSourceFile.

Creates a new trackSourceFile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_source_file_create_operation_payload** | Option<[**TrackSourceFileCreateOperationPayload**](TrackSourceFileCreateOperationPayload.md)> |  |  |

### Return type

[**models::TrackSourceFilesSingleResourceDataDocument**](TrackSourceFiles_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

