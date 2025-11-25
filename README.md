# Rust Bitcoin Address Search

I wrote this to learn more about Rust.  Compared to my Python and JavaScript versions, this just knocks them out of the park.

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

Have Rust installed, grab the huge file from Loyce and split it into smaller chunks unless you have a ton of RAM and are feeling adventerous.

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

Start by executing your compiled app like ./rust-bitcoin-searcher btc_chunk_a --threads 4

The default is four threads.

On my Ryzen7 8C machine with 64GB RAM at four threads I pull about 25% CPU and average around 1.2-14GB of RAM on a 400MB file address chunk.

The output should look similar to this:
```
  Address in memory 11988186
  Using 4 threads
  Address generation in progress... 19250000

  Thread (0): 1750000 loop 18462ms batch search 155ms
  Thread (1): 1750000 loop 18666ms batch search 169ms
  Thread (2): 1400000 loop 18421ms batch search 174ms
  Thread (3): 1400000 loop 22710ms batch search 141ms
```

It will create/check about 18406050000 addresses per hour.

### Dependencies

This project may require additional dependencies for handling Bitcoin address generation and validation. Ensure that you have the necessary crates specified in `Cargo.toml`.

### Contributing

Feel free to submit issues or pull requests if you have suggestions for improvements or additional features.
