use near_sdk::{json_types::Base64VecU8, serde_json};
use crate::*;

pub fn gen_cluster_id() -> ClusterId {
    return generate_id(String::from("cluster"));
}

pub fn gen_project_id() -> ProjectId {
    return generate_id(String::from("project"));
}

fn generate_id(text: String) -> String {
    let account_id = env::signer_account_id();
    let mut raw_id = account_id.to_owned().to_string();
    raw_id.push_str("_");
    raw_id.push_str(&text);
    raw_id.push_str("_");
    raw_id.push_str(&(&env::block_timestamp().to_string()));
    let u8_id = raw_id.as_bytes();
    let vec_id: Vec<u8> = u8_id.iter().cloned().collect();
    let encode = <Base64VecU8 as From<Vec<u8>>>::from(vec_id);
    let enc_vec = <Base64VecU8 as From<Base64VecU8>>::from(encode);
    let enc_str: String = serde_json::to_string(&enc_vec).unwrap().replace('"', "");
    return enc_str;
}
