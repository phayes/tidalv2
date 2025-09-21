# \AlbumsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**albums_get**](AlbumsApi.md#albums_get) | **GET** /albums | Get multiple albums.
[**albums_id_delete**](AlbumsApi.md#albums_id_delete) | **DELETE** /albums/{id} | Delete single album.
[**albums_id_get**](AlbumsApi.md#albums_id_get) | **GET** /albums/{id} | Get single album.
[**albums_id_patch**](AlbumsApi.md#albums_id_patch) | **PATCH** /albums/{id} | Update single album.
[**albums_id_relationships_artists_get**](AlbumsApi.md#albums_id_relationships_artists_get) | **GET** /albums/{id}/relationships/artists | Get artists relationship (\"to-many\").
[**albums_id_relationships_cover_art_get**](AlbumsApi.md#albums_id_relationships_cover_art_get) | **GET** /albums/{id}/relationships/coverArt | Get coverArt relationship (\"to-many\").
[**albums_id_relationships_cover_art_patch**](AlbumsApi.md#albums_id_relationships_cover_art_patch) | **PATCH** /albums/{id}/relationships/coverArt | Update coverArt relationship (\"to-many\").
[**albums_id_relationships_genres_get**](AlbumsApi.md#albums_id_relationships_genres_get) | **GET** /albums/{id}/relationships/genres | Get genres relationship (\"to-many\").
[**albums_id_relationships_items_get**](AlbumsApi.md#albums_id_relationships_items_get) | **GET** /albums/{id}/relationships/items | Get items relationship (\"to-many\").
[**albums_id_relationships_owners_get**](AlbumsApi.md#albums_id_relationships_owners_get) | **GET** /albums/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**albums_id_relationships_providers_get**](AlbumsApi.md#albums_id_relationships_providers_get) | **GET** /albums/{id}/relationships/providers | Get providers relationship (\"to-many\").
[**albums_id_relationships_similar_albums_get**](AlbumsApi.md#albums_id_relationships_similar_albums_get) | **GET** /albums/{id}/relationships/similarAlbums | Get similarAlbums relationship (\"to-many\").
[**albums_post**](AlbumsApi.md#albums_post) | **POST** /albums | Create single album.



## albums_get

> models::AlbumsMultiResourceDataDocument albums_get(country_code, page_left_square_bracket_cursor_right_square_bracket, include, filter_left_square_bracket_owners_period_id_right_square_bracket, filter_left_square_bracket_id_right_square_bracket, filter_left_square_bracket_barcode_id_right_square_bracket)
Get multiple albums.

Retrieves multiple albums by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists, coverArt, genres, items, owners, providers, similarAlbums |  |
**filter_left_square_bracket_owners_period_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | User id |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Album id |  |
**filter_left_square_bracket_barcode_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Barcode Id |  |

### Return type

[**models::AlbumsMultiResourceDataDocument**](Albums_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_delete

> albums_id_delete(id)
Delete single album.

Deletes existing album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_get

> models::AlbumsSingleResourceDataDocument albums_id_get(id, country_code, include)
Get single album.

Retrieves single album by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists, coverArt, genres, items, owners, providers, similarAlbums |  |

### Return type

[**models::AlbumsSingleResourceDataDocument**](Albums_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_patch

> albums_id_patch(id, album_update_operation_payload)
Update single album.

Updates existing album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**album_update_operation_payload** | Option<[**AlbumUpdateOperationPayload**](AlbumUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_artists_get

> models::AlbumsMultiRelationshipDataDocument albums_id_relationships_artists_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get artists relationship (\"to-many\").

Retrieves artists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: artists |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_cover_art_get

> models::AlbumsMultiRelationshipDataDocument albums_id_relationships_cover_art_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get coverArt relationship (\"to-many\").

Retrieves coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: coverArt |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_cover_art_patch

> albums_id_relationships_cover_art_patch(id, album_cover_art_relationship_update_operation_payload)
Update coverArt relationship (\"to-many\").

Updates coverArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**album_cover_art_relationship_update_operation_payload** | Option<[**AlbumCoverArtRelationshipUpdateOperationPayload**](AlbumCoverArtRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_genres_get

> models::AlbumsMultiRelationshipDataDocument albums_id_relationships_genres_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get genres relationship (\"to-many\").

Retrieves genres relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: genres |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_items_get

> models::AlbumsItemsMultiRelationshipDataDocument albums_id_relationships_items_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get items relationship (\"to-many\").

Retrieves items relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: items |  |

### Return type

[**models::AlbumsItemsMultiRelationshipDataDocument**](Albums_Items_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_owners_get

> models::AlbumsMultiRelationshipDataDocument albums_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_providers_get

> models::AlbumsMultiRelationshipDataDocument albums_id_relationships_providers_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get providers relationship (\"to-many\").

Retrieves providers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: providers |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_id_relationships_similar_albums_get

> models::AlbumsMultiRelationshipDataDocument albums_id_relationships_similar_albums_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get similarAlbums relationship (\"to-many\").

Retrieves similarAlbums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Album id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: similarAlbums |  |

### Return type

[**models::AlbumsMultiRelationshipDataDocument**](Albums_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## albums_post

> models::AlbumsSingleResourceDataDocument albums_post(album_create_operation_payload)
Create single album.

Creates a new album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_create_operation_payload** | Option<[**AlbumCreateOperationPayload**](AlbumCreateOperationPayload.md)> |  |  |

### Return type

[**models::AlbumsSingleResourceDataDocument**](Albums_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

