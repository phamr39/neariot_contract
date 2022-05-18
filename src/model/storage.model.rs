use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserStorageModel {
    records: UnorderedMap<String, String>,
    owner: '',
    id: '',
    apiKey: '',
}

impl Default for UserStorageModel {

    fn default() -> Self {
        let account_id: String = env::signer_account_id();
        let id: String = generateId(&account_id);
        let apiKey: String = generateApiKey(&id);
        Self {
            records: UnorderedMap::new(b"StgM".to_vec()),
            owner: &account_id,
            id: &id,
            apiKey: &apiKey,
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
impl UserStorageModel {
    pub fn set_data(&mut self, data: String) {
        // let account_id = env::signer_account_id();
        self.records.insert(&self.id, &data);
    }

    pub fn get_data(&mut self) {
        // return 
    }

    pub fn get_apiKey() {

    }
}
