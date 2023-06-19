use kevlar::apis::{account_name_api::createanaccount, configuration::Configuration};

use serde_json::Value;

#[tokio::main]
async fn main() {
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

            println!("Account Created: \n Address: {} \n", data["address"])
        }
        Err(e) => println!("Error creating account: {:?}", e),
    }
}
