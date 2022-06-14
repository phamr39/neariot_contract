yarn build && near deploy --wasmFile res/neariot_contract.wasm --accountId neariot.testnet

near call ciuz.testnet new "{}" --accountId neariot.testnet

near call neariot.testnet new_cluster '{"name":"aloha","description":"Hello World"}' --accountId neariot.testnet

near call neariot.testnet get_clusters --accountId neariot.testnet

near call neariot.testnet get_cluster '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTUyMTg5NzI1OTM1NDUyODU="}' --accountId neariot.testnet

near call neariot.testnet get_cluster_data '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTUyMTg5NzI1OTM1NDUyODU="}' --accountId neariot.testnet

near call neariot.testnet set_apikey_hash '{"id":"bmVhcmlvdC50ZXN0bmV0XzE2NTUyMTg5NzI1OTM1NDUyODU=", "apikey_hash":"Abchasdasjkjl"}' --accountId neariot.testnet
