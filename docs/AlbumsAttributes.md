# AlbumsAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<**Vec<String>**> | Available usage for this album | [optional]
**barcode_id** | **String** | Barcode id (EAN-13 or UPC-A) | 
**copyright** | Option<[**models::Copyright**](Copyright.md)> |  | [optional]
**duration** | **String** | Duration (ISO 8601) | 
**explicit** | **bool** | Explicit content | 
**external_links** | Option<[**Vec<models::ExternalLink>**](External_Link.md)> | Album links external to TIDAL API | [optional]
**media_tags** | **Vec<String>** |  | 
**number_of_items** | **i32** | Number of items in album | 
**number_of_volumes** | **i32** | Number of volumes | 
**popularity** | **f64** | Popularity (0.0 - 1.0) | 
**release_date** | Option<[**String**](string.md)> | Release date (ISO-8601) | [optional]
**title** | **String** | Album title | 
**r#type** | **String** | Album type | 
**version** | Option<**String**> | Album version | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


