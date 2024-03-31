use converter_waller::classes::{
    cli::{self, Cli}, json::{ JsonParser, WalletData }, user::UserInteraction
};

fn main() {
    let mut cli = Cli::new();

    cli.add_wallet_from_menu();

    // println!("{:?}", );
}
