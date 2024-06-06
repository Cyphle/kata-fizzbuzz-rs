use crate::replace_number;

pub fn fizzbuzz(numbers: &Vec<u8>) -> String {
    numbers
        .iter()
        .map(|x| replace_number(x))
        .collect()
}

#[cfg(test)]
mod fizzbuzz_tests {
    use super::*;

    #[test]
    fn should_fizzbuzz_short_serie() {
        let numbers = vec![0, 1, 2];

        let result = fizzbuzz(&numbers);

        assert_eq!(result, "012");
    }

    #[test]
    fn should_fizzbuzz_long_serie() {
        let numbers = (0..=15).collect();

        let result = fizzbuzz(&numbers);

        assert_eq!(result, "012Fizz4BuzzFizz78FizzBuzz11Fizz1314FizzBuzz");
    }
}