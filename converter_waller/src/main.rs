use converter_waller::classes::{
    json::{ JsonParser, WalletData }, 
    user::{ UserInteraction },
};

fn main() {
    let wallet_from_convert: Vec<WalletData> = Vec::new();
    let wallet_to_convert: Vec<WalletData> = Vec::new();

    let json_parser_instance: JsonParser = JsonParser::new();
    let user_instance: UserInteraction = UserInteraction::new();


    loop {
        println!("\n");
    }

    // loop {
    //     println!("\n");
    //     println!("Welcome to wallet converter CLI. Please pick a option.");
    //     println!("1. Get wallets list");
    //     println!("2. Search by wallet");
    //     let enter_index: u8 = user_instance.read_int();

    //     match enter_index {
    //         1 => println!("{:?}", json_parser_instance.get_json_btree()),
    //         2 => {
    //             let input_value = user_instance.read_string().clone();
    //             println!("Enter country name or currency code. Search working by LIKE.");
    //             let wallet = json_parser_instance.search_wallet(input_value.as_str());
    //             println!("Result: code: {:?}, name: {:?}", wallet.code, wallet.name);

    //             if !wallet.name.contains("Not Found") {
    //                 println!("1. Get wallets list");
    //                 println!("2. Search by wallet");
    //             }
                
    //         },
    //         _ => {
    //             println!("Provide wrong number");
    //             break;
    //         },
    //     }
    // }
}
