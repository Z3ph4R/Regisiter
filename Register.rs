fn change_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let input = "Hello, World!";
    let output = change_case(input);
    println!("Input: {}", input);
    println!("Output: {}", output);
}
