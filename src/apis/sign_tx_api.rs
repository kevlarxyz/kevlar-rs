/*
 * Vault Ethereum Plugin
 *
 * This is a Vault plugin that allows for the creation of Ethereum accounts and signing of
 * transactions using those accounts. Contact Support:  Name: @kevlarxyz  Email:
 * support@kevlar.xyz
 *
 * The version of the kevlar document: 1.0
 *
 * Generated by: https://kevlar-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`signatransaction`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

pub enum SignatransactionError {
    UnknownValue(serde_json::Value),
}

pub async fn signatransaction(
    configuration: &configuration::Configuration,
    account_name: &str,
    accept: &str,
    x_vault_request: &str,
    x_vault_token: &str,
    body: Option<&str>,
) -> Result<String, Error<SignatransactionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str =
        format!("{}/accounts/{}/sign-tx", local_var_configuration.base_path, account_name);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Accept", accept.to_string());
    local_var_req_builder =
        local_var_req_builder.header("X-VAULT_REQUEST", x_vault_request.to_string());
    local_var_req_builder =
        local_var_req_builder.header("X-VAULT-TOKEN", x_vault_token.to_string());
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(local_var_content.to_string())
    } else {
        let local_var_entity: Option<SignatransactionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
