fn main() {
    return;
}

fn validate_check_sum(input: String) -> bool {
    let mut input_vec = vec![];

    for x in input.chars() {
        input_vec.push(x.to_digit(10).unwrap());
    }

    let check_sum = input_vec.pop().unwrap();

    (calculate_check_sum(input_vec) + check_sum) % 10 == 0
}

fn double_digit(digit: u32) -> u32 {
    let mut sum = digit * 2;
    if sum >= 10 {
        sum = 1 + sum % 10;
    }
    sum
}

fn calculate_check_sum(id_vec: Vec<u32>) -> u32 {
    let mut total_sum = 0;

    // for (_, elem) in input_vec.iter().rev().step_by(2).enumerate() {
    //     total_sum = total_sum + double_digit(*elem);
    // }

    // let every_other: u32 = input_vec.iter().rev().skip(1).step_by(2).sum();

    // return total_sum + check_sum + every_other;

    for (pos, elem) in id_vec.iter().rev().enumerate() {
        if pos % 2 == 0 {
            total_sum = total_sum + double_digit(*elem)
        } else {
            total_sum = total_sum + elem
        }
    }

    return total_sum;
}

fn testing(test_value: &str, exp: bool) -> () {
    assert_eq!(validate_check_sum(test_value.to_string()), exp)
}

#[test]
fn test_check_sum() {
    let n = "1762483";
    testing(n, true);
    let n = "51562190040903843";
    testing(n, true);
    let n = "123455";
    testing(n, true);
}
