
// Input

pub fn split_input_into_lines(input: &str) -> Vec<String> {
    input.split('\n').map(String::from).collect()
}
