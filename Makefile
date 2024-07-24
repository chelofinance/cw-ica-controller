union_version := v0.22.0
union_cmd := docker run --rm --network host -v /tmp:/tmp -v $(PWD)/cosmwasm/build:/build -v $(PWD)/cosmwasm/.keyring:/keyring ghcr.io/unionlabs/uniond-release:${union_version}

rpc_url := https://rpc.testnet.bonlulu.uno:443
chain_id := union-testnet-8
account_name := faucet
contract_label := ica-cont-evm
contract_salt := 1

.PHONY: build deploy

deploy: 
	${union_cmd} keys import-hex ${account_name} ${PRIVATE_KEY} --keyring-backend test --keyring-dir /keyring && \
		${union_cmd} tx wasm store /build/contract.wasm --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \
		jq .txhash | \
		xargs -I{} sh -c "${union_cmd} query event-query-tx-for {} --output json --node ${rpc_url} || ${union_cmd} query tx {} --output json --node ${rpc_url}" | \
		jq -r '.events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value' | \
		xargs -I{} ${union_cmd} tx wasm instantiate2 {} '{ "config": { "protocol_timeout": 3600000000000 }}' ${contract_salt} --label ${contract_label} --no-admin --gas auto --gas-adjustment 1.5 --output json -y --keyring-backend test --keyring-dir /keyring --node ${rpc_url} --chain-id ${chain_id} --from ${account_name} | \
		jq .txhash | \
		xargs -I{} sh -c "${union_cmd} query event-query-tx-for {} --output json --node ${rpc_url} || ${union_cmd} query tx {} --output json --node ${rpc_url}" | \
		jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value' | \
		xargs -I{} echo "Contract address {}" && \
		rm -rf /keyring/*

check-channel:
	${union_cmd} --node ${rpc_url} query ibc channel channels --output json | jq -r '.channels[] | select(.counterparty.port_id == "${CONTRACT_ADDRESS}")'
