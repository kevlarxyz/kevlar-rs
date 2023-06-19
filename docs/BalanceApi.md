# \BalanceApi

All URIs are relative to *https://localhost:9200/v1/vault-ethereum*

Method | HTTP request | Description
------------- | ------------- | -------------
[**readbalance**](BalanceApi.md#readbalance) | **GET** /accounts/{account-name}/balance | read balance



## readbalance

> readbalance(account_name, accept, x_vault_request, x_vault_token)
read balance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | [**&str**](.md) |  | [required] |
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

