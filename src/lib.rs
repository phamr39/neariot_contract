use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};
use serde::Serialize;
near_sdk::setup_alloc!();

// ------------------------------
// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
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
        let mut output = account_id.to_owned();
        output.push_str("_");
        output.push_str(&(&env::block_timestamp().to_string()));
        // let decoded = bs58::decode(&output).into_vec()?;
        return output;
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
            // users_storage: UnorderedMap::new(b"uS".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }

    pub fn new_cluster(&mut self, name: String, descriptions: String) -> Cluster {
        let cluster: Cluster = Cluster::new(name, descriptions);
        return match self.clusters_storage.get(&cluster.id) {
            Some(cluster) => cluster,
            None => self.clusters_storage.insert(&cluster.id, &cluster).unwrap(),
        }
    }

    pub fn get_cluster(&mut self, id: String) -> Cluster {
        return self.clusters_storage.get(&id).unwrap();
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
