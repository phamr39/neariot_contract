pub use crate::constants::*;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum SandboxProjectType {
    Public,
    Private
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    pub id: String,
    pub created_at: u64,
    pub description: String,
    pub project_type: SandboxProjectType,
    pub subcribe_fee: Balance,
    pub subcribers: Vec<AccountId>,
    pub repository: String,
    pub meta_data: String,
}

impl Project {
    pub fn new(description: String, project_type: SandboxProjectType, subcribe_fee: Balance, repo: String ) -> Self {
        Self {
            id: env::signer_account_id(),
            created_at: env::block_timestamp(),
            description: description,
            project_type: project_type,
            subcribe_fee: subcribe_fee,
            subcribers: Vec::new(),
            repository: repo,
            meta_data: String::from(""),
        }
    }
}

#[near_bindgen]
impl Contract {
}
