fn maskify(mut string: String) -> String {
    let len = string.len();

    if len <= 4 {
        return string;
    }
    
    let modified_string = "#".repeat(len - 4);
    string.drain(..len-4).for_each(drop);
    string.insert_str(0, &modified_string);

    return string;
}


fn main() {
    println!("Code wars coding");
    let original_string = String::from("Nananananananananananananananana Batman");
    println!("Maskify result: {:?}", maskify(original_string));
}
