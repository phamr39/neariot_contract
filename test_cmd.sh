yarn build && near deploy --wasmFile res/neariot_contract.wasm --accountId neariot_test2.testnet


near call ciuz.testnet new "{}" --accountId ciuz.testnet

near call ciuz.testnet new_cluster '{"name":"aloha","description":"Hello World"}' --accountId ciuz.testnet

near call neariot.testnet get_clusters --accountId neariot.testnet

near call neariot.testnet get_cluster '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTg1NzE0NDUyODc1MTc4Mzc="}' --accountId neariot.testnet

near call neariot.testnet get_cluster_data '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTg1NzE0NDUyODc1MTc4Mzc="}' --accountId neariot.testnet

near call ciuz.testnet set_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg=","name":"abc","description":"abc"}' --accountId ciuz.testnet

near call ciuz.testnet remove_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg="}' --accountId ciuz.testnet


neariot_test2.testnet
hieutest1.testnet

near call neariot_test2.testnet new "{}" --accountId neariot_test2.testnet
near call neariot_test2.testnet join "{}" --accountId hieutest1.testnet
near call neariot_test2.testnet get_user '{"user_id": "hieutest1.testnet"}' --accountId hieutest1.testnet 
near call neariot_test2.testnet create_project '{"metadata": "HAHJDGSJAgjsafghjasgdjasgdjghajkdjksahdjaskdj"}' --accountId hieutest1.testnet 
near call neariot_test2.testnet get_user_projects_created '{"id": "hieutest1.testnet"}' --accountId neariot_test2.testnet 
near call neariot_test2.testnet get_project '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MDE0MDU2MjI4ODg1NDgz"}' --accountId neariot_test2.testnet 
near call neariot_test2.testnet add_project_offer '{"id":"aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MDE0MDU2MjI4ODg1NDgz", "price":1, "expires_at":1664014557681,"metadata":"1"}' --accountId hieutest1.testnet 
near call neariot_test2.testnet remove_project_offer '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MDE0MDU2MjI4ODg1NDgz", "offer_id": "OF_1664017845791869975"}' --accountId hieutest1.testnet 
near call neariot_test2.testnet update_project '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MDE0MDU2MjI4ODg1NDgz", "metadata": "Mot con vit xoe ra hai cai canh"}' --accountId hieutest1.testnet 
