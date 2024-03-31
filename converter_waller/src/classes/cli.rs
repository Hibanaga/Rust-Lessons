use std::collections::BTreeMap;

use super::{
    json_parser::{JsonParser, WalletData}, 
    user::UserInteraction
};

pub struct Cli {
    json: JsonParser,
    user: UserInteraction,
    convert_from: WalletData,
    convert_to: Vec<WalletData>,
}

impl Cli {
   pub fn new() -> Self {
        Cli {
            json: JsonParser::new(), 
            user: UserInteraction::new(),
            convert_from: WalletData{ code: String::from(""), name: String::from("") },
            convert_to: Vec::new(),
        }
    }

    pub fn main_menu(&mut self) {
        loop {
            println!("\n");
            println!("Welcome to wallet converter CLI. Please pick a option.");

            let convert_from = &self.convert_from;
            if convert_from.code != "" {
                println!("\n");
                println!("CONVERT FROM: code: {:?}, name: {:?}", convert_from.code, convert_from.name);
                println!("\n");
            }

            println!("1. Pick wallet 'CONVERT FROM'");
            println!("2. Add wallet 'CONVERT TO'");
            println!("2. Modify wallet 'CONVERT TO'");
            println!("3. Remove wallet from vec 'CONVERT TO'");

            let enter_index: u8 = self.user.read_int();
            match enter_index {
                0 => self.add_wallet_from_menu(),
                1 => self.add_wallet_to_menu(),
                _ => {
                    println!("Opss... You provide something what can't process");
                    break;
                },
            }
            match enter_index {
                1 => self.add_wallet_from_menu(),
                _ => {
                    println!("Provide wrong number");
                    break;
                },
            }
        }
    }

    pub fn add_wallet_from_menu(&mut self) {
        loop {
            let _ = self.convert_from.code != "" && break;

            println!("\n");
            println!("You picked 'CONVERT FROM' option wallet. You can pick only 1 option.");
            println!("1. Get list of existing wallets");
            println!("2. Search by name or wallet code (Search working by LIKE, and return only one result).");
            println!("3. Add wallet by idx");

            let enter_index: u8 = self.user.read_int();
            match enter_index {
                1 => Self::display_wallets(self.json.get_json_btree()),
                2 => {
                    loop {
                        println!("Please enter name or wallet code");
                        let search_input: String = self.user.read_string();

                        let picked_wallet: WalletData = self.json.search_wallet(search_input.as_str());
                        if picked_wallet.name != "" {
                            println!("\n");
                            println!("You picked wallet 'CONVERT_FROM' is code: {:?} | name: {:?}", picked_wallet.code, picked_wallet.name);
                            self.convert_from = picked_wallet;
                            break;
                        } else {
                            println!("Try again. Because you provide unsupportable wallet information");
                        }
                    }
                },
                3 => {
                    loop {
                        println!("Please enter id of searhing wallet from list(Pick number from 0 to {:?})", self.json.list_wallets().len() - 1);
                        let search_idx: u8 = self.user.read_int();

                        let picked_wallet: WalletData = self.json.search_wallet_by_idx(search_idx);
                        if picked_wallet.name != "" {
                            println!("\n");
                            println!("You picked wallet 'CONVERT_FROM' is code: {:?} | name: {:?}", picked_wallet.code, picked_wallet.name);
                            self.convert_from = picked_wallet;
                            break;
                        } else {
                            println!("Try again. Because you provide unsupportable wallet information");
                        }
                    }
                },
                _ => {
                    println!("Opss... You provide something what can't process");
                    break;
                },
            }
        }
    }

    pub fn add_wallet_to_menu(&self) {

    }

    fn display_wallets(wallets: BTreeMap<usize, WalletData>) {
        println!("_________________________________________________");
        for (idx, wallet) in wallets {
            println!("id: {:?} | code: {:?} | name: {:?}", idx, wallet.code, wallet.name);
        }
        println!("__________________________________________________");  
    }
}