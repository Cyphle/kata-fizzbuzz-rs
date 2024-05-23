pub fn replace_number(number: u32) -> String {
    let forbidden = vec![3, 5];

    if forbidden.contains(&number) {
        "Fizz".to_string()
    } else {
        format!("{}", number)
    }
}
