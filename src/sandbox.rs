use near_sdk::{Promise};
// use near_sdk::{json_types::U128, near_bindgen, AccountId};
use near_sdk::collections::{ Vector };
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::*;

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
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
            id,
            price,
            created_at: env::block_timestamp(),
            expires_at,
            metadata,
        }
    }
}

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
// #[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
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
            id,
            created_at: env::block_timestamp(),
            metadata,
            buyer,
            rate: 0,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct Project {
    pub owner: AccountId,
    pub id: ProjectId,
    pub avg_rate: u32,
    pub metadata: String,
    // Offer need to conver to Base64 before
    pub offers: Vector<Offer>,
    pub created_at: u64,
    pub pledgers: Vector<AccountId>,
    pub watchers: Vector<AccountId>,
    pub total_pledge: Balance, // Total pledge amount, include all pledge amount of all offers
    pub total_pledge_locked: Balance, // Total pledge amount locked, include all pledge amount locked of all offers
    pub total_offers_bought: u32,     // Total offers bought
    pub total_offers_completed: u32,  // Total offers completed
    pub total_offers_cancled: u32,    // Total offers cancled
    pub bought_offers: Vector<BoughtOffer>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            owner: env::signer_account_id(),
            id: gen_project_id(),
            avg_rate: 0,
            metadata: String::from(""),
            offers: Vector::new(generate_vector_id(String::from("offers"))),
            created_at: env::block_timestamp(),
            pledgers: Vector::new(generate_vector_id(String::from("pledgers"))),
            watchers: Vector::new(generate_vector_id(String::from("watchers"))),
            total_pledge: 0,
            total_pledge_locked: 0,
            total_offers_bought: 0,
            total_offers_completed: 0,
            total_offers_cancled: 0,
            bought_offers: Vector::new(generate_vector_id(String::from("bought_offers"))),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct ProjectUser {
    pub id: AccountId,
    pub total_spent: Balance,
    pub projects_funded: Vector<ProjectId>,
    pub projects_completed: Vector<ProjectId>,
    pub projects_watched: Vector<ProjectId>,
    pub projects_created: ProjectId,
}

impl ProjectUser {
    pub fn new() -> Self {
        Self {
            id: env::signer_account_id(),
            total_spent: 0,
            projects_funded: Vector::new(generate_vector_id(String::from("projects_funded"))),
            projects_completed: Vector::new(generate_vector_id(String::from("projects_completed"))),
            projects_watched: Vector::new(generate_vector_id(String::from("projects_watched"))),
            projects_created: String::from(""),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct Recommendation {
    pub new_project_list: Vector<ProjectId>,
}

impl Recommendation {
    pub fn new() -> Self {
        Self {
            new_project_list: Vector::new(generate_vector_id(String::from("new_project_list"))),
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
        // Add recommendation list
        let tmp_recommendation = self.recommendations.get(&env::current_account_id());
        if !tmp_recommendation.is_some() {
            self.recommendations
                .insert(&env::current_account_id(), &Recommendation::new());
        }
        let mut recommendation = self
            .recommendations
            .get(&env::current_account_id())
            .unwrap();
        // Check if recommendation list is exist
        let mut tmp_list_recommend = recommendation.new_project_list;
        if tmp_list_recommend.len() >= 20 {
            tmp_list_recommend.pop();
        }
        let mut new_list_recommend =  Vector::new(generate_vector_id(String::from("new_list_recommend")));
        new_list_recommend.push(&project.id);
        new_list_recommend.extend(tmp_list_recommend.iter());
        recommendation.new_project_list = new_list_recommend;
        self.recommendations
            .insert(&env::current_account_id(), &recommendation);
        return project;
    }

    // Add Offer to Project
    pub fn add_project_offer(
        &mut self,
        id: ProjectId,
        price: u32,
        expires_at: u64,
        metadata: String,
    ) -> Vector<Offer> {
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
        project_data.offers.push(&new_offer);
        self.projects.insert(&id, &project_data);
        return project_data.offers;
    }

    // Remove Offer from Project
    pub fn remove_project_offer(&mut self, id: ProjectId, offer_id: String) -> Vector<Offer> {
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
        offers.swap_remove(index);
        offers.pop();
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
            assert!(true, "Offer is not exist!");
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
        user_projects_funded.push(&project_id);
        user_data.projects_funded = user_projects_funded;
        // Transfer a part of money to project owner
        let project_owner = project_data.owner.clone();
        let money = offer.price * (100 - INVESTOR_PROTECT_PERCENT) / 100;
        user_data.total_spent += money.clone();
        Promise::new(project_owner).transfer(money);
        // Update Project Info
        let bought_offer = BoughtOffer::new(offer_id, String::from(""), env::signer_account_id());
        project_data.bought_offers.push(&bought_offer);
        project_data.pledgers.push(&env::signer_account_id());
        project_data.total_pledge += offer.price;
        project_data.total_pledge_locked += offer.price * INVESTOR_PROTECT_PERCENT / 100;
        project_data.total_offers_bought += 1;
        project_data.offers = offers;
        self.users.insert(&env::signer_account_id(), &user_data);
        self.projects.insert(&project_id, &project_data);
    }

    // Approve Project, Release all money to project owner
    #[payable]
    pub fn approve_project(&mut self, id: ProjectId, rate: u32, metadata: String) {
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
        bought_offer.rate = rate;
        bought_offer.metadata = metadata;
        project_data.bought_offers.swap_remove(index);
        project_data.bought_offers.pop();
        project_data.bought_offers.push(&bought_offer);
        project_data.total_offers_completed += 1;
        // Calculate rate for project
        let mut total_rate = 0;
        for offer in project_data.bought_offers.iter() {
            total_rate += offer.rate;
        }
        project_data.avg_rate = total_rate / project_data.bought_offers.len() as u32;
        // Get Offer Information
        let offers = &project_data.offers;
        let mut index = 0;
        for offer in offers.iter() {
            if offer.id == bought_offer.id {
                break;
            }
            index += 1;
        }
        if index >= offers.len() {
            // Offer is exist in Bought offer but not exist in Offer
            assert!(true, "SB0001, Something went wrong!");
        }
        // Calculate money for project owner
        let offer = offers.get(index).unwrap();
        let money = offer.price * (INVESTOR_PROTECT_PERCENT - OFFER_FEES_PERCENT) / 100;
        project_data.total_pledge_locked -= money.clone();
        user_data.total_spent += money.clone();
        // Release remaining money to project
        let project_owner = project_data.owner.clone();
        Promise::new(project_owner).transfer(money);
        // Update Project and User Infor
        self.users.insert(&env::signer_account_id(), &user_data);
        self.projects.insert(&id, &project_data);
    }

    // Reject Project, Cashback remain money to pledger
    #[payable]
    pub fn reject_project(&mut self, id: ProjectId, rate: u32, metadata: String) {
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
            assert!(true, "User is not exist in pledged list!");
        }
        // Update Bought offer and project Rate
        let mut bought_offer = project_data.bought_offers.get(index).unwrap().clone();
        // Get Offer Information, check if bought offer is not exists in offer list
        let offers = &project_data.offers;
        let mut index = 0;
        for offer in offers.iter() {
            if offer.id == bought_offer.id {
                break;
            }
            index += 1;
        }
        if index >= offers.len() {
            // Offer is exist in Bought offer but not exist in Offer
            assert!(true, "SB00005, Something went wrong!");
        }
        bought_offer.rate = rate;
        bought_offer.metadata = metadata;
        project_data.bought_offers.swap_remove(index);
        project_data.bought_offers.pop();
        project_data.bought_offers.push(&bought_offer);
        // Calculate rate for project
        let mut total_rate = 0;
        for offer in project_data.bought_offers.iter() {
            total_rate += offer.rate;
        }
        project_data.avg_rate = total_rate / project_data.bought_offers.len() as u32;
        project_data.total_offers_cancled += 1;
        // Calculate money for project owner
        let offer = offers.get(index).unwrap();
        let money = offer.price * (INVESTOR_PROTECT_PERCENT - 1) / 100;
        project_data.total_pledge_locked -= money.clone();
        user_data.projects_completed.push(&id);

        // Cashback remaining money to pledger
        Promise::new(env::signer_account_id()).transfer(money);
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
        user_data.projects_watched.push(&id);
        self.users.insert(&env::signer_account_id(), &user_data);
        // Add watcher list to project
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        project_data.watchers.push(&env::signer_account_id());
        self.projects.insert(&id, &project_data);
    }

    // Get all bought offers of a project
    pub fn get_bought_offers(&self, id: ProjectId) -> Vector<BoughtOffer> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        return project_data.bought_offers;
    }

    // Get list of projects that user funded
    pub fn get_projects_funded(&self) -> Vector<Project> {
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User did not pledge any project!");
        let user_data = user.unwrap();
        let mut projects = Vector::new(generate_vector_id(String::from("projects")));
        for id in user_data.projects_funded.iter() {
            let project = self.projects.get(&id);
            if project.is_some() {
                projects.push(&project.unwrap());
            }
        }
        return projects;
    }

    // Get list of projects that user watched
    pub fn get_projects_watched(&self) -> Vector<Project> {
        let user = self.users.get(&env::signer_account_id());
        assert!(user.is_some(), "User is not watching any project!");
        let user_data = user.unwrap();
        let mut projects = Vector::new(generate_vector_id(String::from("projects")));
        for id in user_data.projects_watched.iter() {
            let project = self.projects.get(&id);
            if project.is_some() {
                projects.push(&project.unwrap());
            }
        }
        return projects;
    }

    // Get list of pledgers of a project

    pub fn get_pledgers(&self, id: ProjectId) -> Vector<ProjectUser> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        let mut pledgers = Vector::new(generate_vector_id(String::from("pledgers")));
        for id in project_data.pledgers.iter() {
            let pledger = self.users.get(&id);
            if pledger.is_some() {
                pledgers.push(&pledger.unwrap());
            }
        }
        return pledgers;
    }

    // Get list of watcher of a project
    pub fn get_watchers(&self, id: ProjectId) -> Vector<ProjectUser> {
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let project_data = project.unwrap();
        let mut watchers = Vector::new(generate_vector_id(String::from("watchers")));
        for id in project_data.watchers.iter() {
            let watcher = self.users.get(&id);
            if watcher.is_some() {
                watchers.push(&watcher.unwrap());
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
            if data == id {
                break;
            }
            index += 1;
        }
        if index >= user_data.projects_watched.len() {
            // Project is not exist in watchlist
            assert!(true, "Project is not exist in watchlist!");
        }
        user_data.projects_watched.swap_remove(index);
        user_data.projects_watched.pop();
        self.users.insert(&env::signer_account_id(), &user_data);
        // Remove watcher list from project
        let project = self.projects.get(&id);
        assert!(project.is_some(), "Project is not exist!");
        let mut project_data = project.unwrap();
        let mut index = 0;
        for data in project_data.watchers.iter() {
            if data == env::signer_account_id() {
                break;
            }
            index += 1;
        }
        if index >= project_data.watchers.len() {
            // Project is not exist in watchlist
            assert!(true, "Project is not exist in watchlist!");
        }
        project_data.watchers.swap_remove(index);
        project_data.watchers.pop();
        self.projects.insert(&id, &project_data);
    }

    // Get list of recommended projects
    pub fn get_recommended_projects(&self) -> Vec<Project> {
        let mut projects = vec![];
        // Get recommend project list
        let recommend_projects = self.recommendations.get(&env::signer_account_id()).unwrap();
        for id in recommend_projects.new_project_list.iter() {
            let project = self.projects.get(&id);
            if project.is_some() {
                projects.push(project.unwrap());
            }
        }
        return projects;
    }
}
