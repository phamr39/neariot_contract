// #[path = "../model/cluster.rs"]
// mod ClusterModel;

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct ClusterStorage{
//     records: UnorderedMap<String, ClusterModel>,
// }

// impl Default for ClusterStorage {

//     fn default() -> Self {
//         Self {
//             records: UnorderedMap::new(b"cS".to_vec()),
//         }
//     }

// }

// #[near_bindgen]
// impl ClusterStorage {
//     pub fn get(&mut self, id: String) -> (ClusterModel, None) {
//         let mut output = None;
//         match self.records.get(&id) {
//             Some(value) => {
//                 let log_message = format!("Value from UnorderedMap is {:?}", value.clone());
//                 env::log(log_message.as_bytes());
//                 output = value;
//             },
//         }
//         return output;
//     }

//     pub fn set(&mut self, id: String, value: ClusterModel) -> (ClusterModel, None) {
//         return &self.records.insert(&id, &value);
//     }

//     pub fn delete(&mut self, id: String) -> (ClusterModel, None) {
//         return &self.records.remove(&id);
//     }
// }
