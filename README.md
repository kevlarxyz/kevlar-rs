# kevlar-rs

This is a Rust library for interfacing with the Vault plugin that allows for the creation of EVM accounts and signing of transactions using those accounts.

## Overview

This library is providers rust bindings for interacting with the kevlar vault plugin. It is generated using the [OpenAPI Generator](https://openapi-generator.tech) project and references the `openapi.yaml` specification which can be found under `definitions/`. The generated code is under `src/api/` and `src/models/`.

- API version: 1.0
- Package version: 1.0

## Installation

Put the package under your project folder in a directory named `kevlar` and add the following to `Cargo.toml` under `[dependencies]`:

```
kevlar = { path = "./kevlar" }
```

## Contributing
We use various `.githooks` to maintain our codebase, docs, and wiki

Getting started with git hooks
```bash
git config core.hooksPath .githooks
chmod +x .githooks/*
```

- `Wiki Sync Hook` will sync the wiki with the `docs` directory on every commit

## Documentation for API Endpoints

All URIs are relative to *https://localhost:9200/v1/vault-ethereum* this can be changed in the configuration struct see `src/api/configuration.rs` for the default value.

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Examples and Tests 
There are a few examples and tests which can be run with 
```
make test # this tests the library and custom tests in the `tests` directory
```

```
make examples # this runs the examples in the `examples` directory
```

### OpenAPI Specification

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountNameApi* | [**createanaccount**](docs/AccountNameApi.md#createanaccount) | **PUT** /accounts/{account-name} | create an account
*AccountNameApi* | [**readanaccount**](docs/AccountNameApi.md#readanaccount) | **GET** /accounts/{account-name} | read an account
*AccountsApi* | [**listallaccounts**](docs/AccountsApi.md#listallaccounts) | **GET** /accounts | list all accounts
*ApproveApi* | [**erc20approve**](docs/ApproveApi.md#erc20approve) | **POST** /accounts/{account-name}/erc20/approve | erc20 approve
*ApproveApi* | [**erc721approve**](docs/ApproveApi.md#erc721approve) | **POST** /accounts/{account-name}/erc721/approve | erc721 approve
*BalanceApi* | [**readbalance**](docs/BalanceApi.md#readbalance) | **GET** /accounts/{account-name}/balance | read balance
*BalanceOfApi* | [**erc721balanceof**](docs/BalanceOfApi.md#erc721balanceof) | **GET** /accounts/{account-name}/erc721/balanceOf | erc721 balance of
*BalanceOfApi* | [**geterc20balance**](docs/BalanceOfApi.md#geterc20balance) | **POST** /accounts/{account-name}/erc20/balanceOf | get erc20 balance
*ConfigApi* | [**gettheconfiguration**](docs/ConfigApi.md#gettheconfiguration) | **GET** /config | get the configuration
*ConfigApi* | [**writeaconfiguration**](docs/ConfigApi.md#writeaconfiguration) | **PUT** /config | write a configuration
*ConvertApi* | [**getaconversion**](docs/ConvertApi.md#getaconversion) | **POST** /convert | get a conversion
*DeployApi* | [**deployasmartcontract**](docs/DeployApi.md#deployasmartcontract) | **GET** /accounts/{account-name}/deploy | deploy a smart contract
*GetApprovedApi* | [**erc721gettheapprovedaddressforasinglenft**](docs/GetApprovedApi.md#erc721gettheapprovedaddressforasinglenft) | **GET** /accounts/{account-name}/erc721/getApproved | erc721 get the approved address for a single nft
*IsApprovedForAllApi* | [**erc721checkifisapprovedforall**](docs/IsApprovedForAllApi.md#erc721checkifisapprovedforall) | **GET** /accounts/{account-name}/erc721/isApprovedForAll | erc721 check if is approved for all
*MetadataApi* | [**erc721getmetadata**](docs/MetadataApi.md#erc721getmetadata) | **GET** /accounts/{account-name}/erc721/metadata | erc721 get metadata
*OwnerOfApi* | [**geterc721safetransferfrom**](docs/OwnerOfApi.md#geterc721safetransferfrom) | **GET** /accounts/{account-name}/erc721/ownerOf | erc721 safe transfer from
*SafeTransferFromApi* | [**erc721safetransferfrom**](docs/SafeTransferFromApi.md#erc721safetransferfrom) | **POST** /accounts/{account-name}/erc721/safeTransferFrom | erc721 safe transfer from
*SetApprovalForAllApi* | [**erc721setapprovalforall**](docs/SetApprovalForAllApi.md#erc721setapprovalforall) | **POST** /accounts/{account-name}/erc721/setApprovalForAll | erc721 set approval for all
*SignApi* | [**signamessage**](docs/SignApi.md#signamessage) | **PUT** /accounts/{account-name}/sign | sign a message
*SignTxApi* | [**signatransaction**](docs/SignTxApi.md#signatransaction) | **PUT** /accounts/{account-name}/sign-tx | sign a transaction
*TokenByIndexApi* | [**erc721gettokenbyindex**](docs/TokenByIndexApi.md#erc721gettokenbyindex) | **GET** /accounts/{account-name}/erc721/tokenByIndex | erc721 get token by index
*TokenOfOwnerByIndexApi* | [**erc721tokenofownerbyindex**](docs/TokenOfOwnerByIndexApi.md#erc721tokenofownerbyindex) | **GET** /accounts/{account-name}/erc721/tokenOfOwnerByIndex | erc721 token of owner by index
*TokenUriApi* | [**erc721gettokenuri**](docs/TokenUriApi.md#erc721gettokenuri) | **GET** /accounts/{account-name}/erc721/tokenURI | erc721 get token uri
*TotalSupplyApi* | [**geterc20totalsupply**](docs/TotalSupplyApi.md#geterc20totalsupply) | **POST** /accounts/{account-name}/erc20/totalSupply | get erc20 total supply
*TransferApi* | [**transfer_eth**](docs/TransferApi.md#transfer_eth) | **PUT** /accounts/{account-name}/transfer | transfer ETH
*TransferApi* | [**transferanerc20**](docs/TransferApi.md#transferanerc20) | **POST** /accounts/{account-name}/erc20/transfer | transfer an erc20
*TransferFromApi* | [**transfererc20fromoneaddresstothis**](docs/TransferFromApi.md#transfererc20fromoneaddresstothis) | **POST** /accounts/{account-name}/erc20/transferFrom | transfer erc20 from one address to this


## Documentation For Models

 - [Createanaccountrequest](docs/Createanaccountrequest.md)
 - [Signamessagerequest](docs/Signamessagerequest.md)
 - [Signatransactionrequest](docs/Signatransactionrequest.md)
 - [TransferEthRequest](docs/TransferEthRequest.md)
 - [Updateanaccountrequest](docs/Updateanaccountrequest.md)
 - [Writeaconfigurationrequest](docs/Writeaconfigurationrequest.md)
