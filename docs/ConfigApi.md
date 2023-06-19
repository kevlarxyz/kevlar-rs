# \ConfigApi

All URIs are relative to *https://localhost:9200/v1/vault-ethereum*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gettheconfiguration**](ConfigApi.md#gettheconfiguration) | **GET** /config | get the configuration
[**writeaconfiguration**](ConfigApi.md#writeaconfiguration) | **PUT** /config | write a configuration



## gettheconfiguration

> gettheconfiguration(accept, x_vault_request, x_vault_token)
get the configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | [**&str**](.md) |  | [required] |
**x_vault_request** | [**&str**](.md) |  | [required] |
**x_vault_token** | [**&str**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## writeaconfiguration

> writeaconfiguration(accept, x_vault_request, x_vault_token, body)
write a configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept** | [**&str**](.md) |  | [required] |
**x_vault_request** | [**&str**](.md) |  | [required] |
**x_vault_token** | [**&str**](.md) |  | [required] |
**body** | Option<**&str**> |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
