# Rust Bitcoin Address Search

This project is designed to quickly generate Bitcoin addresses and check them against a list of addresses loaded from a file. If a generated address matches one from the file, the corresponding key used to generate that address will be printed.

## Project Structure

```
bitcoin-address-matcher
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

Make sure you have Rust.

### Running the Project

1. Clone the repository or download the project files.
2. Place your Bitcoin addresses in a file.  I get mine from http://addresses.loyce.club/ and split the file into 250MB managable chunks.
3. Open a terminal and navigate to the project directory.
4. Run the following command to build and execute the project:

   ```
   cargo build --release
   ```
5. Go into your target/release folder and run the program from there.  It will first load the file and then begin wasting your CPU searching for a collision.

### Dependencies

This project may require additional dependencies for handling Bitcoin address generation and validation. Ensure that you have the necessary crates specified in `Cargo.toml`.

### Contributing

Feel free to submit issues or pull requests if you have suggestions for improvements or additional features.