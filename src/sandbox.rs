use near_sdk::Promise;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Offer {
    pub id: String,
    pub price: Balance,
    pub created_at: u64,
    pub expires_at: u64,
    pub metadata: String,
}

impl Offer {
    pub fn new(id: String, price: Balance, expires_at: u64, metadata: String) -> Self {
        Self {
            id: id,
            price: price,
            created_at: env::block_timestamp(),
            expires_at: expires_at,
            metadata: metadata,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    pub owner: AccountId,
    pub id: ProjectId,
    pub avg_rate: u32,
    pub metadata: String,
    // Offer need to conver to Base64 before
    pub offers: Vec<Offer>,
    pub created_at: u64,
    pub pledgers: Vec<AccountId>,
    pub watchers: Vec<AccountId>,
    pub total_pledge: Balance, // Total pledge amount, include all pledge amount of all offers
    pub total_pledge_locked: Balance, // Total pledge amount locked, include all pledge amount locked of all offers
    pub total_offers_bought: u32,     // Total offers bought
    pub total_offers_completed: u32,  // Total offers completed
    pub total_offers_cancled: u32,    // Total offers cancled
}

impl Project {
    pub fn new() -> Self {
        Self {
            owner: env::signer_account_id(),
            id: gen_project_id(),
            avg_rate: 0,
            metadata: String::from(""),
            offers: vec![],
            created_at: env::block_timestamp(),
            pledgers: vec![],
            watchers: vec![],
            total_pledge: 0,
            total_pledge_locked: 0,
            total_offers_bought: 0,
            total_offers_completed: 0,
            total_offers_cancled: 0,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectUser {
    pub id: AccountId,
    pub total_spent: Balance,
    pub projects_funded: Vec<ProjectId>,
    pub projects_completed: Vec<ProjectId>,
    pub projects_watched: Vec<ProjectId>,
    pub projects_created: ProjectId,
}

impl ProjectUser {
    pub fn new() -> Self {
        Self {
            id: env::signer_account_id(),
            total_spent: 0,
            projects_funded: vec![],
            projects_completed: vec![],
            projects_watched: vec![],
            projects_created: String::from(""),
        }
    }
}

#[near_bindgen]
impl Contract {
    // Get User Information by AccountId
    pub fn get_user(&mut self, user_id: AccountId) -> ProjectUser {
        let user = self.users.get(&user_id).expect("User not found!");
        return user;
    }
    // Get Project Information by ProjectId
    pub fn get_project(&mut self, id: ProjectId) -> Project {
        let project = self.projects.get(&id);

        assert!(project.is_some(), "Project is not exist!");

        return project.unwrap();
    }
    // Get Project Information by Creator AccountId
    pub fn get_user_projects_created(&mut self, id: AccountId) -> Project {
        let user = self.users.get(&id);

        assert!(user.is_some(), "User is not exist!");

        let user = user.unwrap();

        let project_id = user.projects_created;

        assert!(
            project_id == String::from(""),
            "User has not created any project!"
        );

        let project = self.projects.get(&project_id);

        assert!(
            project.is_some(),
            "Something went wrong, project is not exist!"
        );

        return project.unwrap();
    }

    // Create Project. Each user can only create one project
    pub fn create_project(&mut self, metadata: String) -> Project {
        let mut user = self.users.get(&env::signer_account_id());
        if !user.is_some() {
            self.users
                .insert(&env::signer_account_id(), &ProjectUser::new());
            user = self.users.get(&env::signer_account_id());
        }
        let mut user_data = user.unwrap();
        assert!(
            user_data.projects_created != String::from(""),
            "User has already created a project!"
        );
        let mut project = Project::new();
        user_data.projects_created = project.id.clone();
        project.metadata = metadata;
        project.owner = env::signer_account_id();
        self.projects.insert(&project.id, &project);
        self.users.insert(&env::signer_account_id(), &user_data);
        return project;
    }

    // Add Offer to Project
    pub fn add_project_offer(
        &mut self,
        id: ProjectId,
        price: Balance,
        expires_at: u64,
        metadata: String,
    ) -> Vec<Offer> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner != env::signer_account_id(),
            "You are not the owner of this project!"
        );
        let mut offer_id = String::from("OF_");
        offer_id.push_str(&env::block_timestamp().to_string());
        let new_offer = Offer::new(offer_id, price, expires_at, metadata);
        project_data.offers.push(new_offer);
        self.projects.insert(&id, &project_data);
        return project_data.offers;
    }

    // Remove Offer from Project
    pub fn remove_project_offer(&mut self, id: ProjectId, offer_id: String) -> Vec<Offer> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner != env::signer_account_id(),
            "You are not the owner of this project!"
        );
        let mut offers = project_data.offers;
        let mut index = 0;
        for offer in offers.iter() {
            if offer.id == offer_id {
                break;
            }
            index += 1;
        }
        if index >= offers.len() {
            assert!(true, "Offer is not exist!");
        }
        offers.remove(index);
        project_data.offers = offers;
        self.projects.insert(&id, &project_data);
        return project_data.offers;
    }

    // Buy Offer
    #[payable]
    pub fn buy_offer(&mut self, project_id: ProjectId, offer_id: String) {
        // Get Project Information
        let project = self.projects.get(&project_id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        // Get Offer Information
        let offers = project_data.offers;
        let mut index = 0;
        for offer in offers.iter() {
            if offer.id == offer_id {
                break;
            }
            index += 1;
        }
        if index >= offers.len() {
            assert!(true, "Offer is not exist!");
        }
        // Validate Offer and User role
        let offer = offers.get(index).unwrap();
        assert!(
            offer.expires_at < env::block_timestamp(),
            "Offer is expired!"
        );
        assert!(
            &env::signer_account_id() == &project_data.owner,
            "You are the owner of this project!"
        );
        // Check if user's deposite is enough
        if env::attached_deposit() < offer.price {
            assert!(true, "Not enough money!");
        }
        // Update User Info
        let mut user = self.users.get(&env::signer_account_id());
        if !user.is_some() {
            self.users
                .insert(&env::signer_account_id(), &ProjectUser::new());
            user = self.users.get(&env::signer_account_id());
        }
        let mut user_data = user.unwrap();
        let mut user_projects_funded = user_data.projects_funded;
        user_projects_funded.push(project_id.clone());
        user_data.projects_funded = user_projects_funded;
        self.users.insert(&env::signer_account_id(), &user_data);
        // Transfer a part of money to project owner
        let project_owner = project_data.owner.clone();
        Promise::new(project_owner).transfer(offer.price * (100 - INVESTOR_PROTECT_PERCENT) / 100);
        // Update Project Info
        project_data.total_pledge += offer.price;
        project_data.total_pledge_locked += offer.price * INVESTOR_PROTECT_PERCENT / 100;
        project_data.total_offers_bought += 1;
        project_data.offers = offers;
        self.projects.insert(&project_id, &project_data);
    }
}
