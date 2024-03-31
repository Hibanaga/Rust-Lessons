use std::io;

pub struct UserInteraction {}

impl UserInteraction {
    pub fn read_wallet() -> String {
        println!("Enter a wallet: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        return guess.trim().to_string();
    }

    pub fn read_amount() -> u8 {
        println!("Enter a amount: ");
        let mut string = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read line");

        let trimmed = string.trim();
        match trimmed.parse::<u8>() {
            Ok(i) => i,
            Err(..) => panic!("Passed wrong data"),
        }
    }
    
    pub fn read_wallets(count: u8) -> Vec<String> {
        let mut vec_wallets = Vec::with_capacity(count as usize);

        let mut i = 0;
        while i < count {
            let wallet = Self::read_wallet();

            vec_wallets.push(wallet);
            i += 1;
        }

        return vec_wallets;
    }
}

mod user_mode {

}