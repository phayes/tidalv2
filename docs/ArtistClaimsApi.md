# \ArtistClaimsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**artist_claims_id_get**](ArtistClaimsApi.md#artist_claims_id_get) | **GET** /artistClaims/{id} | Get single artistClaim.
[**artist_claims_id_patch**](ArtistClaimsApi.md#artist_claims_id_patch) | **PATCH** /artistClaims/{id} | Update single artistClaim.
[**artist_claims_id_relationships_accepted_artists_get**](ArtistClaimsApi.md#artist_claims_id_relationships_accepted_artists_get) | **GET** /artistClaims/{id}/relationships/acceptedArtists | Get acceptedArtists relationship (\"to-many\").
[**artist_claims_id_relationships_accepted_artists_patch**](ArtistClaimsApi.md#artist_claims_id_relationships_accepted_artists_patch) | **PATCH** /artistClaims/{id}/relationships/acceptedArtists | Update acceptedArtists relationship (\"to-many\").
[**artist_claims_id_relationships_owners_get**](ArtistClaimsApi.md#artist_claims_id_relationships_owners_get) | **GET** /artistClaims/{id}/relationships/owners | Get owners relationship (\"to-many\").
[**artist_claims_id_relationships_recommended_artists_get**](ArtistClaimsApi.md#artist_claims_id_relationships_recommended_artists_get) | **GET** /artistClaims/{id}/relationships/recommendedArtists | Get recommendedArtists relationship (\"to-many\").
[**artist_claims_post**](ArtistClaimsApi.md#artist_claims_post) | **POST** /artistClaims | Create single artistClaim.



## artist_claims_id_get

> models::ArtistClaimsSingleResourceDataDocument artist_claims_id_get(id, include)
Get single artistClaim.

Retrieves single artistClaim by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: acceptedArtists, owners, recommendedArtists |  |

### Return type

[**models::ArtistClaimsSingleResourceDataDocument**](ArtistClaims_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_claims_id_patch

> artist_claims_id_patch(id, artist_claims_update_operation_payload)
Update single artistClaim.

Updates existing artistClaim.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**artist_claims_update_operation_payload** | Option<[**ArtistClaimsUpdateOperationPayload**](ArtistClaimsUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_claims_id_relationships_accepted_artists_get

> models::ArtistClaimsMultiRelationshipDataDocument artist_claims_id_relationships_accepted_artists_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get acceptedArtists relationship (\"to-many\").

Retrieves acceptedArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: acceptedArtists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistClaimsMultiRelationshipDataDocument**](ArtistClaims_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_claims_id_relationships_accepted_artists_patch

> artist_claims_id_relationships_accepted_artists_patch(id, artist_claim_accepted_artists_relationship_update_operation_payload)
Update acceptedArtists relationship (\"to-many\").

Updates acceptedArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**artist_claim_accepted_artists_relationship_update_operation_payload** | Option<[**ArtistClaimAcceptedArtistsRelationshipUpdateOperationPayload**](ArtistClaimAcceptedArtistsRelationshipUpdateOperationPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_claims_id_relationships_owners_get

> models::ArtistClaimsMultiRelationshipDataDocument artist_claims_id_relationships_owners_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get owners relationship (\"to-many\").

Retrieves owners relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: owners |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistClaimsMultiRelationshipDataDocument**](ArtistClaims_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_claims_id_relationships_recommended_artists_get

> models::ArtistClaimsMultiRelationshipDataDocument artist_claims_id_relationships_recommended_artists_get(id, include, page_left_square_bracket_cursor_right_square_bracket)
Get recommendedArtists relationship (\"to-many\").

Retrieves recommendedArtists relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Artist claim id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | Allows the client to customize which related resources should be returned. Available options: recommendedArtists |  |
**page_left_square_bracket_cursor_right_square_bracket** | Option<**String**> | Server-generated cursor value pointing a certain page of items. Optional, targets first page if not specified |  |

### Return type

[**models::ArtistClaimsMultiRelationshipDataDocument**](ArtistClaims_Multi_Relationship_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artist_claims_post

> models::ArtistClaimsSingleResourceDataDocument artist_claims_post(country_code, artist_claims_create_operation_payload)
Create single artistClaim.

Creates a new artistClaim.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code | [required] |[default to US]
**artist_claims_create_operation_payload** | Option<[**ArtistClaimsCreateOperationPayload**](ArtistClaimsCreateOperationPayload.md)> |  |  |

### Return type

[**models::ArtistClaimsSingleResourceDataDocument**](ArtistClaims_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

