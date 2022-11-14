use std::num::Wrapping;

use near_sdk::Promise;
// use near_sdk::{json_types::U128, near_bindgen, AccountId};

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
pub struct BoughtOffer {
    pub id: String,
    pub created_at: u64,
    pub metadata: String,
    pub buyer: AccountId,
    pub rate: u32,
}

impl BoughtOffer {
    pub fn new(id: String, metadata: String, buyer: AccountId) -> Self {
        Self {
            id: id,
            created_at: env::block_timestamp(),
            metadata: metadata,
            buyer: buyer,
            rate: 0,
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
    pub bought_offers: Vec<BoughtOffer>,
    pub milestones: String,
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
            bought_offers: vec![],
            milestones: String::from(""),
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
    pub fn join(&mut self) {
        let user_id = env::signer_account_id();
        let existed_user = self.users.get(&user_id);
        if existed_user.is_none() {
            let user = ProjectUser::new();
            self.users.insert(&user_id, &user);
        }
    }
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

        assert!(project_id.len() > 0, "User has not created any project!");

        let project = self.projects.get(&project_id);

        assert!(
            project.is_some(),
            "SB00006: Something went wrong, project is not exist!"
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
            user_data.projects_created.len() == 0,
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
        price: u32,
        expires_at: u64,
        metadata: String,
    ) -> Vec<Offer> {
        let balance_price = u128::from(price) * NEAR_DECIMAL;
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner == env::signer_account_id(),
            "You are not the owner of this project!"
        );
        let mut offer_id = String::from("OF_");
        offer_id.push_str(&env::block_timestamp().to_string());
        let new_offer = Offer::new(offer_id, balance_price, expires_at, metadata);
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
            project_data.owner == env::signer_account_id(),
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
            assert!(false, "Offer is not exist!");
        }
        offers.remove(index);
        project_data.offers = offers;
        self.projects.insert(&id, &project_data);
        return project_data.offers;
    }

