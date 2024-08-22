CREATE TABLE blocks (
                        block_hash TEXT PRIMARY KEY,
                        height INT,
                        timestamp TIMESTAMP
);

CREATE TABLE transactions (
                              tx_hash TEXT PRIMARY KEY,
                              block_hash TEXT REFERENCES blocks(block_hash),
                              op_return_data TEXT
);