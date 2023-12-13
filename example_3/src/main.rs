use indexmap::IndexMap;

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

fn two_sums(array: Vec<u32>, target: u32) -> [u8; 2] {
    for (i, _el) in array.iter().enumerate() {
        if  array[i] + array[i + 1] == target {
            return [i.try_into().unwrap(), (i + 1).try_into().unwrap()];
        }
    }

    return [0, 0];
}

fn is_palindrome(first_number: u32) -> bool {
    let str = first_number.to_string();
    return str.chars().rev().collect::<String>() == str
}

fn integer_to_roman(mut number: u32) -> String {
    let mut roman_number: String = "".to_string();
    let roman_table: IndexMap<&str, u32> = IndexMap::from([
        ("M", 1000),
        ("D", 500),
        ("C", 100),
        ("L", 50),
        ("X", 10),
        ("V", 5),
        ("I", 1),
    ])
    .into_iter()
    .collect();

    while number > 0 {
        for (key, value) in roman_table.iter() {
            if number >= *value {
                number -= value;
                roman_number += key;
                break;
            };
        }
    }

    return roman_number.to_string();
}

fn longest_common_prefix(mut array: Vec<&str>) -> String {
    array.sort();

    let first_string = array[0];
    let last_string = array[array.len() - 1];
    let mut idx = 0;

    while idx < first_string.len() && idx < last_string.len() {
        if first_string.chars().nth(idx) == last_string.chars().nth(idx) {
            idx += 1;
        } else {
            break;
        }
    }
    
    return first_string[..idx].to_string();
}


fn main() {
    println!("Code wars coding");
    println!("Maskify result: {:?}", maskify(String::from("3221-4432-1231-5412")));
    println!("Two sums result: {:?}", two_sums([15, 24, 12, 43].to_vec(), 36));

    println!("Is palindrome result: {:?}", is_palindrome(121));
    println!("Is Integer to roman result: {:?}", integer_to_roman(142));
    println!("Longest prefix result: {:?}", longest_common_prefix(["flower", "flow", "flight"].to_vec()));
}
