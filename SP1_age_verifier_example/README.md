# SP1 Age Verifier Example

This guide shows how to:

- Write an SP1 zkVM program that checks if a user is over 21.
- Create binary inputs for the program.
- Run the SP1 program to generate a proof.
- Verify the proof and view the result.

At each stage, we'll explain what SP1 (Succinct's zkVM) is doing under the hood.

## What is SP1?

SP1 is a zero-knowledge Virtual Machine (zkVM) that compiles Rust programs into RISC-V instructions and generates proofs that attest to the correctness of program execution. These proofs are:

- **Succinct**: Small and fast to verify.
- **Zero-knowledge**: Only the necessary outputs are revealed, protecting sensitive inputs.

## Step 1: Set Up the Project

Create a new Rust project:

```bash
cargo new SP1_age_verifier_example
cd SP1_age_verifier_example
```

**What's happening:** This initializes a standard Rust project structure.

Add the SP1 dependencies in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
sp1-zkvm = "3.0.0-rc4"   # Use the latest release candidate

[[bin]]
name = "generate_input"
path = "src/generate_input.rs"
```

**What's happening:** We're adding libraries for input handling, serialization, and SP1 zkVM execution.

## Step 2: Write the SP1 Program

Replace `src/main.rs` with the following code:

```rust
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
    // Step 1: Read input data
    let user = io::read::<UserData>();
    println!("Received input: {:?}", user);

    // Step 2: Check if age > 21
    let is_over_21 = user.age > 21;
    println!("Committed output (is_over_21): {}", is_over_21);
    io::commit(&is_over_21);

    // Step 3: Output result
    if is_over_21 {
        println!("Hello, {}! You are over 21!", user.name);
    } else {
        println!("Hello, {}! You are not over 21.", user.name);
    }
}
```

### What SP1 is Doing Here:

- **Compiling the Program**: SP1 will take this Rust code and compile it to RISC-V assembly instructions.
- **Program Logic**: The program checks if the input age is greater than 21 and commits the result (true or false) as a public output.
- **Commitment**: `io::commit` ensures the output is included in the proof for verification.

## Step 3: Generate Binary Input

Create a helper program to convert user data from JSON into binary format (understood by SP1).

Create `src/generate_input.rs`:

```rust
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    age: u32,
}

fn main() {
    // Step 1: Read input.json
    let mut file = File::open("input.json").expect("Failed to open input.json");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Failed to read input.json");

    // Step 2: Parse JSON and serialize to binary
    let user: UserData = serde_json::from_str(&json_content).expect("Failed to parse JSON");
    let encoded: Vec<u8> = bincode::serialize(&user).expect("Failed to serialize");

    // Step 3: Write binary input to input.bin
    let mut output_file = File::create("input.bin").expect("Failed to create input.bin");
    output_file.write_all(&encoded).expect("Failed to write binary data");

    println!("Binary input written to input.bin");
}
```

Add `input.json` in the project root with this content:

```json
{
    "name": "Harry Potter",
    "age": 16
}
```

Run the helper program to generate the binary input:

```bash
cargo run --bin generate_input
```

**What's happening:** This helper program:

- Reads `input.json`.
- Serializes the data into binary format (bincode).
- Writes the result to `input.bin`.

## Step 4: Build the SP1 Program

To prepare for execution, build the SP1 program:

```bash
cargo prove build --binary SP1_age_verifier_example
```

### What SP1 is Doing:

- **Compiling to RISC-V**: SP1 compiles the Rust program into a RISC-V binary (optimized for zkVM execution).
- **Preparing for Proof**: The compiled RISC-V binary is ready to run and generate a proof.

## Step 5: Run the SP1 Program and Generate the Proof

Run the SP1 program with the binary input:

```bash
cargo prove --input input.bin --binary SP1_age_verifier_example
```

### What SP1 is Doing:

- **Executing the Program**: SP1 runs the RISC-V binary with the provided input (`input.bin`).
- **Creating the Proof**: SP1 generates a zero-knowledge proof attesting that:
  - The program executed correctly.
  - The committed result (`is_over_21`) is correct.
- **Verifying the Proof**: SP1 automatically verifies the proof and confirms its validity.

## Step 6: View the Output

The terminal output will look like this:

```plaintext
Received input: UserData { name: "Harry Potter", age: 16 }
Committed output (is_over_21): false
Hello, Harry Potter! You are not over 21.
INFO vk verification: true
```

- **Committed output**: `false` means the user did not pass the condition `age > 21`.
- **Verification**: `INFO vk verification: true` confirms that the proof was verified successfully.

## Step 7: Test with Different Inputs

Modify `input.json`:

```json
{
    "name": "Albus Dumbledore",
    "age": 115
}
```

Regenerate the binary input:

```bash
cargo run --bin generate_input
```

Run the SP1 program again:

```bash
cargo prove --input input.bin --binary SP1_age_verifier_example
```

The output will now show:

```plaintext
Received input: UserData { name: "Albus Dumbledore", age: 115 }
Committed output (is_over_21): true
Hello, Albus Dumbledore! You are over 21!
INFO vk verification: true
```

## What Happens Under the Hood in SP1?

At a high level:

- **Compiling Rust to RISC-V**: SP1 compiles the program into RISC-V instructions for execution in the zkVM.
- **Proving Execution**: SP1 generates a proof that:
  - The program logic (`age > 21`) was executed correctly.
  - The committed output (`true` or `false`) matches the program's execution.
- **Verification**: SP1 verifies the proof to ensure correctness.
- **Output**: The result (e.g., `true` or `false`) is printed, showing whether the user meets the condition.

## Summary of Steps

1. **Set up the project**: Create a Rust project and add SP1 dependencies.
2. **Write the SP1 program**: Check `age > 21` and commit the result.
3. **Generate binary input**: Convert JSON to `input.bin` using a helper script.
4. **Build and run the program**: Use SP1 to generate and verify the proof.
5. **View the result**: Verify the proof and output the result.
