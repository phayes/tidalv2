# VideosAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<**Vec<String>**> | Available usage for this video | [optional]
**copyright** | Option<[**models::Copyright**](Copyright.md)> |  | [optional]
**duration** | **String** | Duration (ISO 8601) | 
**explicit** | **bool** | Explicit content | 
**external_links** | Option<[**Vec<models::ExternalLink>**](External_Link.md)> | Video links external to TIDAL API | [optional]
**isrc** | **String** | International Standard Recording Code (ISRC) | 
**popularity** | **f64** | Popularity (0.0 - 1.0) | 
**release_date** | Option<[**String**](string.md)> | Release date (ISO-8601) | [optional]
**title** | **String** | Video title | 
**version** | Option<**String**> | Video version, complements title | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


