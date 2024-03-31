use std::io;

#[derive(impl_new::New)]
pub struct UserInteraction {}

impl UserInteraction {
    pub fn read_string(&self) -> String {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        return guess.trim().to_string();
    }

    pub fn read_int(&self) -> u8 {
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
}

mod user_mode {

}