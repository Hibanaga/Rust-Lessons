use std::{collections::HashMap, fs};

use serde_json::{from_str, Value};

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct WalletData {
    pub code: String,
    pub name: String,
}

#[derive(impl_new::New)]
pub struct JsonParser {}

impl JsonParser {
    const JSON_WALLET_PATH: &str = "src/consts/predefined_wallets.json";

    pub fn list_wallets(&self) -> Vec<WalletData> {
        lazy_static! {
            static ref LIST_WALLETS: Vec<WalletData> = JsonParser::read_wallet_json();
        }
        LIST_WALLETS.clone()
    }

    fn read_wallet_json()-> Vec<WalletData> {
        let contents = fs::read_to_string(Self::JSON_WALLET_PATH).expect("Error reading JSON file");
        let data: Value = from_str(&contents).expect("Error parsing JSON data");
        let mut wallets: Vec<WalletData> = Vec::new();

        println!("fff");

        if let Value::Array(array) = data {
            for element in array.iter() {
                if let Value::Object(json_map) = element {
                    let code = match json_map.get("code").and_then(Value::as_str) {
                        Some(code) => code.to_owned(),
                        None => panic!("Error by validation"),
                    };

                    let name = match json_map.get("name").and_then(Value::as_str) {
                        Some(name) => name.to_owned(),
                        None => panic!("Error by validation"),
                    };

                    wallets.push(WalletData{code, name});
                }
            }
        } else {
            panic!("Invalid JSON format: expected an array");
        }

        return wallets;
    }

pub fn display_wallet_list(&self) -> HashMap<usize, WalletData> {
    let mut wallet_map: HashMap<usize, WalletData> = HashMap::new();
    let mut index = 0;
    for wallet in Self::list_wallets(&self) {
        wallet_map.insert(index, wallet);
        index += 1;
    }

    let mut entries: Vec<_> = wallet_map.drain().collect();

    entries.sort_by_key(|x| x.0);

    let sorted_map: HashMap<usize, WalletData> = entries.into_iter().collect();

    return sorted_map;
}

 }