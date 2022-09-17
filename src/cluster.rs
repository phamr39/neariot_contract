use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cluster {
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ClusterMetaData {
    pub owner: AccountId,
    pub id: String,
    pub name: String,
    pub description: String,
    pub apikey_hash: String,
    pub data: String,
    pub create_at:u64,
    pub update_at:u64,
}

impl ClusterMetaData {
    pub fn new(name: String, description: String) -> Self {
        Self {
            owner: env::signer_account_id(),
            id: gen_cluster_id(),
            name: name,
            description: description,
            apikey_hash: String::from(""),
            data: String::from(""),
            create_at:env::block_timestamp(),
            update_at:env::block_timestamp(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_clusters(&mut self) -> Vec<ClusterMetaData> {
        let owner = env::predecessor_account_id();

        let clusters_for_owner_set = self.cluster_per_owner.get(&owner);
        let clusters = if let Some(clusters_for_owner_set) = clusters_for_owner_set {
            clusters_for_owner_set
        } else {
            return vec![];
        };
        return clusters
            .iter()
            .skip(0 as usize)
            .take(50 as usize)
            .map(|cluster_id| self.cluster_metadata.get(&cluster_id).unwrap())
            .collect();
    }

    pub fn get_cluster(&mut self, id: String) -> Cluster {
        let cluster = self.cluster.get(&id);

        assert!(cluster.is_some(), "Cluster is not exist!");

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id().to_string(),
            "This cluster is not belong to you"
        );

        return cluster.unwrap();
    }

    pub fn set_cluster(
        &mut self,
        id: String,
        name: String,
        description: String,
    ) -> ClusterMetaData {
        let cluster = self.cluster.get(&id);

        assert!(cluster.is_some(), "Cluster is not exist!");

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id().to_string(),
            "This cluster is not belong to you"
        );

        let mut metadata = self.cluster_metadata.get(&id).unwrap();
        metadata.name = name;
        metadata.description = description;
        self.cluster_metadata.insert(&id, &metadata);
        return metadata;
    }

    pub fn remove_cluster(&mut self, id: String) -> String {
        let cluster = self.cluster.get(&id);
        let owner = env::signer_account_id();

        assert!(cluster.is_some(), "Cluster is not exist!");

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id().to_string(),
            "This cluster is not belong to you"
        );
        self.cluster.remove(&id);
        self.cluster_metadata.remove(&id);
        let mut clusters_for_owner_set = self.cluster_per_owner.get(&owner).unwrap();
        clusters_for_owner_set.remove(&id);
        self.cluster_per_owner
            .insert(&owner, &clusters_for_owner_set);
        return String::from("Successfull");
    }

    pub fn get_apikey_hash(&mut self, id: String) -> String {
        let cluster = self.cluster.get(&id);

        assert!(cluster.is_some(), "Cluster is not exist!");

        return self.cluster_metadata.get(&id).unwrap().apikey_hash;
    }

    pub fn set_apikey_hash(&mut self, id: String, apikey_hash: String) -> ClusterMetaData {
        let cluster = self.cluster.get(&id);

        assert!(cluster.is_some(), "Cluster is not exist!");

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id().to_string(),
            "This cluster is not belong to you"
        );

        let mut metadata = self.cluster_metadata.get(&id).unwrap();
        metadata.apikey_hash = apikey_hash;
        self.cluster_metadata.insert(&id, &metadata);
        return metadata;
    }

    pub fn get_cluster_data(&mut self, id: String) -> ClusterMetaData {
        let cluster = self.cluster.get(&id);

        assert!(cluster.is_some(), "Cluster is not exist!");

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id().to_string(),
            "This cluster is not belong to you"
        );

        return self.cluster_metadata.get(&id).unwrap();
    }
}
