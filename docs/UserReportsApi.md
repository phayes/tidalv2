# \UserReportsApi

All URIs are relative to *https://openapi.tidal.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_reports_post**](UserReportsApi.md#user_reports_post) | **POST** /userReports | Create single userReport.



## user_reports_post

> models::UserReportsSingleResourceDataDocument user_reports_post(user_report_create_operation_payload)
Create single userReport.

Creates a new userReport.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_report_create_operation_payload** | Option<[**UserReportCreateOperationPayload**](UserReportCreateOperationPayload.md)> |  |  |

### Return type

[**models::UserReportsSingleResourceDataDocument**](UserReports_Single_Resource_Data_Document.md)

### Authorization

[Authorization_Code_PKCE](../README.md#Authorization_Code_PKCE)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

