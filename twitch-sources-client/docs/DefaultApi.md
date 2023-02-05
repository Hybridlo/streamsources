# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_generate_login_token_get**](DefaultApi.md#api_generate_login_token_get) | **GET** /api/generate_login_token | 
[**api_login_check_get**](DefaultApi.md#api_login_check_get) | **GET** /api/login_check | 
[**api_request_login_get**](DefaultApi.md#api_request_login_get) | **GET** /api/request_login | 
[**api_test_get**](DefaultApi.md#api_test_get) | **GET** /api/test | 



## api_generate_login_token_get

> crate::models::LoginTokenResponse api_generate_login_token_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoginTokenResponse**](LoginTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_login_check_get

> crate::models::UserInfo api_login_check_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserInfo**](UserInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_request_login_get

> crate::models::LoginUrlResponse api_request_login_get(callback_url)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callback_url** | **String** |  | [required] |

### Return type

[**crate::models::LoginUrlResponse**](LoginUrlResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_test_get

> serde_json::Value api_test_get(test)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

