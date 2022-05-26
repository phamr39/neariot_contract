use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};
use near_sdk::json_types::Base64VecU8;
use serde::Serialize;
near_sdk::setup_alloc!();

// ------------------------------
// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
pub struct Cluster {
    owner: String,
    name: String,
    descriptions: String,
    pub id: String,
    api_key: String,
    data: String,
}

impl Cluster {
    fn generate_id() -> String {
        let account_id = env::signer_account_id();
        let mut raw_id = account_id.to_owned();
        raw_id.push_str("_");
        raw_id.push_str(&(&env::block_timestamp().to_string()));
        let u8_id = raw_id.as_bytes();
        let vec_id: Vec<u8> = u8_id.iter().cloned().collect();
        let encode: Base64VecU8 = <Base64VecU8 as From<Vec<u8>>>::from(vec_id);
        let enc_vec = <Base64VecU8 as From<Base64VecU8>>::from(encode);
        let enc_str: String = serde_json::to_string(&enc_vec).unwrap().replace('"', "");
        return enc_str;
    }

    fn generate_api_key() -> String {
        // let id = env::signer_account_id();
        return String::from("2g8g3gd8g27g332dh3g78732dg328g327ug3d2");
    }

    pub fn new(name: String, description: String) -> Self {
        Self {
            owner: env::signer_account_id(),
            id: Cluster::generate_id(),
            name: name,
            descriptions: description,
            api_key: Cluster::generate_api_key(),
            data: String::from(""),
        }
    }

    pub fn update_data(mut self, data: String) -> Self {
        self.data = data;
        return self;
    }
}

// ------------------------------

// #[derive(BorshDeserialize, BorshSerialize, Default, Serialize)]
// pub struct User {
//     id: String,
//     clusters: Vec<String>,
//     project: Vec<String>,
// }
// ------------------------------

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: LookupMap<String, String>,
    clusters_storage: UnorderedMap<String, Cluster>,
    // users_storage: UnorderedMap<String, User>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
            clusters_storage: UnorderedMap::new(b"cS".to_vec()),
            // cluste: UnorderedMap::new(b"cS".to_vec()),
            // users_storage: UnorderedMap::new(b"uS".to_vec()),
        }
    }
}

pub enum Response<T> {
    None,
    Some(T),
}

pub enum ClusterResponse {
    Cluster(Cluster),
    Error(String),
}

#[near_bindgen]
#[allow(unused_variables)]
impl Contract {
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }

    pub fn new_cluster(&mut self, name: String, descriptions: String) -> String {
        let cluster: Cluster = Cluster::new(name, descriptions);
        let is_existed = match self.clusters_storage.get(&cluster.id) {
            Some(cluster) => true,
            None => false,
        };

        if is_existed == false {
            self.clusters_storage.insert(&cluster.id, &cluster);
            return cluster.id;
        } else {
            return String::from("Existed");
        }
    }

    pub fn get_cluster(&mut self, id: String) -> Cluster {
        let cluster = self.clusters_storage.get(&id).unwrap();
        let owner = &cluster.owner;
        let sender = &env::signer_account_id();

        if owner == sender {
            return cluster;
        } else {
            panic!("Can not get Cluster"); 
        }
    }

    pub fn get_num_cluster(&mut self, user_id: String) -> i32 {

    }

    // pub fun get_clusters(&mut self, user_id: String) -> 

    pub fn get_api_key(&mut self, id: String) -> String {
        let cluster: Cluster = self.clusters_storage.get(&id).unwrap();
        return cluster.api_key;
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: "alice_near".to_string(),
//             signer_account_id: "bob_near".to_string(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: "carol_near".to_string(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 0,
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 0,
//         }
//     }

//     #[test]
//     fn set_get_message() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = StatusMessage::default();
//         contract.set_status("hello".to_string());
//         assert_eq!(
//             "hello".to_string(),
//             contract.get_status("bob_near".to_string()).unwrap()
//         );
//     }

//     #[test]
//     fn get_nonexistent_message() {
//         let context = get_context(vec![], true);
//         testing_env!(context);
//         let contract = StatusMessage::default();
//         assert_eq!(None, contract.get_status("francis.near".to_string()));
//     }
// }
