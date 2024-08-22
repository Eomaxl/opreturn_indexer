use diesel::{QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;
use serde_json::json;
use rocket::State;
use crate::db::DbPool;
use diesel::prelude::*;
use diesel::ExpressionMethods;



#[get("/opreturn/<op_return_data>")]
pub fn get_opreturn_data(conn: &State<DbPool>, op_return_data: String) -> Json<serde_json::Value> {
    use crate::schema::transactions::dsl::*;

    // Get a connection from the pool as mutable
    let mut conn = conn.get().expect("Failed to get a database connection from the pool");

    let results = transactions
        .filter(op_return_data.eq(op_return_data))
        .select((tx_hash, block_hash, op_return_data))
        .load::<(String, String, String)>(&mut conn)
        .expect("Error loading transactions");

    Json(json!({
        "transactions": results.iter().map(|(tx_hash_val, _, _)| tx_hash_val).collect::<Vec<_>>(),
        "blocks": results.iter().map(|(_, block_hash_val, _)| block_hash_val).collect::<Vec<_>>(),
    }))
}
