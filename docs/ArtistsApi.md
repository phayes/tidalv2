# \ArtistsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**artists_get**](ArtistsApi.md#artists_get) | **GET** /artists | Get multiple artists.
[**artists_id_get**](ArtistsApi.md#artists_id_get) | **GET** /artists/{id} | Get single artist.
[**artists_id_patch**](ArtistsApi.md#artists_id_patch) | **PATCH** /artists/{id} | Update single artist.
[**artists_id_relationships_albums_get**](ArtistsApi.md#artists_id_relationships_albums_get) | **GET** /artists/{id}/relationships/albums | Get albums relationship (\"to-many\").
[**artists_id_relationships_biography_get**](ArtistsApi.md#artists_id_relationships_biography_get) | **GET** /artists/{id}/relationships/biography | Get biography relationship (\"to-one\").
[**artists_id_relationships_followers_get**](ArtistsApi.md#artists_id_relationships_followers_get) | **GET** /artists/{id}/relationships/followers | Get followers relationship (\"to-many\").
[**artists_id_relationships_following_delete**](ArtistsApi.md#artists_id_relationships_following_delete) | **DELETE** /artists/{id}/relationships/following | Delete from following relationship (\"to-many\").
[**artists_id_relationships_following_get**](ArtistsApi.md#artists_id_relationships_following_get) | **GET** /artists/{id}/relationships/following | Get following relationship (\"to-many\").
[**artists_id_relationships_following_post**](ArtistsApi.md#artists_id_relationships_following_post) | **POST** /artists/{id}/relationships/following | Add to following relationship (\"to-many\").
[**artists_id_relationships_owners_get**](ArtistsApi.md#artists_id_relationships_owners_get) | **GET** /artists/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**artists_id_relationships_profile_art_get**](ArtistsApi.md#artists_id_relationships_profile_art_get) | **GET** /artists/{id}/relationships/profileArt | Get profileArt relationship (\"to-many\").
[**artists_id_relationships_profile_art_patch**](ArtistsApi.md#artists_id_relationships_profile_art_patch) | **PATCH** /artists/{id}/relationships/profileArt | Update profileArt relationship (\"to-many\").
[**artists_id_relationships_radio_get**](ArtistsApi.md#artists_id_relationships_radio_get) | **GET** /artists/{id}/relationships/radio | Get radio relationship (\"to-many\").
[**artists_id_relationships_roles_get**](ArtistsApi.md#artists_id_relationships_roles_get) | **GET** /artists/{id}/relationships/roles | Get roles relationship (\"to-many\").
[**artists_id_relationships_similar_artists_get**](ArtistsApi.md#artists_id_relationships_similar_artists_get) | **GET** /artists/{id}/relationships/similarArtists | Get similarArtists relationship (\"to-many\").
[**artists_id_relationships_track_providers_get**](ArtistsApi.md#artists_id_relationships_track_providers_get) | **GET** /artists/{id}/relationships/trackProviders | Get trackProviders relationship (\"to-many\").
[**artists_id_relationships_tracks_get**](ArtistsApi.md#artists_id_relationships_tracks_get) | **GET** /artists/{id}/relationships/tracks | Get tracks relationship (\"to-many\").
[**artists_id_relationships_videos_get**](ArtistsApi.md#artists_id_relationships_videos_get) | **GET** /artists/{id}/relationships/videos | Get videos relationship (\"to-many\").
[**artists_post**](ArtistsApi.md#artists_post) | **POST** /artists | Create single artist.



## artists_get

> models::ArtistsMultiResourceDataDocument artists_get(country_code, include, filter_left_square_bracket_handle_right_square_bracket, filter_left_square_bracket_id_right_square_bracket)
Get multiple artists.

Retrieves multiple artists by available filters, or without if applicable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, biography, followers, following, owners, profileArt, radio, roles, similarArtists, trackProviders, tracks, videos |  |
**filter_left_square_bracket_handle_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist handle |  |
**filter_left_square_bracket_id_right_square_bracket** | Option<[**Vec<String>**](String.md)> | Artist id |  |

### Return type

[**models::ArtistsMultiResourceDataDocument**](Artists_Multi_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_get

> models::ArtistsSingleResourceDataDocument artists_id_get(id, country_code, include)
Get single artist.

Retrieves single artist by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums, biography, followers, following, owners, profileArt, radio, roles, similarArtists, trackProviders, tracks, videos |  |

### Return type

[**models::ArtistsSingleResourceDataDocument**](Artists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_patch

> artists_id_patch(id, artist_update_body)
Update single artist.

Updates existing artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_update_body** | Option<[**ArtistUpdateBody**](ArtistUpdateBody.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_albums_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_albums_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get albums relationship (\"to-many\").

Retrieves albums relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: albums |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_biography_get

> models::ArtistsSingleRelationshipDataDocument artists_id_relationships_biography_get(id, country_code, include)
Get biography relationship (\"to-one\").

Retrieves biography relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: biography |  |

### Return type

[**models::ArtistsSingleRelationshipDataDocument**](Artists_Single_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_followers_get

> models::ArtistsFollowersMultiRelationshipDataDocument artists_id_relationships_followers_get(id, viewer_context, page_left_square_bracket_cursor_right_square_bracket, include)
Get followers relationship (\"to-many\").

Retrieves followers relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**viewer_context** | Option<**String**> |  |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: followers |  |

### Return type

[**models::ArtistsFollowersMultiRelationshipDataDocument**](Artists_Followers_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_following_delete

> artists_id_relationships_following_delete(id, artist_following_relationship_remove_operation_payload)
Delete from following relationship (\"to-many\").

Deletes item(s) from following relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_following_relationship_remove_operation_payload** | Option<[**ArtistFollowingRelationshipRemoveOperationPayload**](ArtistFollowingRelationshipRemoveOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_following_get

> models::ArtistsFollowingMultiRelationshipDataDocument artists_id_relationships_following_get(id, viewer_context, page_left_square_bracket_cursor_right_square_bracket, include)
Get following relationship (\"to-many\").

Retrieves following relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**viewer_context** | Option<**String**> |  |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: following |  |

### Return type

[**models::ArtistsFollowingMultiRelationshipDataDocument**](Artists_Following_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_following_post

> artists_id_relationships_following_post(id, country_code, artist_following_relationship_add_operation_payload)
Add to following relationship (\"to-many\").

Adds item(s) to following relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**artist_following_relationship_add_operation_payload** | Option<[**ArtistFollowingRelationshipAddOperationPayload**](ArtistFollowingRelationshipAddOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_owners_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_profile_art_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_profile_art_get(id, country_code, include, page_left_square_bracket_cursor_right_square_bracket)
Get profileArt relationship (\"to-many\").

Retrieves profileArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: profileArt |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_profile_art_patch

> artists_id_relationships_profile_art_patch(id, artist_profile_art_relationship_update_operation_payload)
Update profileArt relationship (\"to-many\").

Updates profileArt relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**artist_profile_art_relationship_update_operation_payload** | Option<[**ArtistProfileArtRelationshipUpdateOperationPayload**](ArtistProfileArtRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_radio_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_radio_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get radio relationship (\"to-many\").

Retrieves radio relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: radio |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_roles_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_roles_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get roles relationship (\"to-many\").

Retrieves roles relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: roles |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_similar_artists_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_similar_artists_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get similarArtists relationship (\"to-many\").

Retrieves similarArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: similarArtists |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_track_providers_get

> models::ArtistsTrackProvidersMultiRelationshipDataDocument artists_id_relationships_track_providers_get(id, page_left_square_bracket_cursor_right_square_bracket, include)
Get trackProviders relationship (\"to-many\").

Retrieves trackProviders relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: trackProviders |  |

### Return type

[**models::ArtistsTrackProvidersMultiRelationshipDataDocument**](Artists_TrackProviders_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_tracks_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_tracks_get(id, country_code, collapse_by, page_left_square_bracket_cursor_right_square_bracket, include)
Get tracks relationship (\"to-many\").

Retrieves tracks relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**collapse_by** | **String** | Collapse by options for getting artist tracks. Available options: FINGERPRINT, ID. FINGERPRINT option might collapse similar tracks based entry fingerprints while collapsing by ID always returns all available items. | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: tracks |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_id_relationships_videos_get

> models::ArtistsMultiRelationshipDataDocument artists_id_relationships_videos_get(id, country_code, page_left_square_bracket_cursor_right_square_bracket, include)
Get videos relationship (\"to-many\").

Retrieves videos relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist id | [required] |
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: videos |  |

### Return type

[**models::ArtistsMultiRelationshipDataDocument**](Artists_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE), [Client_Credentials](../README.md#Client_Credentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artists_post

> models::ArtistsSingleResourceDataDocument artists_post(artist_create_operation_payload)
Create single artist.

Creates a new artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_create_operation_payload** | Option<[**ArtistCreateOperationPayload**](ArtistCreateOperationPayload.md)> |  |  |

### Return type

[**models::ArtistsSingleResourceDataDocument**](Artists_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

