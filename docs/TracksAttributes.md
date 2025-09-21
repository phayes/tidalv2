# TracksAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_type** | Option<**String**> | Access type | [optional]
**availability** | Option<**Vec<String>**> | Available usage for this track | [optional]
**bpm** | Option<**f32**> | Beats per minute | [optional]
**copyright** | Option<[**models::Copyright**](Copyright.md)> |  | [optional]
**created_at** | Option<**String**> | Datetime of track creation (ISO 8601) | [optional]
**duration** | **String** | Duration (ISO 8601) | 
**explicit** | **bool** | Explicit content | 
**external_links** | Option<[**Vec<models::ExternalLink>**](External_Link.md)> | Track links external to TIDAL API | [optional]
**isrc** | **String** | International Standard Recording Code (ISRC) | 
**key** | **String** | Key | 
**key_scale** | **String** | The scale of the key | 
**media_tags** | **Vec<String>** |  | 
**popularity** | **f64** | Popularity (0.0 - 1.0) | 
**spotlighted** | Option<**bool**> | Is the track spotlighted? | [optional]
**title** | **String** | Track title | 
**tone_tags** | Option<**Vec<String>**> |  | [optional]
**version** | Option<**String**> | Track version, complements title | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


