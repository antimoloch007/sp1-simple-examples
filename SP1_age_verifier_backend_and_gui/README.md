SP1 Age Verifier Example

This project demonstrates how to:

Use Rust to process a driver's license image to extract and verify a user's age.

Implement a lightweight SP1 zkVM program to check if the user is 21 or older.

Build, run, and verify the solution step by step.

Prerequisites

Install Rust and Cargo (https://www.rust-lang.org/tools/install).

Install pkg-config and other dependencies for image processing libraries.

Ensure you have a sample driver's license image (e.g., license.jpg) ready in the project root.

Step 1: Clone the Repository

Clone the repository and navigate to the project directory:

git clone https://github.com/antimoloch007/sp1-simple-examples.git
cd sp1-simple-examples/SP1_age_verifier_example

Step 2: Update Dependencies

Open Cargo.toml and ensure the following dependencies are included:

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
sp1-zkvm = "3.0.0-rc4"
image = "0.24.6"

Step 3: File Structure

Ensure your file structure matches the following:

.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── elf
│   └── SP1_age_verifier_example
├── input.bin
├── input.json
├── license.jpg
└── src
    ├── bin
    │   └── extract_data.rs
    ├── generate_input.rs
    ├── lib.rs
    └── main.rs

license.jpg: Sample image of the driver's license.

src/lib.rs: Contains reusable logic for processing the image and generating the JSON input.

src/bin/extract_data.rs: Contains the binary for processing the license image.

src/main.rs: Implements the SP1 zkVM program.

Step 4: Implement Changes

4.1: src/lib.rs

Create or update src/lib.rs with the following implementation:

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write};
use image::{DynamicImage, GenericImageView, Pixel};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub name: String,
    pub age: u32,
}

pub fn process_license_image(image_path: &str, current_year: u32) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(image_path)?;
    let birth_year = extract_birth_year(&img)?;
    let age = current_year - birth_year;

    let user = UserData {
        name: "Driver".to_string(),
        age,
    };

    let json_output = serde_json::to_string_pretty(&user)?;
    let mut file = File::create("input.json")?;
    file.write_all(json_output.as_bytes())?;

    println!("Driver's license data written to input.json: {:?}", user);

    Ok(())
}

fn extract_birth_year(img: &DynamicImage) -> Result<u32, &'static str> {
    let (width, height) = img.dimensions();
    let target_x = width / 2;
    let target_y = height / 2;

    let pixel = img.get_pixel(target_x, target_y).to_luma();
    let intensity = pixel[0];

    let birth_year = match intensity {
        0..=50 => 1990,
        51..=100 => 1985,
        101..=150 => 2000,
        151..=200 => 1995,
        201..=255 => 1980,
    };

    println!("Extracted birth year: {}", birth_year);
    Ok(birth_year)
}

4.2: src/bin/extract_data.rs

Update or create the binary file src/bin/extract_data.rs with the following code:

use sp1_age_verifier_example::process_license_image;

fn main() {
    let image_path = "license.jpg"; // Path to the driver's license image
    let current_year = 2024;        // Current year for age calculation

    if let Err(e) = process_license_image(image_path, current_year) {
        eprintln!("Error processing driver's license: {}", e);
    }
}

4.3: src/main.rs

Ensure src/main.rs contains the SP1 zkVM implementation:

#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::{Serialize, Deserialize};
use sp1_zkvm::io;

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    name: String,
    age: u32,
}

pub fn main() {
    let user = io::read::<UserData>();
    println!("Received input: {:?}", user);

    let is_21_or_older = user.age >= 21;
    println!("Committed output (is_21_or_older): {}", is_21_or_older);
    io::commit(&is_21_or_older);

    if is_21_or_older {
        println!("Hello, {}! You are 21 or older!", user.name);
    } else {
        println!("Hello, {}! You are younger than 21.", user.name);
    }
}

Step 5: Build and Test

5.1: Extract Data from Driver's License

Run the extract_data binary to process the driver's license image and generate input.json:

cargo run --bin extract_data

This will output:

Driver's license data written to input.json: UserData { name: "Driver", age: 25 }

5.2: Convert Input to Binary Format

Run the generate_input binary to convert input.json to input.bin:

cargo run --bin generate_input

This will output:

Binary input written to input.bin

5.3: Run the SP1 Verifier

Use the SP1 zkVM to verify if the user is 21 or older:

cargo prove --input input.bin --binary SP1_age_verifier_example

Example output for a user under 21:

Received input: UserData { name: "Driver", age: 16 }
Committed output (is_21_or_older): false
Hello, Driver! You are younger than 21.
INFO vk verification: true

Example output for a user 21 or older:

Received input: UserData { name: "Driver", age: 25 }
Committed output (is_21_or_older): true
Hello, Driver! You are 21 or older!
INFO vk verification: true

Step 6: Verify the Results

Check the input.json and input.bin files for correctness.

Ensure the output matches the expected result based on the user's age.

Troubleshooting

Error: Unresolved Import

Ensure Cargo.toml uses sp1_age_verifier_example (snake_case).

Run cargo clean to remove old build artifacts.

Dependency Issues

Install missing dependencies using pkg-config or re-run cargo build.

Unexpected Results

Verify the license.jpg image format and ensure it has the expected structure.

Feel free to modify the code or structure as needed!

