# \BalanceOfApi

All URIs are relative to *https://localhost:9200/v1/vault-ethereum*

Method | HTTP request | Description
------------- | ------------- | -------------
[**erc721balanceof**](BalanceOfApi.md#erc721balanceof) | **GET** /accounts/{account-name}/erc721/balanceOf | erc721 balance of
[**geterc20balance**](BalanceOfApi.md#geterc20balance) | **POST** /accounts/{account-name}/erc20/balanceOf | get erc20 balance



## erc721balanceof

> erc721balanceof(account_name, accept)
erc721 balance of

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


## geterc20balance

> geterc20balance(account_name, accept)
get erc20 balance

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

