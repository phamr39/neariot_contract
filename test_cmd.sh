yarn build && near deploy --wasmFile res/neariot_contract.wasm --accountId ciuz.testnet

near call ciuz.testnet new "{}" --accountId ciuz.testnet

near call ciuz.testnet new_cluster '{"name":"aloha","description":"Hello World"}' --accountId ciuz.testnet

near call ciuz.testnet get_clusters --accountId ciuz.testnet

near call ciuz.testnet get_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg="}' --accountId ciuz.testnet

near call ciuz.testnet get_cluster_data '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg="}' --accountId ciuz.testnet

near call ciuz.testnet set_apikey_hash '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg=", "apikey_hash":"Abchasdasjkjl"}' --accountId ciuz.testnet

near call ciuz.testnet set_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg=","name":"abc","description":"abc"}' --accountId ciuz.testnet

near call ciuz.testnet remove_cluster '{"id":"Y2l1ei50ZXN0bmV0XzE2NTczNzYxNzc0Mzc2NTg5MTg="}' --accountId ciuz.testnet