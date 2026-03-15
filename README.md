# Soroban Donation Box

A smart contract for managing donations on the Soroban blockchain.

## Overview

The Donation Box contract allows users to donate funds to a centralized box. The admin (beneficiary) can withdraw the collected donations. The box can be opened or closed to control whether new donations are accepted.

## Features

- **Initialization**: Set an admin who can manage the box and withdraw funds.
- **Donations**: Users can donate positive amounts while the box is open.
- **Withdrawals**: Admin can withdraw all available funds.
- **Box Control**: Admin can open or close the donation box.
- **Queries**: Check total donated, available funds, individual donor amounts, donor list, etc.
- **Admin Transfer**: Admin can transfer the admin role to another address.

## Project Structure

```
.
├── contracts
│   └── hello-world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Setup

1. Ensure you have Rust and Soroban CLI installed.
2. Clone this repository.
3. Build the contract:

```bash
cargo build --release --target wasm32-unknown-unknown
```

## Deployment

To deploy the contract to a Soroban network:

1. Set up your Soroban environment.
2. Deploy using Soroban CLI:

```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/hello_world.wasm --source <your-source>
```

## Usage

### Initialize the Contract

Call `init(admin: Address)` to set the admin and open the box.

### Donate

Call `donate(donor: Address, amount: i128)` to donate funds.

### Withdraw

Admin calls `withdraw()` to withdraw all available funds.

### Control the Box

- `close_box()`: Close the box to stop accepting donations.
- `open_box()`: Re-open the box.

### Queries

- `total_donated()`: Total amount donated.
- `available()`: Funds available for withdrawal.
- `donor_amount(donor: Address)`: Amount donated by a specific donor.
- `donors()`: List of all donors.
- `is_open()`: Whether the box is open.
- `admin()`: Current admin address.

## Testing

Run the tests with:

```bash
cargo test
```

The test suite covers initialization, donations, and withdrawals.

## API Reference

See the contract code in `contracts/hello-world/src/lib.rs` for detailed function signatures and implementations.

## Transaction Link

[View Transaction on Stellar Expert](https://stellar.expert/explorer/testnet/tx/CCAVSF4ULWPBHD6MSOUMUGMK5SCQN3QJ76XST2UXM5LZBZYJ6YKJP7LR)

## Contributing

Contributions are welcome. Please ensure tests pass and follow the existing code style.
