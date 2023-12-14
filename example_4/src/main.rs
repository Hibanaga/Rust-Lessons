
fn merge_two_lists(first_array: Vec<i8>, second_array: Vec<i8>) -> Vec<i8> {
    let mut concatinated_array = [first_array, second_array].concat();
    concatinated_array.sort();
    return concatinated_array;
}

fn is_closed_brackets (string: &str) -> bool {
    const ASCII_DIVIDER_OPERATOR: u32 = 50;
    let mut ascii_array: Vec<u32> = string
        .chars()
        .map(|c| c as u32)
        .collect::<Vec<u32>>();
    ascii_array.sort();

    let chunked_array: Vec<Vec<u32>> = ascii_array
        .chunks(2)
        .map(|chunk| chunk.to_vec())
        .collect();

    for chunk in chunked_array  {
        if chunk.len() < 2 {
            return false;
        }

        if let [current, next] = &chunk[..] {
            if current + (if current > &ASCII_DIVIDER_OPERATOR { 2 } else { 1 }) != *next {
                return false;
            }
        }
    }
    return true;
}

fn remove_duplicates(mut array: Vec<i32>) -> Vec<i32> {
    array.sort();
    array.dedup_by(|a, b| a.eq(&b));
    return array;
}

fn remove_element(nums: Vec<i32>, num: i32) -> Vec<i32> {
    return nums
        .iter()
        .filter(|&a| *a != num)
        .cloned()
        .collect();
}

fn str_str(hay_stack: String, needle: String) -> i32 {
    return match hay_stack.find(&needle) {
        Some(idx) => idx as i32,
        None => -1,
    };
}

fn search_insert (nums: Vec<i32>, target: i32) -> i32 {
    return match nums.clone().iter().position(|&el| el >= target) {
        Some(idx) => idx as i32,
        None => -1,
    };
}

fn main() {}

#[test]
fn it_works() {
    //Merge two lists
    assert_eq!(merge_two_lists(vec![1,2,4], vec![1,3,4]), [1, 1, 2, 3, 4, 4]);

    // Is closed Brackets
    assert_eq!(is_closed_brackets("[{}]"), true);
    assert_eq!(is_closed_brackets("[{}"), false);

    // Remove duplicates
    assert_eq!(remove_duplicates(vec![1, 0, 0, 0, 1, 2, 3, 4]), vec![0,1,2,3,4]);
    assert_eq!(remove_duplicates(vec![3,2,1,0]), vec![0,1,2,3]);

    //Remove element
    assert_eq!(remove_element(vec![1,2,3,4], 2), vec![1,3,4]);
    assert_eq!(remove_element(vec![1,2,3,4], 3), vec![1,2,4]);

    //Str Str
    assert_eq!(str_str("sadbutsad".to_string(), "rrr".to_string()), -1);
    assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(str_str("abracham".to_string(), "ra".to_string()), 2);

    // Search Insert
    assert_eq!(search_insert(vec![1,3,5,9], 5), 2);
    assert_eq!(search_insert(vec![1,3,5,9], 4), 2);
    assert_eq!(search_insert(vec![1,3,5,9], 2), 1);
}