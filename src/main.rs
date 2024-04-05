fn main() {
    let suite = vec![1, 2, 3];

    let converted = suite
        .iter()
        .map(|x|
            if *x == 1 {
                "Fizz"
            } else {
                "None"
            }
        )
        .collect::<Vec<&str>>()
        .join("\n");

    println!("Hello, world! {}", converted);
}
