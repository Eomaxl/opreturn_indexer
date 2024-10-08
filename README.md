# Bitcoin OpReturn_Indexer

This application indexes and serves Bitcoin OP_RETURN data from the Bitcoin Signet blockchain. It stores the OP_RETURN data in a PostgreSQL database and provides an HTTP endpoint to retrieve the transaction and block hashes associated with a specific OP_RETURN value.

## Features

- **Syncing with Blockchain:** Continuously syncs with the Bitcoin Signet blockchain and indexes OP_RETURN data from each transaction.
- **HTTP API:** Exposes a RESTful API to query transaction and block hashes by OP_RETURN data.
- **PostgreSQL Storage:** Stores indexed data in a PostgreSQL database for efficient querying.

## Requirements

- Rust (stable version)
- PostgreSQL
- Bitcoin Core with Signet enabled

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/Eomaxl/opreturn_indexer.git
    cd opreturn_indexer
    ```

2. **Set up PostgreSQL:**

   Create a new PostgreSQL database:

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

3. **Configure Bitcoin Core:**

   Ensure Bitcoin Core is running with Signet enabled:

    ```bash
    bitcoind -signet -daemon
    ```

   Configure RPC access in `bitcoin.conf`:

    ```ini
    rpcuser=your_user
    rpcpassword=your_password
    rpcport=38332
    ```

4. **Build and Run the Application:**

   Ensure you have Rust installed, then build and run the application:

    ```bash
    cargo build --release
    cargo run --release
    ```

   This will start the blockchain sync process and the HTTP server.

## Usage

The API is served on `http://localhost:8000`. You can query OP_RETURN data as follows:

```bash
curl http://localhost:8000/opreturn/65786f647573
