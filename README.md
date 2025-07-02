# VarSwap Smart Contracts

This repository contains the smart contracts for the VarSwap decentralized exchange (DEX) ecosystem, including modules for factory, router, liquidity pool (LP), staking, and wrapped VARA token (wVARA).

---

## Table of Contents

- [Project Structure](#project-structure)
- [Modules Overview](#modules-overview)
- [Getting Started](#getting-started)
- [Building the Contracts](#building-the-contracts)
- [Testing](#testing)
- [Deployment](#deployment)
- [Usage Example](#usage-example)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Project Structure

```
varswap-smart-contract/
│
├── factory_vara_dex/      # Factory contract for creating and managing DEX pairs
│   ├── app/               # Main contract logic (Rust)
│   └── wasm/              # WASM build artifacts and client
│
├── lp_vara_dex/           # Liquidity pool contract for DEX
│   ├── app/               # Main contract logic (Rust)
│   └── wasm/              # WASM build artifacts and client
│
├── lp_staking/            # Staking contract for LP tokens
│   ├── app/               # Main contract logic (Rust)
│   ├── client/            # Client library for interacting with the contract
│   └── lib/               # TypeScript bindings
│
├── router_vara_dex/       # Router contract for multi-hop swaps and routing
│   ├── app/               # Main contract logic (Rust)
│   └── wasm/              # WASM build artifacts and client
│
├── wvara/                 # Wrapped VARA token contract
│   ├── app/               # Main contract logic (Rust)
│   └── wasm/              # WASM build artifacts and client
│
└── README.md              # This file
```

---

## Modules Overview

### 1. `factory_vara_dex`
- **Purpose:** Manages the creation and registry of trading pairs on the DEX.
- **Key Features:**
  - Deploy new liquidity pools for token pairs.
  - Maintain a registry of all pools.
  - Manage pool ownership and permissions.

### 2. `lp_vara_dex`
- **Purpose:** Handles liquidity pools, swaps, and liquidity management.
- **Key Features:**
  - Add/remove liquidity.
  - Swap tokens within pools.
  - Track pool reserves and fees.

### 3. `lp_staking`
- **Purpose:** Allows users to stake LP tokens and earn rewards.
- **Key Features:**
  - Stake/unstake LP tokens.
  - Distribute rewards to stakers.
  - Track user balances and reward history.

### 4. `router_vara_dex`
- **Purpose:** Facilitates complex swaps and routing between pools.
- **Key Features:**
  - Multi-hop swaps across different pools.
  - Find optimal swap paths.
  - Aggregate liquidity.

### 5. `wvara`
- **Purpose:** Provides wrapped VARA token (wVARA) functionality.
- **Key Features:**
  - Mint/burn wVARA tokens.
  - 1:1 conversion between VARA and wVARA.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (with `wasm32-unknown-unknown` target)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Node.js](https://nodejs.org/) (for TypeScript bindings)
- [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/) (for JS/TS libraries)

### Install Rust WASM Target

```bash
rustup target add wasm32-unknown-unknown
```

---

## Building the Contracts

Build each contract to generate WASM binaries:

```bash
cd <module>/app
cargo build --release 
```

For example:

```bash
cd factory_vara_dex/app
cargo build --release 
```

Build TypeScript bindings (if available):

```bash
cd <module>/lib
yarn install
yarn build
```

---

## Testing

Run Rust unit and integration tests:

cargo test --release -- --include-ignored


```bash
cd <module>/app
cargo test
```

For JS/TS tests (if available):

```bash
cd <module>/lib
yarn test
```

---

## Deployment

Deployment depends on your target blockchain (e.g., Vara Network, Substrate-based chains).

**General steps:**
1. Upload the WASM binary to the blockchain.
2. Instantiate the contract with the required constructor arguments.
3. Interact with the contract using the client libraries or Polkadot.js Apps.

> **Note:** Check each module's `README.md` or documentation for specific deployment scripts or instructions.

---

## Usage Example


For Rust-based interaction, use the generated WASM and your preferred Substrate client.

---

## Contributing

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Create a new Pull Request

---

## License

This project is licensed under the MIT License.

---

## Contact

- **Maintainer:** [Your Name or Team]
- **Email:** [your.email@example.com]
- **Telegram:** [@yourtelegram]
- **Issues:** Please use the GitHub Issues tab for bug reports and feature requests.
