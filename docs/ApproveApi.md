# \ApproveApi

All URIs are relative to *https://localhost:9200/v1/vault-ethereum*

Method | HTTP request | Description
------------- | ------------- | -------------
[**erc20approve**](ApproveApi.md#erc20approve) | **POST** /accounts/{account-name}/erc20/approve | erc20 approve
[**erc721approve**](ApproveApi.md#erc721approve) | **POST** /accounts/{account-name}/erc721/approve | erc721 approve



## erc20approve

> erc20approve(account_name, accept)
erc20 approve

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | [**&str**](.md) |  | [required] |
**accept** | [**&str**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## erc721approve

> erc721approve(account_name, accept)
erc721 approve

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | [**&str**](.md) |  | [required] |
**accept** | [**&str**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

