mod classes {
    pub mod user;
    pub mod json;
    pub mod Json;
}

use classes::{json::{ JsonParser, WalletData }, user::UserInteraction};

fn main() {
    let json_parser_instance: JsonParser = JsonParser::new();
    let user_instance: UserInteraction = UserInteraction::new();

    loop {
        println!("\n");
        println!("Select Menu element");
        println!("1. Get wallets list");
        println!("2. Search by wallet");
        let enter_index: u8 = user_instance.read_int();

        match enter_index {
            1 => println!("{:?}", json_parser_instance.get_json_btree()),
            2 => {
                let input_value = user_instance.read_string().clone();
                let wallet = json_parser_instance.search_wallet(input_value.as_str());

                println!("{:?}", wallet);

                
            },
            _ => {
                println!("Provide wrong number");
                break;
            },
        }
    }
}
