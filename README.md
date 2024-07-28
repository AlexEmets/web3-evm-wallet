# Ethereum Wallet Project

This project implements a simple Ethereum wallet using Rust. The wallet allows you to create a new wallet, load a wallet from a file, save a wallet to a file, check the balance, and send transactions using the Ethereum Sepolia testnet.

## Features

1. Create a new wallet
2. Load wallet from a file
3. Save wallet to a file
4. Check wallet balance
5. Send transactions

## Requirements

- Rust and Cargo installed
- Internet connection
- Sepolia testnet URL (provided in the code)

## Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
secp256k1 = {version = "0.27.0", features = ["rand"]}
web3 = "0.19.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rust-crypto = "0.2.36"
rand = "0.8.4"
tiny-keccak = "2.0.2"
hex = "0.4.3"
```


## Project structure
```
.
├── src
│   ├── main.rs
│   ├── wallet.rs
│   ├── utils.rs
├── Cargo.toml
└── README.md
```
