use converter_waller::classes::{
    cli::{Cli},
    json_parser::{JsonParser, WalletData}
};

fn main() {
    let mut cli = Cli::new();

    cli.add_wallet_from_menu();
}
