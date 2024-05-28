pub fn replace_number(number: u8) -> String {
    let forbidden: Vec<(u8, String)> = vec![
        (3, "Fizz".to_string()),
        (5, "Buzz".to_string())
    ];

    if is_dividable_by(number, 3) {
        "Fizz".to_string()
    } else if is_dividable_by(number, 5) {
        "Buzz".to_string()
    } else {
        format!("{}", number)
    }
}

pub fn is_dividable_by(to_test: u8, divisor: u8) -> bool {
    to_test % divisor == 0
}

#[cfg(test)]
mod replace_number_tests {
    use super::*;

    mod fizz {
        use crate::replace_number;

        #[test]
        fn should_replace_number_by_fizz_when_three() {
            let result = replace_number(3);
            assert_eq!(result, "Fizz");
        }

        #[test]
        fn should_not_replace_by_fizz_when_not_three() {
            let result = replace_number(2);
            assert_eq!(result, "2");
        }

        #[test]
        fn should_replace_number_by_fizz_when_dividable_by_three() {
            let result = replace_number(6);
            assert_eq!(result, "Fizz");
        }
    }

    mod buzz {
        use crate::replace_number;

        #[test]
        fn should_replace_number_by_buzz_when_five() {
            let result = replace_number(5);
            assert_eq!(result, "Buzz");
        }

        #[test]
        fn should_not_replace_by_buzz_when_not_five() {
            let result = replace_number(2);
            assert_eq!(result, "2");
        }

        #[test]
        fn should_replace_number_by_buzz_when_dividable_by_five() {
            let result = replace_number(10);
            assert_eq!(result, "Buzz");
        }
    }

    mod dividable_by_three {
        use crate::fizzbuzz::forbidden_word::is_dividable_by;

        #[test]
        fn should_return_true_when_three() {
            let res = is_dividable_by(3, 3);
            assert_eq!(res, true)
        }

        #[test]
        fn should_return_true_when_six() {
            let res = is_dividable_by(6, 3);
            assert_eq!(res, true)
        }

        #[test]
        fn should_return_false_when_not_dividable_by_three() {
            let res = is_dividable_by(5, 3);
            assert_eq!(res, false);
        }
    }
}