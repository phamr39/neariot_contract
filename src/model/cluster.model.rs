#[path = "../storage/cluster.storage.rs"]
mod ClusterStorage;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
pub struct ClusterModel {
    storage: ClusterStorage,
    owner: String,
    id: String,
    api_key: String,
    data: String,
}

impl Default for ClusterModel {

    fn default() -> Self {
        let account_id: String = env::signer_account_id();
        let id: String = generateId(&account_id);
        let api_key: String = generateApiKey(&id);
        let data: String = '';
        Self {
            owner: &account_id,
            id: &id,
            api_key: &api_key,
        }
    }

    fn generateId(account_id: String) -> String {
        let mut output = account_id.to_owned();
        output.push_str("_")
        output.push_str(&env::block_timestamp())
        return output;
    }

    fn generateApiKey(id: String) -> String {
        return &id;
    }

}

#[near_bindgen]
impl ClusterModel {
    pub fn set_data(&mut self, data: String) {
        self.data = &data;
        
    }

    pub fn get_data(&mut self) {
        // return 
    }

    pub fn get_apiKey() {

    }

    pub fn save() {
        let mut storage = ClusterStorage::default();
    }
}
