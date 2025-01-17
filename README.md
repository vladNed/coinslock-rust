# Coinslock (Rust)

This is Coinslock project re-written in Rust language.The main feature is creating lockers that help users secure funds
before interacting with an unknown party. This makes available a secure space where bitcoin users can make escrows,
atomic swaps between themselves by leveraging the power of the Coinslock lockers.

## Installation

To install the project, you need to have Rust installed on your machine. After that just run:

```bash
cargo build
```

This will build the project and create the binary in the target folder.

## Usage

Starting the server on the local machine in the most simplest way is to run the start script:

```bash
./scripts/start.sh
```

but, if you want to change the log type or you don't want to run with watch mode, you can run the binary directly:

```bash
RUST_LOG=<the_type_you_want> cargo run coinslock
```