    // Update project metadata
    pub fn update_project(&mut self, id: ProjectId, metadata: String) -> Project {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner == env::signer_account_id(),
            "You are not the owner of this project!"
        );
        project_data.metadata = metadata;
        self.projects.insert(&id, &project_data);
        return project_data;
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
            assert!(false, "Offer is not exist!");
        }
        // Validate Offer and User role
        let offer = offers.get(index).unwrap();
        assert!(
            offer.expires_at < env::block_timestamp(),
            "Offer is expired!"
        );
        assert!(
            &env::signer_account_id() != &project_data.owner,
            "You are the owner of this projecty!"
        );
        // Check if user's deposite is enough
        if env::attached_deposit() < offer.price {
            assert!(false, "Not enough money!");
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
        // Transfer a part of money to project owner
        let project_owner = project_data.owner.clone();
        let money = Wrapping(offer.price * (100 - INVESTOR_PROTECT_PERCENT) / 100);
        user_data.total_spent = (Wrapping(user_data.total_spent) + money.clone()).0;
        Promise::new(project_owner).transfer(money.0);
        // Update Project Info
        let bought_offer = BoughtOffer::new(offer_id, String::from(""), env::signer_account_id());
        project_data.bought_offers.push(bought_offer);
        project_data.pledgers.push(env::signer_account_id());
        project_data.total_pledge += offer.price;
        project_data.total_pledge_locked = (Wrapping(project_data.total_pledge_locked) + Wrapping(offer.price * INVESTOR_PROTECT_PERCENT / 100)).0;
        project_data.total_offers_bought += 1;
        project_data.offers = offers;
        self.users.insert(&env::signer_account_id(), &user_data);
        self.projects.insert(&project_id, &project_data);
    }

    // Approve Project, Release all money to project owner
    #[payable]
    pub fn approve_project(&mut self, id: ProjectId, rate: u32, metadata: String) {
        assert!(
            rate >= 1,
            "Minimum rate is 1, please check your rate again!"
        );
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner != env::signer_account_id(),
            "You are the owner of this project!"
        );
        // Get offer Information
        // Check if project sender is pledged to this project
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User is not exist!");
        let mut user_data = user.unwrap();
        let mut index = 0;
        for data in project_data.bought_offers.iter() {
            if data.buyer == user_data.id {
                break;
            }
            index += 1;
        }
        // Update Bought offer and project Rate
        let mut bought_offer = project_data.bought_offers.get(index).unwrap().clone();
        assert!(
            bought_offer.rate == 0,
            "This offer has been completed!"
        );
        bought_offer.rate = rate;
        bought_offer.metadata = metadata;
        project_data.bought_offers[index] = bought_offer.clone();
        project_data.total_offers_completed += 1;
        // Calculate rate for project
        let mut total_rate = 0;
        for offer in project_data.bought_offers.iter() {
            total_rate += offer.rate;
        }
        project_data.avg_rate = total_rate / project_data.bought_offers.len() as u32;
        // Get Offer Information
        let offers = project_data.offers.clone();
        let mut index = 0;
        for offer in offers.iter() {
            if offer.id == bought_offer.id {
                break;
            }
            index += 1;
        }
        if index >= offers.len() {
            // Offer is exist in Bought offer but not exist in Offer
            assert!(false, "SB0001, Something went wrong!");
        }
        // Calculate money for project owner
        let offer = offers.get(index).unwrap();
        let money = Wrapping(offer.price * (INVESTOR_PROTECT_PERCENT - OFFER_FEES_PERCENT) / 100);
        project_data.total_pledge_locked = (Wrapping(project_data.total_pledge_locked) - money).0;
        user_data.total_spent += money.clone().0;
        // Release remaining money to project
        let project_owner = project_data.owner.clone();
        Promise::new(project_owner).transfer(money.0);
        // Update Project and User Infor
        self.users.insert(&env::signer_account_id(), &user_data);
        self.projects.insert(&id, &project_data);
    }

    // Reject Project, Cashback remain money to pledger
    #[payable]
    pub fn reject_project(&mut self, id: ProjectId, rate: u32, metadata: String) {
        assert!(
            rate >= 1,
            "Minimum rate is 1, please check your rate again!"
        );
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner != env::signer_account_id(),
            "You are the owner of this project!"
        );
        // Get offer Information
        // Check if project sender pledged to this project
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User is not exist!");
        let mut user_data = user.unwrap();
        let mut index = 0;
        for data in project_data.bought_offers.iter() {
            if data.buyer == user_data.id {
                break;
            }
            index += 1;
        }
        if index >= project_data.bought_offers.len() {
            // User ID is not exist in pledged list
            assert!(false, "User is not exist in pledged list!");
        }
        // Update Bought offer and project Rate
        let mut bought_offer = project_data.bought_offers.get(index).unwrap().clone();
        assert!(
            bought_offer.rate == 0,
            "This offer has been completed!"
        );
        // Get Offer Information, check if bought offer is not exists in offer list
        let offers = project_data.offers.clone();
        let mut index = 0;
        for offer in offers.iter() {
            if offer.id == bought_offer.id {
                break;
            }
            index += 1;
        }
        if index >= offers.len() {
            // Offer is exist in Bought offer but not exist in Offer
            assert!(false, "SB00005, Something went wrong!");
        }
        bought_offer.rate = rate;
        bought_offer.metadata = metadata;
        project_data.bought_offers[index] = bought_offer.clone();
        // Calculate rate for project
        let mut total_rate = 0;
        for offer in project_data.bought_offers.iter() {
            total_rate += offer.rate;
        }
        project_data.avg_rate = total_rate / project_data.bought_offers.len() as u32;
        project_data.total_offers_cancled += 1;
        // Calculate money for project owner
        let offer = offers.get(index).unwrap();
        // let money = (offer.price * (INVESTOR_PROTECT_PERCENT - 1)) / 100;
        // project_data.total_pledge_locked -= money.clone();
        let money = Wrapping(offer.price * (INVESTOR_PROTECT_PERCENT - 1) / 100);
        project_data.total_pledge_locked = (Wrapping(project_data.total_pledge_locked) - money).0;
        user_data.projects_completed.push(id.clone());
        // Cashback remaining money to pledger
        Promise::new(env::signer_account_id()).transfer(money.0);
        // Update Project Infor
        self.projects.insert(&id, &project_data);
        self.users.insert(&env::signer_account_id(), &user_data);
        // Update user information
    }

    // Add Project to watchlist
    pub fn add_to_watchlist(&mut self, id: ProjectId) {
        let mut user = self.users.get(&env::signer_account_id());
        // assert!(user.is_some(), "User is not exist!");
        if !user.is_some() {
            // Create new user
            let new_user = ProjectUser::new();
            self.users.insert(&env::signer_account_id(), &new_user);
            user = self.users.get(&env::signer_account_id());
        }
        let mut user_data = user.unwrap();
        for project in user_data.projects_watched.iter() {
            if project == &id {
                assert!(false, "Project is already in watchlist!");
            }
        }
        user_data.projects_watched.push(id.clone());
        // Add watcher list to project
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        project_data.watchers.push(env::signer_account_id());
        self.users.insert(&env::signer_account_id(), &user_data);
        self.projects.insert(&id, &project_data);
    }

    // Get all bought offers of a project
    pub fn get_bought_offers(&self, id: ProjectId) -> Vec<BoughtOffer> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        return project_data.bought_offers;
    }

    // Get list of projects that user funded
    pub fn get_projects_funded(&self) -> Vec<Project> {
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User did not pledge any project!");
        let user_data = user.unwrap();
        let mut projects = vec![];
        for id in user_data.projects_funded.iter() {
            let project = self.projects.get(id);
            if project.is_some() {
                projects.push(project.unwrap());
            }
        }
        return projects;
    }

    // Get list of projects that user watched
    pub fn get_projects_watched(&self) -> Vec<Project> {
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User is not watching any project!");
        let user_data = user.unwrap();
        let mut projects = vec![];
        for id in user_data.projects_watched.iter() {
            let project = self.projects.get(id);
            if project.is_some() {
                projects.push(project.unwrap());
            }
        }
        return projects;
    }

    // Get list of pledgers of a project

    pub fn get_pledgers(&self, id: ProjectId) -> Vec<ProjectUser> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        let mut pledgers = vec![];
        for id in project_data.pledgers.iter() {
            let pledger = self.users.get(id);
            if pledger.is_some() {
                pledgers.push(pledger.unwrap());
            }
        }
        return pledgers;
    }

    // Get list of watcher of a project
    pub fn get_watchers(&self, id: ProjectId) -> Vec<ProjectUser> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        let mut watchers = vec![];
        for id in project_data.watchers.iter() {
            let watcher = self.users.get(id);
            if watcher.is_some() {
                watchers.push(watcher.unwrap());
            }
        }
        return watchers;
    }

    // Remove Project from watchlist
    pub fn remove_from_watchlist(&mut self, id: ProjectId) {
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User is not exist!");
        let mut user_data = user.unwrap();
        let mut index = 0;
        for data in user_data.projects_watched.iter() {
            if data == &id {
                break;
            }
            index += 1;
        }
        if index >= user_data.projects_watched.len() {
            // Project is not exist in watchlist
            assert!(false, "Project is not exist in watchlist!");
        }
        user_data.projects_watched.remove(index);
        self.users.insert(&env::signer_account_id(), &user_data);
        // Remove watcher list from project
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        let mut index = 0;
        for data in project_data.watchers.iter() {
            if data == &env::signer_account_id() {
                break;
            }
            index += 1;
        }
        if index >= project_data.watchers.len() {
            // Project is not exist in watchlist
            assert!(false, "Project is not exist in watchlist!");
        }
        project_data.watchers.remove(index);
        self.projects.insert(&id, &project_data);
    }

    pub fn get_rcm_projects(&self) -> Vec<Project> {
        let mut projects = vec![];
        let list_projects = self.projects.to_vec();
        for i in 0..(list_projects.len()) {
            if projects.len() > 20 {
                break;
            }
            projects.push(list_projects[list_projects.len() - i - 1].1.clone());
        }
        return projects;
    }

    pub fn get_milestone(&self, id: ProjectId) -> String {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        return project_data.milestones;
    }

    pub fn set_milestone(&mut self, id: ProjectId, milestones: String) {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        assert!(
            project_data.owner == env::signer_account_id(),
            "You are not the owner of this project!"
        );
        project_data.milestones = milestones;
        self.projects.insert(&id, &project_data);
    }
}
