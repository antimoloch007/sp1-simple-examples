use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    age: u32,
}

fn main() {
    // Step 1: Open the input.json file
    let mut file = File::open("input.json").expect("Failed to open input.json");

    // Step 2: Read the content of the file into a string
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Failed to read input.json");

    // Step 3: Deserialize JSON into the UserData struct
    let user: UserData = serde_json::from_str(&json_content).expect("Failed to parse JSON");

    // Step 4: Serialize the UserData struct into bincode format
    let encoded: Vec<u8> = bincode::serialize(&user).expect("Failed to serialize to bincode");

    // Step 5: Write the binary-encoded data to input.bin
    let mut output_file = File::create("input.bin").expect("Failed to create input.bin");
    output_file.write_all(&encoded).expect("Failed to write binary data");

    println!("Binary input successfully written to input.bin");
}
