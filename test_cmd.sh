near deploy --wasmFile res/neariot_contract.wasm --accountId neariot.testnet

near call neariot.testnet new_cluster '{"name": "aloha", "descriptions": "Hello World"}' --accountId neariot.testnet

near call neariot.testnet set_status '{"message": "aloha!"}' --accountId neariot.testnet

near call neariot.testnet get_cluster '{"id": "neariot.testnet_1652952960077587217"}' --accountId neariot.testnet


neariot_test1.testnet
