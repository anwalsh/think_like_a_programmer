fn main() {
    let n = "1762483";
    println!("{}", compute_check_sum(n.to_string()));
}

fn compute_check_sum(input: String) -> u32 {
    let mut input_vec = vec![];
    let mut total_sum = 0;

    for x in input.chars() {
        input_vec.push(x.to_digit(10).unwrap());
    }

    let check_sum = input_vec.pop().unwrap();

    // for (_, elem) in input_vec.iter().rev().step_by(2).enumerate() {
    //     total_sum = total_sum + double_digit(*elem);
    // }

    // let every_other: u32 = input_vec.iter().rev().skip(1).step_by(2).sum();

    // return total_sum + check_sum + every_other;

    for (pos, elem) in input_vec.iter().rev().enumerate() {
        if pos % 2 == 0 {
            total_sum = total_sum + double_digit(*elem)
        } else {
            total_sum = total_sum + elem
        }
    }
    total_sum + check_sum
}

fn double_digit(digit: u32) -> u32 {
    let mut sum = digit * 2;
    if sum >= 10 {
        sum = 1 + sum % 10;
    }
    sum
}
