pub fn match_1(input: Vec<String>) -> bool {
    match input.as_slice() {
        [_, second_last, last] => second_last.starts_with("PBA") && last.starts_with("PBA"),
        _ => false, // Less than two elements in the vector
    }
}

fn main() {
    // Create a sample vector of strings
    let input = vec!["Sample string 1".to_string(), "PBA sample string".to_string(), "PBA sample string".to_string()];
    
    // Call the match_1 function with the input vector
    let result = match_1(input);
    
    // Print the result
    println!("{}", result);
}
