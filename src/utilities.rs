use std::fs;

pub fn read_input_file(file_name: &str) -> String {
    let path = String::from("inputs/") + file_name;
    fs::read_to_string(&path).expect(
        format!("Could not read {}", path).as_str()
    )
}