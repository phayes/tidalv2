# \PlaylistsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**playlists_get**](PlaylistsApi.md#playlists_get) | **GET** /playlists | Get multiple playlists.
[**playlists_id_delete**](PlaylistsApi.md#playlists_id_delete) | **DELETE** /playlists/{id} | Delete single playlist.
[**playlists_id_get**](PlaylistsApi.md#playlists_id_get) | **GET** /playlists/{id} | Get single playlist.
[**playlists_id_patch**](PlaylistsApi.md#playlists_id_patch) | **PATCH** /playlists/{id} | Update single playlist.
[**playlists_id_relationships_cover_art_get**](PlaylistsApi.md#playlists_id_relationships_cover_art_get) | **GET** /playlists/{id}/relationships/coverArt | Get coverArt relationship (\"to-many\").
[**playlists_id_relationships_cover_art_patch**](PlaylistsApi.md#playlists_id_relationships_cover_art_patch) | **PATCH** /playlists/{id}/relationships/coverArt | Update coverArt relationship (\"to-many\").
[**playlists_id_relationships_items_delete**](PlaylistsApi.md#playlists_id_relationships_items_delete) | **DELETE** /playlists/{id}/relationships/items | Delete from items relationship (\"to-many\").
[**playlists_id_relationships_items_get**](PlaylistsApi.md#playlists_id_relationships_items_get) | **GET** /playlists/{id}/relationships/items | Get items relationship (\"to-many\").
[**playlists_id_relationships_items_patch**](PlaylistsApi.md#playlists_id_relationships_items_patch) | **PATCH** /playlists/{id}/relationships/items | Update items relationship (\"to-many\").
[**playlists_id_relationships_items_post**](PlaylistsApi.md#playlists_id_relationships_items_post) | **POST** /playlists/{id}/relationships/items | Add to items relationship (\"to-many\").
[**playlists_id_relationships_owners_get**](PlaylistsApi.md#playlists_id_relationships_owners_get) | **GET** /playlists/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**playlists_post**](PlaylistsApi.md#playlists_post) | **POST** /playlists | Create single playlist.



## playlists_get

> models::PlaylistsMultiResourceDataDocument playlists_get(country_code, page_left_square_bracket_cursor_right_square_bracket, sort, include, filter_left_square_bracket_owners_period_id_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple playlists.

Retrieves multiple playlists by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**sort** | Option<[**Vec<String>**](String.md)> | Values prefixed with \"-\" are sorted descending; values without it are sorted ascending. |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt, items, owners |  |
**filter_left_square_bracket_owners_period_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Playlist id |  |

### Return type

[**models::PlaylistsMultiResourceDataDocument**](Playlists_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_delete

> playlists_id_delete(id)
Delete single playlist.

Deletes existing playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_get

> models::PlaylistsSingleResourceDataDocument playlists_id_get(id, country_code, include)
Get single playlist.

Retrieves single playlist by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt, items, owners |  |

### Return type

[**models::PlaylistsSingleResourceDataDocument**](Playlists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_patch

> playlists_id_patch(id, country_code, playlist_update_operation_payload)
Update single playlist.

Updates existing playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**playlist_update_operation_payload** | Option<[**PlaylistUpdateOperationPayload**](PlaylistUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_cover_art_get

> models::PlaylistsMultiRelationshipDataDocument playlists_id_relationships_cover_art_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get coverArt relationship (\"to-many\").

Retrieves coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::PlaylistsMultiRelationshipDataDocument**](Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_cover_art_patch

> playlists_id_relationships_cover_art_patch(id, playlist_cover_art_relationship_update_operation_payload)
Update coverArt relationship (\"to-many\").

Updates coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**playlist_cover_art_relationship_update_operation_payload** | Option<[**PlaylistCoverArtRelationshipUpdateOperationPayload**](PlaylistCoverArtRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_items_delete

> playlists_id_relationships_items_delete(id, playlist_items_relationship_remove_operation_payload)
Delete from items relationship (\"to-many\").

Deletes item(s) from items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**playlist_items_relationship_remove_operation_payload** | Option<[**PlaylistItemsRelationshipRemoveOperationPayload**](PlaylistItemsRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_items_get

> models::PlaylistsItemsMultiRelationshipDataDocument playlists_id_relationships_items_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get items relationship (\"to-many\").

Retrieves items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items |  |

### Return type

[**models::PlaylistsItemsMultiRelationshipDataDocument**](Playlists_Items_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_items_patch

> playlists_id_relationships_items_patch(id, playlist_items_relationship_reorder_operation_payload)
Update items relationship (\"to-many\").

Updates items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**playlist_items_relationship_reorder_operation_payload** | Option<[**PlaylistItemsRelationshipReorderOperationPayload**](PlaylistItemsRelationshipReorderOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_items_post

> playlists_id_relationships_items_post(id, country_code, playlist_items_relationship_add_operation_payload)
Add to items relationship (\"to-many\").

Adds item(s) to items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**playlist_items_relationship_add_operation_payload** | Option<[**PlaylistItemsRelationshipAddOperationPayload**](PlaylistItemsRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_id_relationships_owners_get

> models::PlaylistsMultiRelationshipDataDocument playlists_id_relationships_owners_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Playlist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::PlaylistsMultiRelationshipDataDocument**](Playlists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## playlists_post

> models::PlaylistsSingleResourceDataDocument playlists_post(country_code, playlist_create_operation_payload)
Create single playlist.

Creates a new playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**playlist_create_operation_payload** | Option<[**PlaylistCreateOperationPayload**](PlaylistCreateOperationPayload.md)> |  |  |

### Return type

[**models::PlaylistsSingleResourceDataDocument**](Playlists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

