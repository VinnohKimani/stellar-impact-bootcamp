# stellar-impact-bootcamp


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


## Week 3: School Management System

### Deployed Contract Details
* **Network:** Stellar Testnet
* **Deployer Identity:** schoolAdmin
* **Contract ID:** CDLLRXJWMNLXXY6BQJ2YVQPMTJ4HT6QH32RIOW3ERNRZDMLNH4S4OBI
* **Stellar Expert Explorer Link:** https://stellar.expert/explorer/testnet/tx/acccb06a77a4b55dd38eb08cdcd931149c260ece369d38adc4255242bca6d9f2

### Features Implemented
* **Student Registration:** Robust management of student enrollment records linked directly to cryptographic wallet addresses.
* **On-Chain Payments Engine:** Integrated payment settlement workflows that interface dynamically with custom underlying asset tokens to capture real-time school fee collections.
* **Class Profiling & History Audits:** Added specialized administration functions allowing verified updates to individual student metrics and safe profile record removal.
* **Historical State Retrieval:** Implemented deep log array scanning to query structural payment vectors directly out of persistent contract storage.