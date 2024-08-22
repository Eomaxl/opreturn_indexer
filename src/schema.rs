use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    blocks (block_hash) {
        block_hash -> Text,
        height -> Int4,
        timestamp -> Timestamp,
    }
}

table! {
    transactions (tx_hash) {
        tx_hash -> Text,
        block_hash -> Text,
        op_return_data -> Text,
    }
}

joinable!(transactions -> blocks (block_hash));

allow_tables_to_appear_in_same_query!(
    blocks,
    transactions,
);
