# \TransferApi

All URIs are relative to *https://localhost:9200/v1/vault-ethereum*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transfer_eth**](TransferApi.md#transfer_eth) | **PUT** /accounts/{account-name}/transfer | transfer ETH
[**transferanerc20**](TransferApi.md#transferanerc20) | **POST** /accounts/{account-name}/erc20/transfer | transfer an erc20



## transfer_eth

> transfer_eth(account_name, accept, x_vault_request, x_vault_token, body)
transfer ETH

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_name** | [**&str**](.md) |  | [required] |
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


## transferanerc20

> transferanerc20(account_name, accept)
transfer an erc20

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

