mod classes {
    pub mod user;
    pub mod json;
    mod Json;
}

use classes::json::{ JsonParser, WalletData };
use classes::user::UserInteraction;

fn main() {
    let json_parser_instance: JsonParser = JsonParser::new();

    println!("{:?}", json_parser_instance.display_wallet_list());
    // println!("{:?}", json_parser_instance.display_wallet_list());

    // let wallet_to_convert: String = UserInteraction::read_wallet();
    // println!("Entered wallet: {}", wallet_to_convert);

    // let wallet_to_convert_amount: u8 = UserInteraction::read_amount();
    // println!("Entered wallet: {}", wallet_to_convert_amount);

    // let wallets_to_convert = classes::UserInteraction::read_wallets(2);
    // println!("Entered wallet: {:?}", wallets_to_convert);

    // let json_data: Result<_, Box<dyn std::error::Error>> = ;
    
    // println!("{:?}", wallets);

    // if let Ok(Value::Array(array)) = json_data {
    //     if let Some(element) = array.first() {
    //         println!("Code: {}", element["code"].as_str().unwrap());
    //         println!("Name: {}", element["name"].as_str().unwrap());
    //     } else {
    //         println!("Array is empty");
    //     }
    // } else {
    //     eprintln!("Error: Failed to process JSON data");
    // }
}
