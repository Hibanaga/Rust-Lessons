use std::collections::HashSet;



fn my_sqrt(x: i32) -> i32 {
    let number: f64 = (x as f64).sqrt();
    return number as i32;
}

fn delete_duplicates(head: Vec<u32>) -> Vec<u32> {
    let mut without_duplicate: Vec<u32> =  head
        .into_iter()
        .collect::<HashSet<u32>>()
        .into_iter()
        .collect::<Vec<u32>>();
    without_duplicate.sort();

    return without_duplicate;
}

fn main() {}

#[test]
fn unit_tests() {
    // SQRT 
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(9), 3);
    assert_eq!(my_sqrt(12), 3);

    // Remove duplicates
    assert_eq!(delete_duplicates(vec![1,1,2,3]), vec![1,2,3]);
}