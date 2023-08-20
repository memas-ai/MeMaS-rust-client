# \DpApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**recollect**](DpApi.md#recollect) | **POST** /dp/recollect | Recollects
[**remember**](DpApi.md#remember) | **POST** /dp/remember | Memorize information



## recollect

> Vec<crate::models::CitedDocument> recollect(recollect_request)
Recollects



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recollect_request** | [**RecollectRequest**](RecollectRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::CitedDocument>**](CitedDocument.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remember

> crate::models::Remember200Response remember(remember_request)
Memorize information

Memorize information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remember_request** | [**RememberRequest**](RememberRequest.md) | Request object for remembering a document | [required] |

### Return type

[**crate::models::Remember200Response**](remember_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

