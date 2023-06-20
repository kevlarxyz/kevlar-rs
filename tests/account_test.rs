use kevlar_rs::apis::{account_name_api::createanaccount, configuration::Configuration};

use ethers::types::Address;
use serde_json::Value;

#[tokio::test]
async fn create_account() {
    // create a new config
    let config = Configuration::new();

    // create an account
    let account =
        createanaccount(&config, "accountName", "application/json", "true", "VAULT_TOKEN", None)
            .await;

    // unpack the result
    match account {
        Ok(_) => {
            let account = account.unwrap().to_string();
            let json_value: Value = serde_json::from_str(&account).unwrap();
            let data = &json_value["data"];

            println!("Account Created: \n Address: {} \n", data["address"]);
            let result: bool = is_valid_eth_address(&data["address"].to_string());
            assert!(result);
        }
        Err(e) => println!("Error creating account: {:?}", e),
    }
}

fn is_valid_eth_address(s: &str) -> bool {
    let address = s.trim_matches('"');
    if let Ok(_address) = address.parse::<Address>() {
        return true;
    }
    false
}
