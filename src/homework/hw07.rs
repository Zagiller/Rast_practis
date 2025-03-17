fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_lowercase() { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() })
        .collect()
}

fn main() {
    let input = "Hello, Rust!";
    let swapped = swap_case(input);
    println!("Original: {}", input);
    println!("Swapped: {}", swapped);
}
