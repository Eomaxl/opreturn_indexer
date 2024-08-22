use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::bitcoin::Txid;
use bitcoincore_rpc::bitcoincore_rpc_json::ScriptPubkeyType;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Transaction {
    pub tx_hash: String,
    pub block_hash: String,
    pub op_return_data: String,
}

pub fn get_op_return_data(client: &Client, txid: &Txid) -> Vec<String> {
    let raw_tx = client.get_raw_transaction_info(txid, None).unwrap();
    let mut op_return_data = Vec::new();

    for vout in raw_tx.vout {
        if vout.script_pub_key.type_ == Some(ScriptPubkeyType::NullData) {
            let hex_data = vout.script_pub_key.asm.split(' ').skip(1);
            op_return_data.extend(hex_data.map(|s| s.to_string()));
        }
    }

    op_return_data
}

pub fn connect_to_bitcoin() -> Client {
    let rpc_url = "http://127.0.0.1:38332";
    let rpc_auth = Auth::UserPass("yourrpcuser".to_string(), "yourrpcpassword".to_string());
    Client::new(&*rpc_url.to_string(), rpc_auth).unwrap()
}
