use std::fs;


pub fn  read_file(file: &str) -> Vec<Vec<String>> {
    let sdk = fs::read_to_string(file).expect("Cannot read file.");
    let sdk = sdk.split("\r\n");
    let sdk = sdk.collect::<Vec<&str>>();

    let mut sdk_by_lines: Vec<Vec<String>> = vec![];
    
    for line in sdk {
        let mut line_content: Vec<String> = vec![];
        for char in line.chars() {
            line_content.push(char.to_string());}
        sdk_by_lines.push(line_content);
    }

    match sdk_by_lines.len() {
        9 => for line in &sdk_by_lines {
                match line.len() {
                    9 => continue,
                    _ => panic!("One of the lines of the sudoku does not contain 9 characters.")
                }
            },
        _ => panic!("Your sudoku doesnt contains 9 lines.")
    }
    sdk_by_lines
}