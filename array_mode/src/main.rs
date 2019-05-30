fn main() {
    return;
}

fn get_mode(input: &mut [i32]) -> i32 {
    input.sort();
    let mut mode = 0;
    let mut prev_freq = 0;

    for i in 0..input.len() {
        let mut freq = 0;
        for j in 0..input.len() {
            if input[i] == input[j] {
                freq += 1;
            }
        }
        if freq > prev_freq {
            mode = input[i];
            prev_freq = freq;
        }
    }
    return mode;
}

fn get_mean(input: &[i32]) -> f32 {
    return input.iter().sum::<i32>() as f32 / (input.len() as f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_mode_test() {
        let mut arr: [i32; 10] = [1, 1, 2, 4, 7, 22, 22, 3, 3, 3];
        assert_eq!(3, get_mode(&mut arr));
        let mut arr: [i32; 16] = [
            1, 1, 1, 1, 1, 2, 22, 25, 23, 90, 100, 120, 101, 101, 101, 101,
        ];
        assert_eq!(1, get_mode(&mut arr))
    }

    #[test]
    fn find_mean_test() {
        let mut arr: [i32; 10] = [1, 1, 2, 4, 7, 22, 22, 3, 3, 3];
        assert_eq!(6.8, get_mean(&mut arr));
    }
}
