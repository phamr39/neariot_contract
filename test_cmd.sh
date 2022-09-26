yarn build && near deploy --wasmFile res/neariot_contract.wasm --accountId neariot_contract.testnet



near call ciuz.testnet new "{}" --accountId ciuz.testnet

near call ciuz.testnet new_cluster '{"name":"aloha","description":"Hello World"}' --accountId ciuz.testnet

near call neariot.testnet get_clusters --accountId neariot.testnet

near call neariot.testnet get_cluster '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTg1NzE0NDUyODc1MTc4Mzc="}' --accountId neariot.testnet

near call neariot.testnet get_cluster_data '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTg1NzE0NDUyODc1MTc4Mzc="}' --accountId neariot.testnet

near call ciuz.testnet set_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg=","name":"abc","description":"abc"}' --accountId ciuz.testnet

near call ciuz.testnet remove_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg="}' --accountId ciuz.testnet


neariot_test2.testnet
hieutest1.testnet


near call neariot_contract.testnet new "{}" --accountId neariot_test2.testnet
near call neariot_contract.testnet join "{}" --accountId hieutest1.testnet
near call neariot_contract.testnet get_user '{"user_id": "hieutest1.testnet"}' --accountId hieutest1.testnet 
near call neariot_contract.testnet create_project '{"metadata": "HAHJDGSJAgjsafghjasgdjasgdjghajkdjksahdjaskdj"}' --accountId hieutest1.testnet 
near call neariot_contract.testnet get_user_projects_created '{"id": "hieutest1.testnet"}' --accountId neariot_contract.testnet 
near call neariot_contract.testnet get_project '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MjA4MDY1Njc2NDM2MjE5"}' --accountId neariot_test2.testnet 
near call neariot_contract.testnet add_project_offer '{"id":"aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MjA4MDY1Njc2NDM2MjE5", "price":1, "expires_at":1664014557681,"metadata":"1"}' --accountId hieutest1.testnet 
near call neariot_contract.testnet remove_project_offer '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MjA4MDY1Njc2NDM2MjE5", "offer_id": "OF_1664017845791869975"}' --accountId hieutest1.testnet 
near call neariot_contract.testnet update_project '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MjA4MDY1Njc2NDM2MjE5", "metadata": "Mot con vit xoe ra hai cai canh"}' --accountId hieutest1.testnet 

neariot_contract.testnet
near call neariot_contract.testnet new "{}" --accountId neariot_contract.testnet

near call neariot_contract.testnet get_projects_funded "{}" --accountId hieutest1.testnet
near call neariot_contract.testnet get_projects_watched "{}" --accountId neariot_test2.testnet
near call neariot_contract.testnet add_to_watchlist '{"id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MjA4MDY1Njc2NDM2MjE5"}' --accountId neariot_test2.testnet 
near call neariot_contract.testnet get_user '{"user_id": "neariot_test2.testnet"}' --accountId neariot_test2.testnet 
near call neariot_contract.testnet buy_offer '{"project_id": "aGlldXRlc3QxLnRlc3RuZXRfcHJvamVjdF8xNjY0MjA4MDY1Njc2NDM2MjE5", "offer_id": "OF_1664208112775238844"}' --accountId hieutest1.testnet --deposit 1