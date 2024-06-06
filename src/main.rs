use kata_fizzbuzz_rs::{fizzbuzz, replace_number};

fn main() {
    let suite = (0..=15).collect();
    let result = fizzbuzz(&suite);

    println!("Rust FizzBuzz");
    println!("Example: {:?}", suite);
    println!("Result {:?}", result);
}
