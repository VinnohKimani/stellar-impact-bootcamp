# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

# Stellar Impact Bootcamp 

## Week 2: SEP-41 Token Implementation

### Deployed Contract Details
* **Network:** Stellar Testnet
* **Deployer Identity:** vinnohKimani
* **Contract ID:** CCFTBU53IQV3X6SWNLEP56QLAC2QC6MN6DIBNBDZSDAOS5R2EZRZLCGP
* **Stellar Expert Explorer Link:**  https://stellar.expert/explorer/testnet/tx/f147036a21e845f3dadf81164be9ec200006bc2c90fc0a34800e948c5563449f

### Features Implemented
* Complete SEP-41 compliant token methods.
* Custom logic for core operations: `mint`, `burn`, `burn_from`, and `transfer_from`.
* Comprehensive local unit tests covering token lifecycles with a passing test suite.