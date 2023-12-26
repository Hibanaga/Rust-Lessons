
fn narcissistic(num: u64)-> bool {
    let length_num: u32 = num.to_string().len().try_into().unwrap();

    let mut sum: u64 = 0;
    for el in num.to_string().chars() {
        let number_pow = (el.to_string()).parse::<u64>().unwrap().pow(length_num);
        sum += number_pow;
    }

    return sum == num;
}

fn dig_pow(n: i64, p: i32) -> i64 {
    let mut sum: i64 = 0;
    for (i, el) in n.to_string().chars().enumerate() {
        let number_pow = (el.to_string()).parse::<i64>().unwrap().pow((p + i as i32).try_into().unwrap());
        sum += number_pow;
    }

    if sum == n * (sum / n) {
        return sum / n;
    }

    return -1;
}

fn validate_pin(pin: &str) -> bool {
    let chars = pin.chars().into_iter().collect::<Vec<_>>().len();
    let only_numeric: Vec<_> = pin.chars().into_iter().filter(|&x| x.is_numeric()).collect();
    return (chars == 4 || chars == 6) && (only_numeric.len() == 4 || only_numeric.len() == 6) && chars == only_numeric.len();
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let sum = p0 + p0 * (percent as i32 / 100) + 100;
    println!("{:?}", sum);
    return 2;
}

fn main() {
    println!("{:?}", nb_year(1500, 5.0, 100, 5000));
}

#[test]
fn unit_tests() {
    assert_eq!(narcissistic(153), true);
    assert_eq!(narcissistic(1652), false);
    assert_eq!(narcissistic(371), true);

    assert_eq!(dig_pow(46288, 3), 51);
    assert_eq!(dig_pow(46288, 5), -1);

    assert_eq!(validate_pin("1234"), true);
    assert_eq!(validate_pin("0000"), true);
    assert_eq!(validate_pin("1111"), true);
}
