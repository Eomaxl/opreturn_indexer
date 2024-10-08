
---

### `design_document.md`

markdown
# Bitcoin OP_RETURN Indexer Design Document

## Overview

The Bitcoin OP_RETURN Indexer is a Rust-based application designed to index and serve OP_RETURN data from Bitcoin transactions on the Signet blockchain. The data is stored in a PostgreSQL database and is accessible via an HTTP API.

## System Architecture

### Components

1. **Bitcoin Core Integration**:
   - The application connects to a local Bitcoin Core node running on the Signet network using RPC.
   - It fetches transaction data and extracts OP_RETURN values from each transaction output.

2. **Database Layer**:
   - PostgreSQL is used to store indexed OP_RETURN data.
   - The schema is designed to efficiently store transaction hashes, block hashes, and associated OP_RETURN data.

3. **Web Server**:
   - The web server is built using the Rocket framework.
   - It exposes a single endpoint, `/opreturn/{opReturnData}`, to query transactions by OP_RETURN data.

### Data Flow

1. **Blockchain Sync**:
   - The application begins by connecting to Bitcoin Core.
   - It fetches the latest block and iterates through all transactions, extracting OP_RETURN data.
   - For each transaction with OP_RETURN data, the transaction ID, block hash, and OP_RETURN data are stored in PostgreSQL.

2. **Querying Data**:
   - Users query the application by making HTTP GET requests to the `/opreturn/{opReturnData}` endpoint.
   - The server queries PostgreSQL for matching OP_RETURN data and returns the relevant transaction and block hashes as a JSON response.

### Database Schema

The PostgreSQL schema consists of a single table:

```sql
CREATE TABLE op_return_data (
    id SERIAL PRIMARY KEY,
    txid TEXT NOT NULL,
    blockhash TEXT NOT NULL,
    op_return_data TEXT NOT NULL
);
``` 
### Project Structure
- src/bitcoin.rs: Handles Bitcoin Core connection and OP_RETURN data extraction.
- src/database.rs: Manages PostgreSQL connection and data storage.
- src/main.rs: Sets up the Rocket server and integrates the blockchain sync.
- src/sync.rs: Syncs the blockchain and indexes OP_RETURN data.

### Design Considerations
The application is designed to be modular, with separate components for handling Bitcoin RPC, database operations, and the web server.<br>
Syncing the blockchain is performed in a loop, ensuring that the application stays up to date with the latest blocks.

### Synchronization Strategy
The application initially syncs from the current block back to the genesis block, processing each transaction for OP_RETURN data.
After the initial sync, the application runs continuously, processing new blocks as they are mined.
The application ensures data integrity by verifying each block and transaction before indexing.
### API Specification
- Endpoint: /opreturn/{opReturnData} <br>
Method: GET<br>
Response: JSON array of transaction hashes and block hashes.<br>

- Example Response:
```
[
    {
        "txid": "ef192a75bf90e430826f32c7aaa0a9d2aa99f8ddc02e5fef33a98bf7fe4cc1f9",
        "blockhash": "0000000000123456789abcdef...",
        "op_return_data": "65786f647573"
    }
]
```

### Error Handling
If the Bitcoin Core RPC is unavailable, the application logs the error and retries the connection.
If a query to PostgreSQL fails, the server returns a 500 Internal Server Error.

### Assumptions
The application assumes a local Bitcoin Core node with Signet enabled.
It assumes the PostgreSQL database is properly set up and accessible.
The application does not handle large-scale deployments and is intended for single-node setups.

## Installation and Setup

### 1. Clone the Repository

First, clone the repository to your local machine:

```bash
git clone https://github.com/Eomaxl/opreturn_indexer
cd opreturn_indexer
``` 

### 2. Set Up PostgreSQL
Create a new PostgreSQL database for storing OP_RETURN data:
```sql
   CREATE DATABASE op_return_db;
\c op_return_db;
CREATE TABLE op_return_data (
       id SERIAL PRIMARY KEY,
       txid TEXT NOT NULL,
       blockhash TEXT NOT NULL,
       op_return_data TEXT NOT NULL
);
```
### 3. Configure Bitcoin Core
Ensure Bitcoin Core is running with Signet enabled. You can start Bitcoin Core with the following command:
```bash
bitcoind -signet -daemon
```
Make sure to configure RPC access in your bitcoin.conf file:
```ini
rpcuser=your_user
rpcpassword=your_password
rpcport=38332
```
### 4. Build and Run Rust Application
Ensure you have Rust installed on your machine.Once Rust is installed, build the application:
```bash
cargo build --release
```
After building the application, you can run it with the following command:
```bash
cargo run --release
```
### Performance Considerations
The application is optimized for fast querying of OP_RETURN data by using indexed queries in PostgreSQL.
Data storage is minimized by only storing necessary transaction and block hashes along with the OP_RETURN data.

### Future Enhancements
Implement caching mechanisms to reduce database load for frequently queried OP_RETURN data.
Extend support to other Bitcoin networks, such as Testnet and Mainnet.
Introduce more robust error handling and monitoring.