pub fn replace_number(number: u8) -> String {
    let forbidden = vec![3, 5];

    if forbidden.contains(&number) {
        "Fizz".to_string()
    } else {
        format!("{}", number)
    }
}

#[cfg(test)]
mod replace_number_tests {
    use super::*;

    #[test]
    fn should_replace_number_by_fizz() {
        let result = replace_number(2);
        assert_eq!(result, "4");
    }
}