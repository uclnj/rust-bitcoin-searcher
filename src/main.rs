use bitcoin::address::Address;
use bitcoin::key::PrivateKey;
use bitcoin::Network;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use hex::decode;
use rand::Rng;
use secp256k1::Secp256k1;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufRead;
use std::io::{stdout, Write};
use std::path::Path;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Rust Bitcoin Collider", version = "1.0", about = "Wastes CPU trying to get a collission")]
struct Args {
    /// Input filename
    filename: String,
    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    threads: usize,
    test: bool,
}

const BATCH_SIZE: usize = 350_000;
// The known test private key hash (hex string, 20 bytes for address hash160)
const PUBKEY_HASH_HEX: &str = "959b1e8c508bf3ffbd9ece6890c392b3ecc6b3b84528b4242990d81b7a077ff2";

fn main() {
    let args = Args::parse();
    let mut stdout = stdout();
    let mut address_total: usize = 0;

    execute!(stdout, Clear(ClearType::All)).unwrap();
    execute!(stdout, MoveTo(0, 0)).unwrap();
    let addresses = load_addresses_from_file(args.filename).expect("Failed to load address file");
    let address_set: HashSet<String> = addresses.into_iter().collect();

    if args.test {
        println!("Test mode...");
        let privkey_bytes = decode(PUBKEY_HASH_HEX).expect("Invalid hex");
        // Build PrivateKey object
        let private_key =
            PrivateKey::from_slice(&privkey_bytes, Network::Bitcoin).expect("Invalid private key");
        // Derive pubkey
        let public_key = private_key.public_key(&bitcoin::secp256k1::SECP256K1);
        // Generate P2PKH
        let address = Address::p2pkh(&public_key, Network::Bitcoin);
        println!("Private Key: {}", PUBKEY_HASH_HEX);
        println!("Public Key:  {}", public_key);
        println!("BTC Address: {}", address);
        if address_set.contains(&address.to_string()) {
            println!("Match found!");
            println!("Address: {}", address);
        }
        process::exit(0);
    }

    let (tx, rx) = mpsc::channel();

    for id in 0..args.threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let secp = Secp256k1::new();
            let mut batch = Vec::with_capacity(BATCH_SIZE);
            let thread_id: usize = id;
            let mut start = Instant::now();
            // make each thread have different batch sizes
            let batch_size = BATCH_SIZE;
            let mut counter: usize = 0;
            loop {
                let mut rng = rand::thread_rng();
                let mut key_bytes = [0u8; 32];
                rng.fill(&mut key_bytes);
                let private_key = PrivateKey::new(
                    bitcoin::secp256k1::SecretKey::from_slice(&key_bytes).unwrap(),
                    Network::Bitcoin,
                );
                let public_key = private_key.public_key(&secp);
                let address = generate_address(&public_key, Network::Bitcoin).to_string();
                batch.push((address, private_key.to_wif(), public_key.to_string()));
                if batch.len() == batch_size {
                    counter += batch_size;
                    let loop_ms = start.elapsed();
                    if tx.send((loop_ms, counter, thread_id, batch)).is_err() {
                        break;
                    }
                    batch = Vec::with_capacity(BATCH_SIZE);
                    start = Instant::now();
                }
            }
        });
    }

    // Main thread: check batches
    for (loop_ms, counter, thread_id, batch) in rx {
        execute!(stdout, MoveTo(0, 0)).unwrap();
        address_total += counter;
        println!("Address in memory {}", address_set.len());
        println!("Using {} threads", args.threads);
        println!("Address generation in progress... {}", address_total);
        execute!(stdout, MoveTo(0, thread_id as u16 + 4)).unwrap();
        let start = Instant::now();
        for (address, wif, pubkey) in batch {
            if address_set.contains(&address) {
                execute!(stdout, MoveTo(0, 15)).unwrap();
                println!("Address: {}", address);
                println!("Private Key (WIF): {}", wif);
                println!("Public Key: {}", pubkey);
                // Append match to matches.csv
                if let Err(e) = append_match_to_csv(&address, &wif, &pubkey) {
                    eprintln!("Failed to write to matches.csv: {}", e);
                }
                return;
            }
        }
        let end = start.elapsed();
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("Thread"),
            Print(" ("),
            Print(thread_id.to_string()),
            Print("): "),
            SetForegroundColor(Color::Green),
            Print(counter.to_string()),
            Print(" loop "),
            Print(loop_ms.as_millis()),
            Print("ms"),
            SetForegroundColor(Color::Cyan),
            Print(" batch search "),
            Print(end.as_millis()),
            Print("ms"),
            ResetColor
        )
        .unwrap();
    }
}

pub fn load_addresses_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let addresses = reader.lines().filter_map(Result::ok).collect();
    Ok(addresses)
}

pub fn generate_address(public_key: &bitcoin::PublicKey, network: Network) -> Address {
    Address::p2pkh(public_key, network)
}

// Append match data to matches.csv
fn append_match_to_csv(address: &str, wif: &str, pubkey: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("matches.csv")?;
    writeln!(file, "{},{},{}", address, wif, pubkey)?;
    Ok(())
}
