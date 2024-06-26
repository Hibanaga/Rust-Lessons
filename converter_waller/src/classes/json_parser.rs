use std::{collections::BTreeMap, fs};

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

    pub fn get_json_btree(&self) -> BTreeMap<usize, WalletData> {
        let mut wallet_map: BTreeMap<usize, WalletData> = BTreeMap::new();
        let mut index = 0;
        for wallet in Self::list_wallets(&self) {
            wallet_map.insert(index, wallet);
            index += 1;
        }

        return wallet_map;
    }

    pub fn search_wallet(&self, search: &str)-> WalletData {
        let wallets = Self::list_wallets(&self);
        let lower_case_search = String::from(search.to_lowercase());

        return match wallets.iter()
            .position(|wallet| wallet.code.to_lowercase().starts_with(&lower_case_search) || wallet.name.to_lowercase().starts_with(&lower_case_search)) {
                Some(index) => wallets[index].clone(),
                None => WalletData {
                    code: String::from(""),
                    name: String::from(""),
                },
        };
    }


    pub fn search_wallet_by_idx(&self, idx: u8)-> WalletData {
        let wallets = Self::list_wallets(&self);

        return match wallets.get(idx as usize) {
            Some(wallet) => wallet.clone(),
            None => WalletData {
                code: String::from(""),
                name: String::from(""),
            },
        }
    }
}